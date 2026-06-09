use std::sync::Arc;
use ringbuf::{HeapRb, CachingProd, CachingCons};
use ringbuf::traits::{Producer, Consumer, Split};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::hardware::m6809::M6809;
use crate::hardware::sound_buffer::{SoundBuffer, SOUND_SIZE};

const AUDIO_DOWNSAMPLING_FACTOR: usize = 4;

const DESIRED_CHANNELS: u16 = 2;
const DESIRED_SAMPLE_FORMAT: cpal::SampleFormat = cpal::SampleFormat::F32;
const DESIRED_SAMPLE_RATE: cpal::SampleRate = 44100;

pub struct Sound {
    pub producer: CachingProd<Arc<HeapRb<u8>>>,
    audio_stream: Option<cpal::Stream>,
    audio: [u8; SOUND_SIZE / AUDIO_DOWNSAMPLING_FACTOR],
}

impl Default for Sound {
    fn default() -> Self {
        let rb = HeapRb::<u8>::new(SOUND_SIZE * 4);
        let (producer, consumer) = rb.split();
        let audio_stream = if let Some((stream, _)) = get_audio_stream(consumer) {
            stream.play().ok();
            Some(stream)
        } else {
            None
        };
        Sound {
            producer,
            audio_stream,
            audio: [0; SOUND_SIZE / 4],
        }
    }
}

impl Sound {
    // Copie du buffer de son provenant du 6809 vers le buffer de la carte son
    // Cette fonction est lancée lorsque le buffer 6809 est plein
    pub fn play_sound(&mut self, cpu: &M6809) {
        self.copy_downsampled_audio_from(&cpu.sound_buffer);
        self.enqueue_audio_samples();
    }

    fn copy_downsampled_audio_from(&mut self, sound_buffer: &SoundBuffer) {
        for destination_index in 0..self.audio.len() {
            let source_index = destination_index * AUDIO_DOWNSAMPLING_FACTOR;
            self.audio[destination_index] = sound_buffer[source_index];
        }
    }

    fn enqueue_audio_samples(&mut self) {
        self.producer.push_slice(&self.audio);
    }
}

// Get audio stream and sample rate to use when processing audio. We pass the shared
// buffer which will be used by the APU.
fn get_audio_stream(mut consumer: CachingCons<Arc<HeapRb<u8>>>) -> Option<(cpal::Stream, cpal::SampleRate)> {
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
                write_audio_data_to_buffer(&mut consumer, data)
            },
            error_function,
            None,
        ),
        cpal::SampleFormat::U16 => device.build_output_stream(
            &config,
            move |data: &mut [u16], _cb: &cpal::OutputCallbackInfo| {
                write_audio_data_to_buffer(&mut consumer, data)
            },
            error_function,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_output_stream(
            &config,
            move |data: &mut [i16], _cb: &cpal::OutputCallbackInfo| {
                write_audio_data_to_buffer(&mut consumer, data)
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
    consumer: &mut CachingCons<Arc<HeapRb<u8>>>,
    output: &mut [T],
) {
    for out in output.iter_mut() {
        if let Some(sample) = consumer.try_pop() {
            *out = T::from_sample(sample);
        } else {
            // If we're out of samples, fill with silence
            *out = T::from_sample(0u8);
        }
    }
}
