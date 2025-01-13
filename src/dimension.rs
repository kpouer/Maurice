pub struct Dimension {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Dimension {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
