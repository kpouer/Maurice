use crate::hardware::screen::color::COLOR_DEPTH;
use std::fmt::Display;

#[derive(Debug)]
pub struct RawImage<'a> {
    pub(crate) data: &'a Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Display for RawImage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RawImage(width={}, height={})", self.width, self.height)
    }
}

impl<'a> RawImage<'a> {
    pub(crate) fn new_with_data(width: usize, height: usize, data: &'a Vec<u8>) -> Self {
        debug_assert_eq!(data.len(), width * height * COLOR_DEPTH);
        Self {
            data,
            width,
            height,
        }
    }
}
