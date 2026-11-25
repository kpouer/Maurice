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
    pub(crate) const fn len(&self) -> u32 {
        self.len
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) const fn reset(&mut self) {
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
        let name = if let Some(path) = &file.path {
            path.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string()
        } else {
            file.name.clone()
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    fn k7(bytes: Vec<u8>) -> K7 {
        K7 {
            name: "test.k7".to_string(),
            len: bytes.len() as u32,
            bytes: Cursor::new(bytes),
        }
    }

    #[test]
    fn test_len_and_name() {
        let k7 = k7(vec![1, 2, 3]);
        assert_eq!(k7.len(), 3);
        assert_eq!(k7.name(), "test.k7");
    }

    #[test]
    fn test_read_sequential() {
        let mut k7 = k7(vec![0xAA, 0xBB]);
        assert_eq!(k7.read(), Some(0xAA));
        assert_eq!(k7.read(), Some(0xBB));
    }

    #[test]
    fn test_read_past_end_returns_zero() {
        let mut k7 = k7(vec![0xAA]);
        k7.read();
        assert_eq!(k7.read(), Some(0));
    }

    #[test]
    fn test_reset_rewinds() {
        let mut k7 = k7(vec![1, 2, 3]);
        k7.read();
        k7.read();
        k7.reset();
        assert_eq!(k7.read(), Some(1));
    }

    #[test]
    fn test_try_from_missing_file_fails() {
        assert!(K7::try_from("/this/file/does/not/exist.k7".to_string()).is_err());
    }

    #[test]
    fn test_try_from_dropped_file_bytes() {
        let dropped = DroppedFile {
            name: "dropped.k7".to_string(),
            bytes: Some(vec![9, 8, 7].into()),
            ..Default::default()
        };
        let mut k7 = K7::try_from(&dropped).unwrap();
        assert_eq!(k7.name(), "dropped.k7");
        assert_eq!(k7.len(), 3);
        assert_eq!(k7.read(), Some(9));
    }

    #[test]
    fn test_try_from_dropped_file_without_data_fails() {
        let dropped = DroppedFile::default();
        assert!(K7::try_from(&dropped).is_err());
    }
}
