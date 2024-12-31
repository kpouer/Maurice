#[cfg(feature = "speedy2d-display")]
use speedy2d::dimen::UVec2;

pub struct Dimension {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Dimension {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

#[cfg(feature = "speedy2d-display")]
impl From<UVec2> for Dimension {
    fn from(v: UVec2) -> Self {
        Self {
            width: v.x as usize,
            height: v.y as usize,
        }
    }
}

#[cfg(feature = "speedy2d-display")]
impl From<Dimension> for UVec2 {
    fn from(v: Dimension) -> Self {
        UVec2::new(v.width as u32, v.height as u32)
    }
}
