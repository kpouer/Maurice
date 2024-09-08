use speedy2d::dimen::UVec2;

#[derive(Debug)]
pub(crate) struct RawImage {
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

    pub(crate) fn size(&self) -> UVec2 {
        UVec2::new(self.width as u32, self.height as u32)
    }
}

impl From<UVec2> for RawImage {
    fn from(size: UVec2) -> Self {
        Self::new(size.x as usize, size.y as usize)
    }
}
