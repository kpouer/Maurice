use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::hardware::M6809::M6809;

const N_BYTES: usize = 1024; // Buffer size

const DESIRED_CHANNELS: u16 = 2;
const DESIRED_SAMPLE_FORMAT: cpal::SampleFormat = cpal::SampleFormat::F32;
const DESIRED_SAMPLE_RATE: cpal::SampleRate = cpal::SampleRate(44100);

pub(crate) struct Sound {
    pub buffer: Arc<Mutex<Vec<u8>>>,
    audio_stream: Option<cpal::Stream>,
    audio: [u8; N_BYTES / 4],
}

impl Sound {
    pub(crate) fn new() -> Self {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let audio_stream = if let Some((stream, _)) = get_audio_stream(buffer.clone()) {
            // let rate = sample_rate.0;
            stream.play().ok();
            Some(stream)
        } else {
            None
        };
        Sound {
            buffer,
            audio_stream,
            audio: [0; N_BYTES / 4],
        }
    }

    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein
    pub(crate) fn play_sound(&mut self, cpu: &M6809) {
        for i in 0..N_BYTES {
            self.audio[i / 4] = cpu.sound_buffer[i];
        }
        let mut buffer = self.buffer.lock().unwrap();
        for i in 0..N_BYTES / 4 {
            buffer.push(self.audio[i]);
        }
    }
}

// Get audio stream and sample rate to use when processing audio. We pass the shared
// buffer which will be used by the APU.
fn get_audio_stream(buffer: Arc<Mutex<Vec<u8>>>) -> Option<(cpal::Stream, cpal::SampleRate)> {
    let device = cpal::default_host().default_output_device()?;
    let supported_configs = device.supported_output_configs().ok()?;
    let mut supported_config = None;
    for c in supported_configs {
        if c.channels() == DESIRED_CHANNELS && c.sample_format() == DESIRED_SAMPLE_FORMAT {
            if c.min_sample_rate() <= DESIRED_SAMPLE_RATE
                && DESIRED_SAMPLE_RATE <= c.max_sample_rate()
            {
                supported_config = Some(c.with_sample_rate(DESIRED_SAMPLE_RATE));
            } else {
                supported_config = Some(c.with_max_sample_rate());
            }
            break;
        }
    }
    let selected_config = supported_config?;
    let sample_rate = selected_config.sample_rate();
    let sample_format = selected_config.sample_format();
    let config: cpal::StreamConfig = selected_config.into();
    let error_function = |err| eprintln!("apu: error playing audio: {}", err);
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data: &mut [f32], _cb: &cpal::OutputCallbackInfo| {
                write_audio_data_to_buffer(&buffer, data)
            },
            error_function,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config,
            move |data: &mut [u16], _cb: &cpal::OutputCallbackInfo| {
                write_audio_data_to_buffer(&buffer, data)
            },
            error_function,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _cb: &cpal::OutputCallbackInfo| {
                write_audio_data_to_buffer(&buffer, data)
            },
            error_function,
            None,
        ),
        _ => panic!("apu: unsupported audio sample format (supported options: F32, U16, I16)"),
    }
    .ok()?;
    Some((stream, sample_rate))
}

// Write audio buffer data to output.
fn write_audio_data_to_buffer<T: cpal::Sample + cpal::FromSample<u8>>(
    buffer: &Arc<Mutex<Vec<u8>>>,
    output: &mut [T],
) {
    let mut buffer = buffer.lock().unwrap();
    let length = std::cmp::min(output.len(), buffer.len());
    for (i, v) in buffer.drain(..length).enumerate() {
        output[i] = T::from_sample(v);
    }
}
