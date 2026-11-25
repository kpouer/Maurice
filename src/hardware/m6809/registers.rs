use crate::int;

use crate::hardware::memory::Memory;
#[cfg(test)]
use rstest::rstest;

#[derive(Debug, Clone, Copy)]
pub enum Register8bit {
    A,
    B,
    DP,
    CC,
}

#[derive(Debug, Clone, Copy)]
pub enum Register16bit {
    D,
    X,
    Y,
    U,
    S,
    PC,
}

#[derive(Debug, Default)]
pub(crate) struct Registers {
    // 8bits registers
    pub(super) A: int,
    pub(super) B: int,
    pub DP: int,
    pub(crate) CC: CCRegister,

    pub(super) D: int, // D is A+B

    // 16bits registers
    pub(super) X: int,
    pub(super) Y: int,
    pub(super) U: int,
    pub(super) S: int,
    pub(crate) PC: int,
}

impl Registers {
    pub const fn read_8(&self, register: Register8bit) -> int {
        match register {
            Register8bit::A => self.A,
            Register8bit::B => self.B,
            Register8bit::DP => self.DP,
            Register8bit::CC => self.CC.value as int,
        }
    }

    pub const fn read_16(&self, register: Register16bit) -> int {
        match register {
            Register16bit::D => self.D,
            Register16bit::X => self.X,
            Register16bit::Y => self.Y,
            Register16bit::U => self.U,
            Register16bit::S => self.S,
            Register16bit::PC => self.PC,
        }
    }

    pub const fn write_8(&mut self, register: Register8bit, value: int) {
        match register {
            Register8bit::A => self.A = value,
            Register8bit::B => self.B = value,
            Register8bit::DP => self.DP = value,
            Register8bit::CC => self.CC.setcc(value as u8),
        }
    }

    pub const fn write_16(&mut self, register: Register16bit, value: int) {
        match register {
            Register16bit::D => self.D = value,
            Register16bit::X => self.X = value,
            Register16bit::Y => self.Y = value,
            Register16bit::U => self.U = value,
            Register16bit::S => self.S = value,
            Register16bit::PC => self.PC = value,
        }
    }

    pub const fn decrement(&mut self, register: Register16bit, amount: i32) -> int {
        match register {
            Register16bit::D => {
                self.D = self.D.saturating_sub(amount);
                self.D
            }
            Register16bit::X => {
                self.X = self.X.saturating_sub(amount);
                self.X
            }
            Register16bit::Y => {
                self.Y = self.Y.saturating_sub(amount);
                self.Y
            }
            Register16bit::U => {
                self.U = self.U.saturating_sub(amount);
                self.U
            }
            Register16bit::S => {
                self.S = self.S.saturating_sub(amount);
                self.S
            }
            Register16bit::PC => {
                self.PC = self.PC.saturating_sub(amount);
                self.PC
            }
        }
    }

    pub(crate) fn reset(&mut self, mem: &Memory) {
        self.PC = mem.read_16(0xFFFE);
        self.DP = 0x00;
        self.S = 0x8000;
        self.CC.value = 0x00;
    }

    // recalculate A and B or D
    pub(super) const fn CALCD(&mut self) {
        self.D = (self.A << 8) | self.B;
    }

    pub(super) const fn CALCAB(&mut self) {
        self.A = self.D >> 8;
        self.B = self.D & 0xFF;
    }

