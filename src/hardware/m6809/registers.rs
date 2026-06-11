use crate::int;

#[derive(Debug, Default)]
pub(super) struct Registers {
    // 8bits registers
    pub(super) A: int,
    pub(super) B: int,

    pub(super) D: int, // D is A+B

    // 16bits registers
    pub(super) X: int,
    pub(super) Y: int,
    pub(super) U: int,
    pub(super) S: int,
}

impl Registers {
    // recalculate A and B or D
    pub(super) const fn CALCD(&mut self) {
        self.D = (self.A << 8) | self.B;
    }

    pub(super) const fn CALCAB(&mut self) {
        self.A = self.D >> 8;
        self.B = self.D & 0xFF;
    }
}