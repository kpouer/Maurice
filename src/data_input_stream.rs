use std::path::Path;
use std::{fs, io};

#[derive(Debug)]
pub(crate) struct DataInputStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl DataInputStream {
    pub(crate) fn new(file: &Path) -> io::Result<DataInputStream> {
        match fs::read(file) {
            Ok(bytes) => Ok(DataInputStream { bytes, pos: 0 }),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn read(&mut self) -> u8 {
        let b = self.bytes[self.pos];
        self.pos += 1;
        b
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }
}
