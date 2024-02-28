use std::time::Duration;
use rodio::Source;

const FRAME_RATE: u32 = 44100;

pub(crate) struct PcmSource {
    data: Vec<f32>,
    position: usize,
}

impl PcmSource {
    pub(crate) fn new(data: Vec<u8>) -> PcmSource {
        let data = data.into_iter().map(|bit| {
            bit as f32 / u8::MAX as f32
        }).collect();
        PcmSource {
            data,
            position: 0,
        }
    }
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
        Some(self.data.len() - self.position)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        FRAME_RATE
    }

    fn total_duration(&self) -> Option<Duration> {
        let ms: u64 = ((1000f32 * self.data.len() as f32) / FRAME_RATE as f32) as u64;
        Some(Duration::from_millis(ms))
    }
}