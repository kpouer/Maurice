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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_empty() {
        let buffer = SoundBuffer::default();
        assert_eq!(buffer.pos, 0);
        assert!((0..SOUND_SIZE).all(|i| buffer[i] == 0));
    }

    #[test]
    fn test_push_stores_values_in_order() {
        let mut buffer = SoundBuffer::default();
        assert!(!buffer.push(10));
        assert!(!buffer.push(20));
        assert_eq!(buffer[0], 10);
        assert_eq!(buffer[1], 20);
        assert_eq!(buffer.pos, 2);
    }

    #[test]
    fn test_push_returns_true_only_when_full() {
        let mut buffer = SoundBuffer::default();
        for i in 0..SOUND_SIZE - 1 {
            assert!(!buffer.push(i as u8), "push {i} must not report full");
        }
        assert!(buffer.push(0xFF));
        assert_eq!(buffer.pos, 0);
        assert_eq!(buffer[SOUND_SIZE - 1], 0xFF);
    }

    #[test]
    fn test_push_wraps_around() {
        let mut buffer = SoundBuffer::default();
        for _ in 0..SOUND_SIZE {
            buffer.push(1);
        }
        assert!(!buffer.push(2));
        assert_eq!(buffer[0], 2);
        assert_eq!(buffer.pos, 1);
    }
}
