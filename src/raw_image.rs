use crate::dimension::Dimension;

#[derive(Debug)]
pub struct RawImage {
    pub(crate) data: Vec<u8>,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl RawImage {
    pub(crate) fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![0; width * height * 3],
            width,
            height,
        }
    }

    pub(crate) fn size(&self) -> Dimension {
        Dimension::new(self.width, self.height)
    }
}

impl From<Dimension> for RawImage {
    fn from(size: Dimension) -> Self {
        Self::new(size.width, size.height)
    }
}
