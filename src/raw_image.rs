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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_data_keeps_dimensions() {
        let data = vec![0u8; 2 * 3 * COLOR_DEPTH];
        let img = RawImage::new_with_data(2, 3, &data);
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 3);
        assert_eq!(img.data.len(), data.len());
    }

    #[test]
    fn display_format() {
        let data = vec![0u8; 4 * 5 * COLOR_DEPTH];
        let img = RawImage::new_with_data(4, 5, &data);
        assert_eq!(img.to_string(), "RawImage(width=4, height=5)");
    }

    #[test]
    #[should_panic]
    #[cfg(debug_assertions)]
    fn new_with_data_rejects_wrong_size() {
        let data = vec![0u8; 1];
        RawImage::new_with_data(2, 2, &data);
    }
}
