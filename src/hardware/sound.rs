use std::time::Duration;

use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use rodio::source::SineWave;

use crate::hardware::M6809::M6809;
use crate::int;

const FRAME_RATE: u32 = 44100;
// 44100 Hz
const DURATION: Duration = Duration::from_millis(20);
const SAMPLE_BYTES: int = 16 / 8; // 8 bits

const N_FRAMES: int = 882;
const N_BYTES: usize = 1024; // Buffer size

pub(crate) struct Sound {
    stream: OutputStream,
    line: OutputStreamHandle,
}

impl Sound {
    pub(crate) fn new() -> Self {
        let (stream, line) = OutputStream::try_default().unwrap();
        Sound {
            stream,
            line,
        }
    }

    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein
    pub(crate) fn play_sound(&mut self, cpu: &M6809) {
        let mut has_data = false;
        let mut data = vec![0f32; N_BYTES];
        for i in 0..N_BYTES {
            let value = cpu.sound_buffer[i];
            if value != 0 {
                has_data = true;
                data[i / 4] = value as f32 /  u8::MAX as f32;
            }
        }
        if has_data {
            let pcm = PcmSource {
                data,
                position: 0,
            };

            // let sine = SineWave::new(440.0);
            // self.line.play_raw(sine).ok();
            let samples_converter = pcm.convert_samples();
            self.line.play_raw(samples_converter).ok();
        }
    }
}

struct PcmSource {
    data: Vec<f32>,
    position: usize,
}

impl Iterator for PcmSource {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.data.len() {
            let result = self.data[self.position];
            self.position += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl Source for PcmSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        FRAME_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}