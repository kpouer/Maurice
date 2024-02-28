use rodio::{OutputStream, OutputStreamHandle, Source};

use crate::hardware::M6809::M6809;
use crate::hardware::sound::pcm_source::PcmSource;

mod pcm_source;

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
        let mut data = vec![0u8; N_BYTES];
        for i in 0..N_BYTES {
            let value = cpu.sound_buffer[i];
            if value != 0 {
                has_data = true;
                data[i / 4] = value;
            }
        }
        let pcm = PcmSource::new(data);
        self.line.play_raw(pcm).ok();
    }
}