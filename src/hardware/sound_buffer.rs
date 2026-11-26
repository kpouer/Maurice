use std::ops::Index;

const SOUND_SIZE: usize = 1024;

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
    fn default_is_zeroed() {
        let buf = SoundBuffer::default();
        assert!((0..SOUND_SIZE).all(|i| buf[i] == 0));
    }

    #[test]
    fn push_stores_values_in_order() {
        let mut buf = SoundBuffer::default();
        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!([buf[0], buf[1], buf[2], buf[3]], [1, 2, 3, 0]);
    }

    #[test]
    fn push_returns_true_only_when_full() {
        let mut buf = SoundBuffer::default();
        for _ in 0..SOUND_SIZE - 1 {
            assert!(!buf.push(7));
        }
        assert!(buf.push(7));
        // wraps around: next push is stored at index 0 again
        assert!(!buf.push(9));
        assert_eq!(buf[0], 9);
    }
}
