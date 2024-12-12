use speedy2d::dimen::UVec2;

pub struct Dimension {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl From<UVec2> for Dimension {
    fn from(v: UVec2) -> Self {
        Self {
            width: v.x as usize,
            height: v.y as usize,
        }
    }
}
