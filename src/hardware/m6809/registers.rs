use crate::int;

#[cfg(test)]
use rstest::rstest;

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
    pub(super) const fn setcc(&mut self, i: u8) {
        let i = i as int;
        self.m1 = 0;
        self.m2 = 0;
        self.res = (((i & 1) << 8) | (4 - (i & 4)));
        self.ovfl = ((i & 2) << 6);
        self.sign = ((i & 8) << 4);
        self.h1 = ((i & 32) >> 2);
        self.h2 = self.h1;
        self.ccrest = (i & 0xd0);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[rstest]
    #[case(0x00, 0, 0, 4, 0, 0x00, 0, 0, 0)]
    #[case(0x01, 0, 0, 260, 0, 0x00, 0, 0, 0)]
    #[case(0x02, 0, 0, 4, 0x80, 0x00, 0, 0, 0)]
    #[case(0x04, 0, 0, 0, 0, 0x00, 0, 0, 0)]
    #[case(0x08, 0, 0, 4, 0, 0x80, 0, 0, 0)]
    #[case(0x20, 0, 0, 4, 0, 0x00, 8, 8, 0)]
    #[case(0xd0, 0, 0, 4, 0, 0x00, 0, 0, 0xd0)]
    #[case(0xff, 0, 0, 0x100, 0x80, 0x80, 8, 8, 0xd0)]
    fn test_setcc(
        #[case] input: u8,
        #[case] expected_m1: int,
        #[case] expected_m2: int,
        #[case] expected_res: int,
        #[case] expected_ovfl: int,
        #[case] expected_sign: int,
        #[case] expected_h1: int,
        #[case] expected_h2: int,
        #[case] expected_ccrest: int,
    ) {
        let mut cc = CCRegister::default();
        cc.setcc(input);

        assert_eq!(cc.m1(), expected_m1, "m1 mismatch for input 0x{:02x}", input);
        assert_eq!(cc.m2(), expected_m2, "m2 mismatch for input 0x{:02x}", input);
        assert_eq!(cc.res(), expected_res, "res mismatch for input 0x{:02x}", input);
        assert_eq!(cc.ovfl(), expected_ovfl, "ovfl mismatch for input 0x{:02x}", input);
        assert_eq!(cc.sign(), expected_sign, "sign mismatch for input 0x{:02x}", input);
        assert_eq!(cc.h1(), expected_h1, "h1 mismatch for input 0x{:02x}", input);
        assert_eq!(cc.h2(), expected_h2, "h2 mismatch for input 0x{:02x}", input);
        assert_eq!(cc.ccrest(), expected_ccrest, "ccrest mismatch for input 0x{:02x}", input);
    }

    #[rstest]
    #[case(0x00, 0, 0, 4, 0, 0x00, 0, 0, 0)]
    #[case(0x01, 0, 0, 260, 0, 0x00, 0, 0, 0)]
    #[case(0x02, 0, 0, 4, 0x80, 0x00, 0, 0, 0)]
    #[case(0x04, 0, 0, 0, 0, 0x00, 0, 0, 0)]
    #[case(0x08, 0, 0, 4, 0, 0x80, 0, 0, 0)]
    #[case(0x20, 0, 0, 4, 0, 0x00, 8, 8, 0)]
    #[case(0xd0, 0, 0, 4, 0, 0x00, 0, 0, 0xd0)]
    #[case(0xff, 0, 0, 0x100, 0x80, 0x80, 8, 8, 0xd0)]
    fn test_getcc(
        #[case] input: u8,
        #[case] expected_m1: int,
        #[case] expected_m2: int,
        #[case] expected_res: int,
        #[case] expected_ovfl: int,
        #[case] expected_sign: int,
        #[case] expected_h1: int,
        #[case] expected_h2: int,
        #[case] expected_ccrest: int,
    ) {
        let mut cc = CCRegister::default();
        cc.m1 = expected_m1;
        cc.m2 = expected_m2;
        cc.res = expected_res;
        cc.ovfl = expected_ovfl;
        cc.sign = expected_sign;
        cc.h1 = expected_h1;
        cc.h2 = expected_h2;
        cc.ccrest = expected_ccrest;

        assert_eq!(cc.getcc(), input);
    }

    #[test]
    fn test_setcc_bit_zero_carry() {
        let mut cc = CCRegister::default();
        cc.setcc(0x01);
        assert_eq!(cc.res(), 260);
    }

    #[test]
    fn test_setcc_bit_zero_no_carry() {
        let mut cc = CCRegister::default();
        cc.setcc(0x00);
        assert_eq!(cc.res(), 4);
    }

    #[test]
    fn test_setcc_bit_two_overflow() {
        let mut cc = CCRegister::default();
        cc.setcc(0x02);
        assert_eq!(cc.ovfl(), 0x80);
    }

    #[test]
    fn test_setcc_bit_three_sign() {
        let mut cc = CCRegister::default();
        cc.setcc(0x08);
        assert_eq!(cc.sign(), 0x80);
    }

    #[test]
    fn test_setcc_bit_five_half_carry() {
        let mut cc = CCRegister::default();
        cc.setcc(0x20);
        assert_eq!(cc.h1(), 8);
        assert_eq!(cc.h2(), 8);
    }

    #[test]
    fn test_setcc_upper_bits_preserved() {
        let mut cc = CCRegister::default();
        cc.setcc(0xd0);
        assert_eq!(cc.ccrest(), 0xd0);
    }
}
