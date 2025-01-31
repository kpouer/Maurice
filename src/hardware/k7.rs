use egui::DroppedFile;
use log::info;
use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;

#[derive(Debug)]
pub struct K7 {
    name: String,
    len: u32,
    bytes: Cursor<Vec<u8>>,
}

impl K7 {
    pub(crate) fn len(&self) -> u32 {
        self.len
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn reset(&mut self) {
        self.bytes.set_position(0);
    }

    pub(crate) fn read(&mut self) -> Option<u8> {
        let mut b = [0];
        self.bytes.read_exact(&mut b).ok();
        Some(b[0])
    }
}

impl TryFrom<String> for K7 {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = fs::read(&value).map_err(|e| e.to_string())?;
        Ok(K7 {
            name: value,
            len: bytes.len() as u32,
            bytes: Cursor::new(bytes),
        })
    }
}

impl TryFrom<PathBuf> for K7 {
    type Error = String;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let name = value
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        Self::try_from(name)
    }
}

impl TryFrom<&DroppedFile> for K7 {
    type Error = String;

    fn try_from(file: &DroppedFile) -> Result<Self, Self::Error> {
        let name = file.name.clone();
        let bytes;
        if let Some(path) = &file.path {
            info!("Dropped file: {name} reading path");
            bytes = fs::read(path).map_err(|e| e.to_string())?;
        } else if let Some(b) = &file.bytes {
            info!("Dropped file: {name} data length {}b", b.len());
            bytes = b.to_vec();
        } else {
            return Err("No path or bytes".to_string());
        }

        Ok(Self {
            name,
            len: bytes.len() as u32,
            bytes: Cursor::new(bytes),
        })
    }
}
