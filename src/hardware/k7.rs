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
    use std::path::Path;

    fn temp_k7(name: &str, content: &[u8]) -> String {
        let path = std::env::temp_dir().join(format!("maurice_test_{}_{name}", std::process::id()));
        fs::write(&path, content).unwrap();
        path.to_str().unwrap().to_string()
    }

    #[test]
    fn try_from_string_reads_file() {
        let path = temp_k7("read.k7", &[1, 2, 3]);
        let mut k7 = K7::try_from(path.clone()).unwrap();
        assert_eq!(k7.len(), 3);
        assert_eq!(k7.name(), path);
        assert_eq!(k7.read(), Some(1));
        assert_eq!(k7.read(), Some(2));
        assert_eq!(k7.read(), Some(3));
        fs::remove_file(Path::new(&path)).unwrap();
    }

    #[test]
    fn try_from_string_missing_file_is_error() {
        assert!(K7::try_from("/nonexistent/maurice/file.k7".to_string()).is_err());
    }

    #[test]
    fn reset_rewinds_to_start() {
        let path = temp_k7("reset.k7", &[10, 20]);
        let mut k7 = K7::try_from(path.clone()).unwrap();
        assert_eq!(k7.read(), Some(10));
        assert_eq!(k7.read(), Some(20));
        k7.reset();
        assert_eq!(k7.read(), Some(10));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn read_past_end_returns_zero() {
        let path = temp_k7("eof.k7", &[5]);
        let mut k7 = K7::try_from(path.clone()).unwrap();
        k7.read();
        // current behaviour: reading past the end yields Some(0)
        assert_eq!(k7.read(), Some(0));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn dropped_file_with_bytes() {
        let file = DroppedFile {
            name: "game.k7".to_string(),
            bytes: Some(vec![9, 8, 7].into()),
            ..Default::default()
        };
        let mut k7 = K7::try_from(&file).unwrap();
        assert_eq!(k7.name(), "game.k7");
        assert_eq!(k7.len(), 3);
        assert_eq!(k7.read(), Some(9));
    }

    #[test]
    fn dropped_file_with_path() {
        let path = temp_k7("dropped.k7", &[4, 5]);
        let file = DroppedFile {
            path: Some(PathBuf::from(&path)),
            ..Default::default()
        };
        let k7 = K7::try_from(&file).unwrap();
        assert_eq!(k7.len(), 2);
        assert!(k7.name().starts_with("maurice_test_"));
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn dropped_file_without_data_is_error() {
        let file = DroppedFile::default();
        assert!(K7::try_from(&file).is_err());
    }
}
