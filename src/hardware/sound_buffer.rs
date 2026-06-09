use std::ops::Index;

pub(crate) const SOUND_SIZE: usize = 1024;

#[derive(Debug)]
pub(crate) struct SoundBuffer {
    buffer: [u8; SOUND_SIZE],
    pos: usize,
}

impl Default for SoundBuffer {
    fn default() -> Self {
        Self {
            buffer: [0; SOUND_SIZE],
            pos: 0,
        }
    }
}

impl SoundBuffer {
    pub(super) const fn push(&mut self, value: u8) -> bool {
        self.buffer[self.pos] = value;
        self.pos = (self.pos + 1) % SOUND_SIZE;
        self.pos == 0
    }
}

impl Index<usize> for SoundBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}