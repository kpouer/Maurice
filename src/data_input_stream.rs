use std::{fs, io};
use std::path::Path;
use crate::int;

#[derive(Debug)]
pub(crate) struct DataInputStream {
    bytes: Vec<u8>,
    pos: usize,
}

impl DataInputStream {
    pub(crate) fn new(file: &Path) -> io::Result<DataInputStream> {
        match fs::read(file) {
            Ok(bytes) => {
                Ok(DataInputStream {
                    bytes,
                    pos: 0,
                })
            }
            Err(e) => {
                e
            }
        }
    }

    pub(crate) fn read(&mut self) -> int {
        let b = self.bytes[self.pos];
        self.pos += 1;
        b as int
    }


    pub fn len(&self) -> usize {
        self.bytes.len()
    }
}