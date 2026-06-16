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

#[derive(Debug, Default)]
pub(crate) struct CCRegister {
    pub(super) value: u8,

    // fast CC bits (as ints)
    pub(super) res: int,
    pub(super) m1: int,
    pub(super) m2: int,
    pub(super) sign: int,
    pub(super) ovfl: int,
    pub(super) h1: int,
    pub(super) h2: int,
    pub(super) ccrest: int,
}

impl CCRegister {
    pub(super) fn value(&self) -> u8 {
        self.value
    }

    pub(super) fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    // calculate CC fast bits from CC register
    pub(super) const fn setcc(&mut self, i: int) {
        self.m1 = 0;
        self.m2 = 0;
        self.res = ((i & 1) << 8) | (4 - (i & 4));
        self.ovfl = (i & 2) << 6;
        self.sign = (i & 8) << 4;
        self.h1 = (i & 32) >> 2;
        self.h2 = self.h1;
        self.ccrest = i & 0xd0;
    }

    pub(super) fn res(&self) -> int {
        self.res
    }

    pub(super) fn m1(&self) -> int {
        self.m1
    }

    pub(super) fn m2(&self) -> int {
        self.m2
    }

    pub(super) fn sign(&self) -> int {
        self.sign
    }

    pub(super) fn ovfl(&self) -> int {
        self.ovfl
    }

    pub(super) fn h1(&self) -> int {
        self.h1
    }

    pub(super) fn h2(&self) -> int {
        self.h2
    }

    pub(super) fn ccrest(&self) -> int {
        self.ccrest
    }

    // cc register recalculate from separate bits
    pub(super) const fn getcc(&mut self) -> u8 {
        if (self.res & 0xff) == 0 {
            self.value = (((((self.h1 & 15) + (self.h2 & 15)) & 16) << 1)
                | ((self.sign & 0x80) >> 4)
                | (4)
                | ((((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl)) & 0x80) >> 6)
                | ((self.res & 0x100) >> 8)
                | self.ccrest) as u8;
        } else {
            self.value = (((((self.h1 & 15) + (self.h2 & 15)) & 16) << 1)
                | ((self.sign & 0x80) >> 4)
                | ((((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl)) & 0x80) >> 6)
                | ((self.res & 0x100) >> 8)
                | self.ccrest) as u8;
        }

        self.value
    }

    pub(crate) const fn readCC(&mut self) -> u8 {
        self.getcc();
        self.value
    }
}