    /// Increment PC and return the previous value
    pub const fn inc_PC(&mut self, amount: int) -> int {
        let old_pc = self.PC;
        self.PC = self.PC.saturating_add(amount);
        old_pc
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

        assert_eq!(
            cc.m1(),
            expected_m1,
            "m1 mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.m2(),
            expected_m2,
            "m2 mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.res(),
            expected_res,
            "res mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.ovfl(),
            expected_ovfl,
            "ovfl mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.sign(),
            expected_sign,
            "sign mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.h1(),
            expected_h1,
            "h1 mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.h2(),
            expected_h2,
            "h2 mismatch for input 0x{:02x}",
            input
        );
        assert_eq!(
            cc.ccrest(),
            expected_ccrest,
            "ccrest mismatch for input 0x{:02x}",
            input
        );
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

    #[rstest]
    #[case(Register8bit::A, 0x12)]
    #[case(Register8bit::B, 0x34)]
    #[case(Register8bit::DP, 0x56)]
    fn test_read_write_8(#[case] register: Register8bit, #[case] value: int) {
        let mut registers = Registers::default();
        registers.write_8(register, value);
        assert_eq!(registers.read_8(register), value);
    }

    #[test]
    fn test_write_8_cc_goes_through_setcc() {
        let mut registers = Registers::default();
        registers.write_8(Register8bit::CC, 0xFF);
        assert_eq!(registers.read_8(Register8bit::CC), 0, "value is lazy");
        assert_eq!(registers.CC.readCC(), 0xFF);
        assert_eq!(registers.read_8(Register8bit::CC), 0xFF);
    }

    #[rstest]
    #[case(Register16bit::D)]
    #[case(Register16bit::X)]
    #[case(Register16bit::Y)]
    #[case(Register16bit::U)]
    #[case(Register16bit::S)]
    #[case(Register16bit::PC)]
    fn test_read_write_16(#[case] register: Register16bit) {
        let mut registers = Registers::default();
        registers.write_16(register, 0x1234);
        assert_eq!(registers.read_16(register), 0x1234);
    }

    #[test]
    fn test_registers_are_independent() {
        let mut registers = Registers::default();
        registers.write_16(Register16bit::X, 0x1111);
        registers.write_16(Register16bit::Y, 0x2222);
        registers.write_16(Register16bit::U, 0x3333);
        registers.write_16(Register16bit::S, 0x4444);
        assert_eq!(registers.read_16(Register16bit::X), 0x1111);
        assert_eq!(registers.read_16(Register16bit::Y), 0x2222);
        assert_eq!(registers.read_16(Register16bit::U), 0x3333);
        assert_eq!(registers.read_16(Register16bit::S), 0x4444);
        assert_eq!(registers.read_16(Register16bit::PC), 0);
    }

    #[rstest]
    #[case(Register16bit::D)]
    #[case(Register16bit::X)]
    #[case(Register16bit::Y)]
    #[case(Register16bit::U)]
    #[case(Register16bit::S)]
    #[case(Register16bit::PC)]
    fn test_decrement(#[case] register: Register16bit) {
        let mut registers = Registers::default();
        registers.write_16(register, 0x100);
        assert_eq!(registers.decrement(register, 1), 0xFF);
        assert_eq!(registers.read_16(register), 0xFF);
        assert_eq!(registers.decrement(register, 2), 0xFD);
    }

    #[test]
    fn test_inc_pc_returns_previous_value() {
        let mut registers = Registers::default();
        registers.PC = 0x1000;
        assert_eq!(registers.inc_PC(2), 0x1000);
        assert_eq!(registers.PC, 0x1002);
        assert_eq!(registers.inc_PC(1), 0x1002);
        assert_eq!(registers.PC, 0x1003);
    }

    #[test]
    fn test_calcd_and_calcab() {
        let mut registers = Registers::default();
        registers.A = 0x12;
        registers.B = 0x34;
        registers.CALCD();
        assert_eq!(registers.D, 0x1234);

        registers.D = 0xABCD;
        registers.CALCAB();
        assert_eq!(registers.A, 0xAB);
        assert_eq!(registers.B, 0xCD);
    }

    #[test]
    fn test_reset() {
        let mut memory = Memory::default();
        memory.set(0xFFFE, 0xF0);
        memory.set(0xFFFF, 0x12);

        let mut registers = Registers::default();
        registers.DP = 0x55;
        registers.S = 0x1234;
        registers.CC.value = 0xFF;
        registers.reset(&memory);

        assert_eq!(registers.PC, 0xF012);
        assert_eq!(registers.DP, 0);
        assert_eq!(registers.S, 0x8000);
        assert_eq!(registers.CC.value(), 0);
    }

    #[test]
    fn test_setcc_getcc_roundtrip_all_values() {
        for value in 0..=255u8 {
            let mut cc = CCRegister::default();
            cc.setcc(value);
            assert_eq!(cc.getcc(), value, "roundtrip failed for 0x{value:02x}");
        }
    }

    #[test]
    fn test_value_and_set_value() {
        let mut cc = CCRegister::default();
        cc.set_value(0x2A);
        assert_eq!(cc.value(), 0x2A);
    }
}
