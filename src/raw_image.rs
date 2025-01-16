use crate::dimension::Dimension;
use crate::hardware::screen::color::COLOR_DEPTH;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct RawImage {
    pub(crate) data: Arc<Mutex<Vec<u8>>>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl RawImage {
    pub(crate) fn new_with_data(width: usize, height: usize, data: Arc<Mutex<Vec<u8>>>) -> Self {
        {
            let data = data.lock().unwrap();
            assert_eq!(data.len(), width * height * COLOR_DEPTH);
        }
        Self {
            data,
            width,
            height,
        }
    }

    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            data: Arc::new(Mutex::new(vec![0; width * height * COLOR_DEPTH])),
            width,
            height,
        }
    }
}

impl From<Dimension> for RawImage {
    fn from(size: Dimension) -> Self {
        Self::new(size.width, size.height)
    }
}
