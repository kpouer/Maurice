#![allow(non_snake_case)]

use crate::hardware::memory::Memory;
use crate::hardware::screen::Screen;
use crate::hardware::sound::Sound;
use crate::int;
use log::warn;
use std::ops::Index;

const SOUND_SIZE: usize = 1024;

#[derive(Debug, Default)]
pub struct M6809 {
    // Sound emulation parameters
    pub(crate) sound_buffer: SoundBuffer,

    cl: int,

    // 8bits registers
    A: int,
    B: int,
    DP: int,
    CC: int,

    // 16bits registers
    X: int,
    Y: int,
    U: int,
    S: int,
    pub(crate) PC: int,
    D: int, // D is A+B

    // fast CC bits (as ints)
    res: int,
    m1: int,
    m2: int,
    sign: int,
    ovfl: int,
    h1: int,
    h2: int,
    ccrest: int,
}

impl M6809 {
    pub fn new(mem: &Memory) -> Self {
        let mut m6809 = M6809::default();
        m6809.reset(mem);
        m6809
    }

    pub(crate) fn reset(&mut self, mem: &Memory) {
        self.PC = mem.read_16(0xFFFE);
        self.DP = 0x00;
        self.S = 0x8000;
        self.CC = 0x00;
    }

    // recalculate A and B or D
    const fn CALCD(&mut self) {
        self.D = (self.A << 8) | self.B;
    }

    const fn CALCAB(&mut self) {
        self.A = self.D >> 8;
        self.B = self.D & 0xFF;
    }

    // basic 6809 addressing modes
    const fn IMMED8(&mut self) -> int {
        let M = self.PC;
        self.PC += 1;
        M
    }

    const fn IMMED16(&mut self) -> int {
        let M = self.PC;
        self.PC += 2;
        M
    }

    fn DIREC(&mut self, mem: &mut Memory) -> int {
        let M = (self.DP << 8) | mem.read(self.PC);
        self.PC += 1;
        M
    }

    fn ETEND(&mut self, mem: &mut Memory) -> int {
        let mut M = mem.read(self.PC) << 8;
        self.PC += 1;
        M |= mem.read(self.PC);
        self.PC += 1;
        M
    }

    fn INDEXE(&mut self, mem: &mut Memory) -> int {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        if m < 0x80 {
            // effectue le complement a 2 sur la precision int
            let delta = if (m & 0x10) == 0x10 {
                ((-1 >> 5) << 5) | (m & 0x1F)
            } else {
                m & 0x1F
            };
            let reg = match m & 0xE0 {
                0x00 => self.X,
                0x20 => self.Y,
                0x40 => self.U,
                0x60 => self.S,
                _ => return 0,
            };
            self.cl += 1;
            return (reg + delta) & 0xFFFF;
        }
        match m {
            0x80 => {
                //i_d_P1_X
                let M = self.X;
                self.X = (self.X + 1) & 0xFFFF;
                self.cl += 2;
                M
            }
            0x81 => {
                //i_d_P2_X
                let M = self.X;
                self.X = (self.X + 2) & 0xFFFF;
                self.cl += 3;
                M
            }
            0x82 => {
                //i_d_M1_X
                self.X = (self.X - 1) & 0xFFFF;
                let M = self.X;
                self.cl += 2;
                M
            }
            0x83 => {
                //i_d_M2_X
                self.X = (self.X - 2) & 0xFFFF;
                let M = self.X;
                self.cl += 3;
                M
            }
            0x84 => self.X, //i_d_X
            0x85 => {
                //i_d_B_X
                let M = (self.X + signedChar(self.B)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0x86 => {
                //i_d_A_X;
                let M = (self.X + signedChar(self.A)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0x87 => 0, //i_undoc;	/* empty */
            0x88 => {
                //i_d_8_X;
                self.m2 = mem.read(self.PC);
                self.PC += 1;
                let M = (self.X + signedChar(self.m2)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0x89 => {
                //i_d_16_X;
                self.m2 = mem.read_16(self.PC);
                self.PC += 2;
                let M = (self.X + signed16bits(self.m2)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0x8A => 0, //i_undoc;	/* empty */
            0x8B => {
                //i_d_D_X;
                let M = (self.X + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0x8C | 0xAC | 0xCC | 0xEC => {
                //i_d_PC8;
                m = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let M = (self.PC + signedChar(m)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0x8D | 0xAD | 0xCD | 0xED => {
                //i_d_PC16;
                let mut M = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                M = (self.PC + signed16bits(M)) & 0xFFFF;
                self.cl += 5;
                M
            }
            0x8E..=0x90 => 0, //i_undoc;	/* empty */
            0x91 => {
                //i_i_P2_X;
                let M = mem.read_16(self.X);
                self.X = (self.X + 2) & 0xFFFF;
                self.cl += 6;
                M
            }
            0x92 => 0, //i_undoc;	/* empty */
            0x93 => {
                //i_i_M2_X;
                self.X = (self.X - 2) & 0xFFFF;
                let M = mem.read_16(self.X);
                self.cl += 6;
                M
            }
            0x94 => {
                //i_i_0_X;
                let M = mem.read_16(self.X);
                self.cl += 3;
                M
            }
            0x95 => {
                //i_i_B_X;
                let mut M = (self.X + signedChar(self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0x96 => {
                //i_i_A_X;
                let mut M = (self.X + signedChar(self.A)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0x97 => 0, //i_undoc;	/* empty */
            0x98 => {
                //i_i_8_X;
                self.m2 = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let mut M = (self.X + signedChar(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0x99 => {
                //i_i_16_X;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let mut M = (self.X + signed16bits(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0x9A => 0, //i_undoc;	/* empty */
            0x9B => {
                //i_i_D_X;
                let mut M = (self.X + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0x9C | 0xBC | 0xDC | 0xFC => {
                //i_i_PC8;
                self.m2 = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let mut M = (self.PC + signedChar(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0x9D | 0xBD | 0xDD | 0xFD => {
                //i_i_PC16;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let mut M = (self.PC + signed16bits(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 8;
                M
            }
            0x9E => 0, //i_undoc;	/* empty */
            0x9F | 0xBF | 0xDF | 0xFF => {
                //i_i_e16;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let M = mem.read_16(self.m2);
                self.cl += 5;
                M
                // Y
            }
            0xA0 => {
                //i_d_P1_Y;
                let M = self.Y;
                self.Y = (self.Y + 1) & 0xFFFF;
                self.cl += 2;
                M
            }
            0xA1 => {
                //i_d_P2_Y;
                let M = self.Y;
                self.Y = (self.Y + 2) & 0xFFFF;
                self.cl += 3;
                M
            }
            0xA2 => {
                //i_d_M1_Y;
                self.Y = (self.Y - 1) & 0xFFFF;
                let M = self.Y;
                self.cl += 2;
                M
            }
            0xA3 => {
                //i_d_M2_Y;
                self.Y = (self.Y - 2) & 0xFFFF;
                let M = self.Y;
                self.cl += 3;
                M
            }
            0xA4 => self.Y, //i_d_Y;
            0xA5 => {
                //i_d_B_Y;
                let M = (self.Y + signedChar(self.B)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xA6 => {
                //i_d_A_Y;
                let M = (self.Y + signedChar(self.A)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xA7 => 0, //i_undoc;	/* empty */
            0xA8 => {
                //i_d_8_Y;
                self.m2 = mem.read(self.PC);
                self.PC += 1;
                let M = (self.Y + signedChar(self.m2)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xA9 => {
                //i_d_16_Y;
                self.m2 = mem.read_16(self.PC);
                self.PC += 2;
                let M = (self.Y + signed16bits(self.m2)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xAA => 0, //i_undoc;	/* empty */
            0xAB => {
                //i_d_D_Y;
                let M = (self.Y + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xAE..=0xB0 => 0, //i_undoc;	/* empty */
            0xB1 => {
                //i_i_P2_Y;
                let M = mem.read_16(self.Y);
                self.Y = (self.Y + 2) & 0xFFFF;
                self.cl += 6;
                M
            }
            0xB2 => 0, //i_undoc;	/* empty */
            0xB3 => {
                //i_i_M2_Y;
                self.Y = (self.Y - 2) & 0xFFFF;
                let M = mem.read_16(self.Y);
                self.cl += 6;
                M
            }
            0xB4 => {
                //i_i_0_Y;
                let M = mem.read_16(self.Y);
                self.cl += 3;
                M
            }
            0xB5 => {
                //i_i_B_Y;
                let mut M = (self.Y + signedChar(self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xB6 => {
                //i_i_A_Y;
                let mut M = (self.Y + signedChar(self.A)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xB7 => 0, //i_undoc;	/* empty */
            0xB8 => {
                //i_i_8_Y;
                self.m2 = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let mut M = (self.Y + signedChar(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xB9 => {
                //i_i_16_Y;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let mut M = (self.Y + signed16bits(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xBA => 0, //i_undoc;	/* empty */
            0xBB => {
                //i_i_D_Y;
                let mut M = (self.Y + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xBE => 0, //i_undoc;	/* empty */ U
            0xC0 => {
                //i_d_P1_U;
                let M = self.U;
                self.U = (self.U + 1) & 0xFFFF;
                self.cl += 2;
                M
            }
            0xC1 => {
                //i_d_P2_U;
                let M = self.U;
                self.U = (self.U + 2) & 0xFFFF;
                self.cl += 3;
                M
            }
            0xC2 => {
                //i_d_M1_U;
                self.U = (self.U - 1) & 0xFFFF;
                let M = self.U;
                self.cl += 2;
                M
            }
            0xC3 => {
                //i_d_M2_U;
                self.U = (self.U - 2) & 0xFFFF;
                let M = self.U;
                self.cl += 3;
                M
            }
            0xC4 => self.U, //i_d_U;
            0xC5 => {
                //i_d_B_U;
                let M = (self.U + signedChar(self.B)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xC6 => {
                //i_d_A_U;
                let M = (self.U + signedChar(self.A)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xC7 => 0, //i_undoc;	/* empty */
            0xC8 => {
                //i_d_8_U;
                self.m2 = mem.read(self.PC);
                self.PC += 1;
                let M = (self.U + signedChar(self.m2)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xC9 => {
                //i_d_16_U;
                self.m2 = mem.read_16(self.PC);
                self.PC += 2;
                let M = (self.U + signed16bits(self.m2)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xCA => 0, //i_undoc;	/* empty */
            0xCB => {
                //i_d_D_U;
                let M = (self.U + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xCE..=0xD0 => 0, //i_undoc;	/* empty */
            0xD1 => {
                //i_i_P2_U;
                let M = mem.read_16(self.U);
                self.U = (self.U + 2) & 0xFFFF;
                self.cl += 6;
                M
            }
            0xD2 => 0, //i_undoc;	/* empty */
            0xD3 => {
                //i_i_M2_U;
                self.U = (self.U - 2) & 0xFFFF;
                let M = mem.read_16(self.U);
                self.cl += 6;
                M
            }
            0xD4 => {
                //i_i_0_U;
                let M = mem.read_16(self.U);
                self.cl += 3;
                M
            }
            0xD5 => {
                //i_i_B_U;
                let mut M = (self.U + signedChar(self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xD6 => {
                //i_i_A_U;
                let mut M = (self.U + signedChar(self.A)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xD7 => 0, //i_undoc;	/* empty */
            0xD8 => {
                //i_i_8_U;
                self.m2 = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let mut M = (self.U + signedChar(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xD9 => {
                //i_i_16_U;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let mut M = (self.U + signed16bits(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xDA => 0, //i_undoc;	/* empty */
            0xDB => {
                //i_i_D_U;
                let mut M = (self.U + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xDE => 0, //i_undoc;	/* empty */// S
            0xE0 => {
                //i_d_P1_S;
                let M = self.S;
                self.S = (self.S + 1) & 0xFFFF;
                self.cl += 2;
                M
            }
            0xE1 => {
                //i_d_P2_S;
                let M = self.S;
                self.S = (self.S + 2) & 0xFFFF;
                self.cl += 3;
                M
            }
            0xE2 => {
                //i_d_M1_S;
                self.S = (self.S - 1) & 0xFFFF;
                let M = self.S;
                self.cl += 2;
                M
            }
            0xE3 => {
                //i_d_M2_S;
                self.S = (self.S - 2) & 0xFFFF;
                let M = self.S;
                self.cl += 3;
                M
            }
            0xE4 => self.S, //i_d_S;
            0xE5 => {
                //i_d_B_S;
                let M = (self.S + signedChar(self.B)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xE6 => {
                //i_d_A_S;
                let M = (self.S + signedChar(self.A)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xE7 => 0, //i_undoc;	/* empty */
            0xE8 => {
                //i_d_8_S;
                self.m2 = mem.read(self.PC);
                self.PC += 1;
                let M = (self.S + signedChar(self.m2)) & 0xFFFF;
                self.cl += 1;
                M
            }
            0xE9 => {
                //i_d_16_S;
                self.m2 = mem.read_16(self.PC);
                self.PC += 2;
                let M = (self.S + signed16bits(self.m2)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xEA => 0, //i_undoc;	/* empty */
            0xEB => {
                //i_d_D_S;
                let M = (self.S + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                self.cl += 4;
                M
            }
            0xEE..=0xF0 => 0, //i_undoc;	/* empty */
            0xF1 => {
                //i_i_P2_S;
                let M = mem.read_16(self.S);
                self.S = (self.S + 2) & 0xFFFF;
                self.cl += 6;
                M
            }
            0xF2 => 0, //i_undoc;	/* empty */
            0xF3 => {
                //i_i_M2_S;
                self.S = (self.S - 2) & 0xFFFF;
                let M = mem.read_16(self.S);
                self.cl += 6;
                M
            }
            0xF4 => {
                //i_i_0_S;
                let M = mem.read_16(self.S);
                self.cl += 3;
                M
            }
            0xF5 => {
                //i_i_B_S;
                let mut M = (self.S + signedChar(self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xF6 => {
                //i_i_A_S;
                let mut M = (self.S + signedChar(self.A)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xF7 => 0, //i_undoc;	/* empty */
            0xF8 => {
                //i_i_8_S;
                self.m2 = mem.read(self.PC);
                self.PC = (self.PC + 1) & 0xFFFF;
                let mut M = (self.S + signedChar(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 4;
                M
            }
            0xF9 => {
                //i_i_16_S;
                self.m2 = mem.read_16(self.PC);
                self.PC = (self.PC + 2) & 0xFFFF;
                let mut M = (self.S + signed16bits(self.m2)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xFA => 0, //i_undoc;	/* empty */
            0xFB => {
                //i_i_D_S;
                let mut M = (self.S + signed16bits((self.A << 8) | self.B)) & 0xFFFF;
                M = mem.read_16(M);
                self.cl += 7;
                M
            }
            0xFE => 0, //i_undoc;	/* empty */
            _ => {
                warn!("Unknown opcode {m:02X}");
                0
            }
        }
    }

    // cc register recalculate from separate bits
    const fn getcc(&mut self) -> int {
        if (self.res & 0xff) == 0 {
            self.CC = ((((self.h1 & 15) + (self.h2 & 15)) & 16) << 1)
                | ((self.sign & 0x80) >> 4)
                | (4)
                | ((((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl)) & 0x80) >> 6)
                | ((self.res & 0x100) >> 8)
                | self.ccrest;
        } else {
            self.CC = ((((self.h1 & 15) + (self.h2 & 15)) & 16) << 1)
                | ((self.sign & 0x80) >> 4)
                | ((((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl)) & 0x80) >> 6)
                | ((self.res & 0x100) >> 8)
                | self.ccrest;
        }

        self.CC
    }

    // calculate CC fast bits from CC register
    const fn setcc(&mut self, i: int) {
        self.m1 = 0;
        self.m2 = 0;
        self.res = ((i & 1) << 8) | (4 - (i & 4));
        self.ovfl = (i & 2) << 6;
        self.sign = (i & 8) << 4;
        self.h1 = (i & 32) >> 2;
        self.h2 = self.h1;
        self.ccrest = i & 0xd0;
    }

    pub(crate) const fn readCC(&mut self) -> int {
        self.getcc();
        self.CC
    }

    fn LOAD8(ADR: int, mem: &mut Memory) -> int {
        mem.read(ADR)
    }

    // LDx
    fn LD8(&mut self, M: int, c: int, mem: &mut Memory) -> int {
        self.sign = mem.read(M);
        self.m1 = self.ovfl;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
        self.sign
    }

    fn LD16(&mut self, M: int, c: int, mem: &mut Memory) -> int {
        let R = mem.read_16(M) & 0xFFFF;
        self.m1 = self.ovfl;
        self.sign = R >> 8;
        self.res = (self.res & 0x100) | ((self.sign | R) & 0xFF);
        self.cl += c;
        R
    }

    // STx
    fn ST8(&mut self, R: int, adr: int, c: int, mem: &mut Memory) {
        mem.write(adr, R);
        self.m1 = self.ovfl;
        self.sign = R;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ST16(&mut self, R: int, adr: int, c: int, mem: &mut Memory) {
        mem.write(adr, R >> 8);
        mem.write(adr + 1, R & 0xFF);
        self.m1 = self.ovfl;
        self.sign = R >> 8;
        self.res = (self.res & 0x100) | ((self.sign | R) & 0xFF);
        self.cl += c;
    }

    // LEA
    fn LEA(&mut self, mem: &mut Memory) -> int {
        let R = self.INDEXE(mem);
        self.res = (self.res & 0x100) | ((R | (R >> 8)) & 0xFF);
        self.cl += 4;
        R
    }

    // CLR
    fn CLR(&mut self, M: int, c: int, mem: &mut Memory) {
        mem.write(M, 0);
        self.m1 = !self.m2;
        self.sign = 0;
        self.res = 0;
        self.cl += c;
    }

    // EXG
    fn EXG(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        let r1 = (m & 0xF0) >> 4;
        // only for javac
        // of match r1
        let k: int = match r1 {
            0x00 => (self.A << 8) | self.B,
            0x01 => self.X,
            0x02 => self.Y,
            0x03 => self.U,
            0x04 => self.S,
            0x05 => self.PC,
            0x06 => self.getcc(),
            0x07 => self.getcc(),
            0x08 => self.A,
            0x09 => self.B,
            0x0A => self.getcc(),
            0x0B => self.DP,
            0x0C => self.getcc(),
            0x0D => self.getcc(),
            0x0E => self.getcc(),
            0x0F => self.getcc(),
            _ => 0, // only for javac
        };
        let mut l = 0;
        let r2 = m & 0x0F;
        match r2 {
            0x00 => {
                l = (self.A << 8) | self.B;
                self.A = (k >> 8) & 255;
                self.B = k & 255;
            }
            0x01 => {
                l = self.X;
                self.X = k;
            }
            0x02 => {
                l = self.Y;
                self.Y = k;
            }
            0x03 => {
                l = self.U;
                self.U = k;
            }
            0x04 => {
                l = self.S;
                self.S = k;
            }
            0x05 => {
                l = self.PC;
                self.PC = k;
            }
            0x06 => {
                l = self.getcc();
                self.setcc(k);
            }
            0x07 => {
                l = self.getcc();
                self.setcc(k);
            }
            0x08 => {
                l = self.A;
                self.A = k & 0xff;
            }
            0x09 => {
                l = self.B;
                self.B = k & 0xff;
            }
            0x0A => {
                l = self.getcc();
                self.setcc(k);
            }
            0x0B => {
                l = self.DP;
                self.DP = k & 0xff;
            }
            0x0C => {
                l = self.getcc();
                self.setcc(k);
            }
            0x0D => {
                l = self.getcc();
                self.setcc(k);
            }
            0x0E => {
                l = self.getcc();
                self.setcc(k);
            }
            0x0F => {
                l = self.getcc();
                self.setcc(k);
            }
            _ => {}
        } // of match r2
        match r1 {
            0x00 => {
                self.A = (l >> 8) & 255;
                self.B = l & 255;
            }
            0x01 => self.X = l,
            0x02 => self.Y = l,
            0x03 => self.U = l,
            0x04 => self.S = l,
            0x05 => self.PC = l,
            0x06 => self.setcc(l),
            0x07 => self.setcc(l),
            0x08 => self.A = l & 0xff,
            0x09 => self.B = l & 0xff,
            0x0A => self.setcc(l),
            0x0B => self.DP = l & 0xff,
            0x0C => self.setcc(l),
            0x0D => self.setcc(l),
            0x0E => self.setcc(l),
            0x0F => self.setcc(l),
            _ => {}
        } // of second match r1
        self.cl += 8;
    }

    fn TFR(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        let r1 = (m & 0xF0) >> 4;
        // only for javac
        // of match r1
        let k: int = match r1 {
            0x00 => (self.A << 8) | self.B,
            0x01 => self.X,
            0x02 => self.Y,
            0x03 => self.U,
            0x04 => self.S,
            0x05 => self.PC,
            0x06 | 0x07 => self.getcc(),
            0x08 => self.A,
            0x09 => self.B,
            0x0A => self.getcc(),
            0x0B => self.DP,
            0x0C | 0x0D | 0x0E | 0x0F => self.getcc(),
            _ => 0,
        };
        let r2 = m & 0x0F;
        match r2 {
            0x00 => {
                self.A = (k >> 8) & 255;
                self.B = k & 255;
            }
            0x01 => self.X = k,
            0x02 => self.Y = k,
            0x03 => self.U = k,
            0x04 => self.S = k,
            0x05 => self.PC = k,
            0x06 | 0x07 => self.setcc(k),
            0x08 => self.A = k & 0xff,
            0x09 => self.B = k & 0xff,
            0x0A => self.setcc(k),
            0x0B => self.DP = k & 0xff,
            0x0C | 0x0D | 0x0E | 0x0F => self.setcc(k),
            _ => {}
        } // of match r2
    }

    fn PSHS(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (m & 0x80) != 0 {
            self.S -= 1;
            mem.write(self.S, self.PC & 0x00FF);
            self.S -= 1;
            mem.write(self.S, self.PC >> 8);
            self.cl += 2;
        }
        if (m & 0x40) != 0 {
            self.S -= 1;
            mem.write(self.S, self.U & 0x00FF);
            self.S -= 1;
            mem.write(self.S, self.U >> 8);
            self.cl += 2;
        }
        if (m & 0x20) != 0 {
            self.S -= 1;
            mem.write(self.S, self.Y & 0x00FF);
            self.S -= 1;
            mem.write(self.S, self.Y >> 8);
            self.cl += 2;
        }
        if (m & 0x10) != 0 {
            self.S -= 1;
            mem.write(self.S, self.X & 0x00FF);
            self.S -= 1;
            mem.write(self.S, self.X >> 8);
            self.cl += 2;
        }
        if (m & 0x08) != 0 {
            self.S -= 1;
            mem.write(self.S, self.DP);
            self.cl += 1;
        }
        if (m & 0x04) != 0 {
            self.S -= 1;
            mem.write(self.S, self.B);
            self.cl += 1;
        }
        if (m & 0x02) != 0 {
            self.S -= 1;
            mem.write(self.S, self.A);
            self.cl += 1;
        }
        if (m & 0x01) != 0 {
            self.S -= 1;
            self.getcc();
            mem.write(self.S, self.CC);
            self.cl += 1;
        }
        self.cl += 5;
    }

    fn PSHU(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (m & 0x80) != 0 {
            self.U -= 1;
            mem.write(self.U, self.PC & 0x00FF);
            self.U -= 1;
            mem.write(self.U, self.PC >> 8);
            self.cl += 2;
        }
        if (m & 0x40) != 0 {
            self.U -= 1;
            mem.write(self.U, self.S & 0x00FF);
            self.U -= 1;
            mem.write(self.U, self.S >> 8);
            self.cl += 2;
        }
        if (m & 0x20) != 0 {
            self.U -= 1;
            mem.write(self.U, self.Y & 0x00FF);
            self.U -= 1;
            mem.write(self.U, self.Y >> 8);
            self.cl += 2;
        }
        if (m & 0x10) != 0 {
            self.U -= 1;
            mem.write(self.U, self.X & 0x00FF);
            self.U -= 1;
            mem.write(self.U, self.X >> 8);
            self.cl += 2;
        }
        if (m & 0x08) != 0 {
            self.U -= 1;
            mem.write(self.U, self.DP);
            self.cl += 1;
        }
        if (m & 0x04) != 0 {
            self.U -= 1;
            mem.write(self.U, self.B);
            self.cl += 1;
        }
        if (m & 0x02) != 0 {
            self.U -= 1;
            mem.write(self.U, self.A);
            self.cl += 1;
        }
        if (m & 0x01) != 0 {
            self.U -= 1;
            self.getcc();
            mem.write(self.U, self.CC);
            self.cl += 1;
        }
        self.cl += 5;
    }

    fn PULS(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (m & 0x01) != 0 {
            self.CC = mem.read(self.S);
            self.setcc(self.CC);
            self.S += 1;
            self.cl += 1;
        }
        if (m & 0x02) != 0 {
            self.A = mem.read(self.S);
            self.S += 1;
            self.cl += 1;
        }
        if (m & 0x04) != 0 {
            self.B = mem.read(self.S);
            self.S += 1;
            self.cl += 1;
        }
        if (m & 0x08) != 0 {
            self.DP = mem.read(self.S);
            self.S += 1;
            self.cl += 1;
        }
        if (m & 0x10) != 0 {
            self.X = mem.read_16(self.S);
            self.S += 2;
            self.cl += 2;
        }
        if (m & 0x20) != 0 {
            self.Y = mem.read_16(self.S);
            self.S += 2;
            self.cl += 2;
        }
        if (m & 0x40) != 0 {
            self.U = mem.read_16(self.S);
            self.S += 2;
            self.cl += 2;
        }
        if (m & 0x80) != 0 {
            self.PC = mem.read_16(self.S);
            self.S += 2;
            self.cl += 2;
        }
        self.cl += 5;
    }

    fn PULU(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (m & 0x01) != 0 {
            self.CC = mem.read(self.U);
            self.setcc(self.CC);
            self.U += 1;
            self.cl += 1;
        }
        if (m & 0x02) != 0 {
            self.A = mem.read(self.U);
            self.U += 1;
            self.cl += 1;
        }
        if (m & 0x04) != 0 {
            self.B = mem.read(self.U);
            self.U += 1;
            self.cl += 1;
        }
        if (m & 0x08) != 0 {
            self.DP = mem.read(self.U);
            self.U += 1;
            self.cl += 1;
        }
        if (m & 0x10) != 0 {
            self.X = mem.read_16(self.U);
            self.U += 2;
            self.cl += 2;
        }
        if (m & 0x20) != 0 {
            self.Y = mem.read_16(self.U);
            self.U += 2;
            self.cl += 2;
        }
        if (m & 0x40) != 0 {
            self.S = mem.read_16(self.U);
            self.U += 2;
            self.cl += 2;
        }
        if (m & 0x80) != 0 {
            self.PC = mem.read_16(self.U);
            self.U += 2;
            self.cl += 2;
        }
        self.cl += 5;
    }

    const fn INCA(&mut self) {
        self.m1 = self.A;
        self.m2 = 0;
        self.A = (self.A + 1) & 0xFF;
        self.ovfl = self.A;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    const fn INCB(&mut self) {
        self.m1 = self.B;
        self.m2 = 0;
        self.B = (self.B + 1) & 0xFF;
        self.ovfl = self.B;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    fn INC(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = val;
        self.m2 = 0;
        val += 1;
        mem.write(adr, val);
        self.ovfl = val & 0xFF;
        self.sign = self.ovfl;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    // DEC
    const fn DECA(&mut self) {
        self.m1 = self.A;
        self.m2 = 0x80;
        self.A = (self.A - 1) & 0xFF;
        self.ovfl = self.A;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    const fn DECB(&mut self) {
        self.m1 = self.B;
        self.m2 = 0x80;
        self.B = (self.B - 1) & 0xFF;
        self.ovfl = self.B;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    fn DEC(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = val;
        self.m2 = 0x80;
        val -= 1;
        mem.write(adr, val);
        self.ovfl = val & 0xFF;
        self.sign = self.ovfl;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn BIT(&mut self, R: int, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.sign = R & val;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn CMP8(&mut self, R: int, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = R;
        self.m2 = -val;
        self.ovfl = R - val;
        self.res = self.ovfl;
        self.sign = self.ovfl;
        self.cl += c;
    }

    fn CMP16(&mut self, R: int, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read_16(adr);
        self.m1 = R >> 8;
        self.m2 = (-val) >> 8;
        self.ovfl = ((R - val) >> 8) & 0xFFFFFF;
        self.res = self.ovfl;
        self.sign = self.ovfl;
        self.res |= (R - val) & 0xFF;
        self.cl += c;
    }

    // TST
    const fn TSTAi(&mut self) {
        self.m1 = self.ovfl;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    const fn TSTBi(&mut self) {
        self.m1 = self.ovfl;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    fn TST(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = !self.m2;
        self.sign = val;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ANDA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.A &= val;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ANDB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.B &= val;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ANDCC(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        //	getcc();
        self.CC &= val;
        self.setcc(self.CC);
        self.cl += c;
    }

    fn ORA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.A |= val;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ORB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.B |= val;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn ORCC(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.getcc();
        self.CC |= val;
        self.setcc(self.CC);
        self.cl += c;
    }

    fn EORA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.A ^= val;
        self.sign = self.A;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    fn EORB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.ovfl;
        self.B ^= val;
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += c;
    }

    const fn COMA(&mut self) {
        self.m1 = self.ovfl;
        self.A = (!self.A) & 0xFF;
        self.sign = self.A;
        self.res = self.sign | 0x100;
        self.cl += 2;
    }

    const fn COMB(&mut self) {
        self.m1 = self.ovfl;
        self.B = (!self.B) & 0xFF;
        self.sign = self.B;
        self.res = self.sign | 0x100;
        self.cl += 2;
    }

    fn COM(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = !self.m2;
        val = (!val) & 0xFF;
        mem.write(adr, val);
        self.sign = val;
        self.res = self.sign | 0x100;
        self.cl += c;
    }

    const fn NEGA(&mut self) {
        self.m1 = self.A;
        self.m2 = -self.A;
        self.A = -self.A;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.A &= 0xFF;
        self.cl += 2;
    }

    const fn NEGB(&mut self) {
        self.m1 = self.B;
        self.m2 = -self.B;
        self.B = -self.B;
        self.ovfl = self.B;
        self.res = self.B;
        self.sign = self.B;
        self.B &= 0xFF;
        self.cl += 2;
    }

    fn NEG(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = val;
        self.m2 = -val;
        val = -val;
        mem.write(adr, val);
        self.ovfl = val;
        self.res = val;
        self.sign = val;
        self.cl += c;
    }

    const fn ABX(&mut self) {
        self.X = (self.X + self.B) & 0xFFFF;
        self.cl += 3;
    }

    fn ADDA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.A;
        self.h1 = self.A;
        self.m2 = val;
        self.h2 = val;
        self.A += val;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.A &= 0xFF;
        self.cl += c;
    }

    fn ADDB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.B;
        self.h1 = self.B;
        self.m2 = val;
        self.h2 = val;
        self.B += val;
        self.ovfl = self.B;
        self.res = self.B;
        self.sign = self.B;
        self.B &= 0xFF;
        self.cl += c;
    }

    fn ADDD(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read_16(adr);
        self.m1 = self.A;
        self.m2 = val >> 8;
        self.D = (self.A << 8) + self.B + val;
        self.A = self.D >> 8;
        self.B = self.D & 0xFF;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.res |= self.B;
        self.A &= 0xFF;
        self.cl += c;
    }

    fn ADCA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.A;
        self.h1 = self.A;
        self.m2 = val;
        self.h2 = val + ((self.res & 0x100) >> 8);
        self.A += self.h2;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.A &= 0xFF;
        self.cl += c;
    }

    fn ADCB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.B;
        self.h1 = self.B;
        self.m2 = val;
        self.h2 = val + ((self.res & 0x100) >> 8);
        self.B += self.h2;
        self.ovfl = self.B;
        self.res = self.B;
        self.sign = self.B;
        self.B &= 0xFF;
        self.cl += c;
    }

    const fn MUL(&mut self) {
        let k = self.A * self.B;
        self.A = (k >> 8) & 0xFF;
        self.B = k & 0xFF;
        self.res = ((self.B & 0x80) << 1) | ((k | (k >> 8)) & 0xFF);
        self.cl += 11;
    }

    fn SBCA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.A;
        self.m2 = -val;
        self.A -= val + ((self.res & 0x100) >> 8);
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.A &= 0xFF;
        self.cl += c;
    }

    fn SBCB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.B;
        self.m2 = -val;
        self.B -= val + ((self.res & 0x100) >> 8);
        self.ovfl = self.B;
        self.res = self.B;
        self.sign = self.B;
        self.B &= 0xFF;
        self.cl += c;
    }

    fn SUBA(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.A;
        self.m2 = -val;
        self.A -= val;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.A &= 0xFF;
        self.cl += c;
    }

    fn SUBB(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read(adr);
        self.m1 = self.B;
        self.m2 = -val;
        self.B -= val;
        self.ovfl = self.B;
        self.res = self.B;
        self.sign = self.B;
        self.B &= 0xFF;
        self.cl += c;
    }

    fn SUBD(&mut self, adr: int, c: int, mem: &mut Memory) {
        let val = mem.read_16(adr);
        self.m1 = self.A;
        self.m2 = (-val) >> 8;
        self.D = (self.A << 8) + self.B - val;
        self.A = self.D >> 8;
        self.B = self.D & 0xFF;
        self.ovfl = self.A;
        self.res = self.A;
        self.sign = self.A;
        self.res |= self.B;
        self.A &= 0xFF;
        self.cl += c;
    }

    const fn SEX(&mut self) {
        if (self.B & 0x80) == 0x80 {
            self.A = 0xFF;
        } else {
            self.A = 0;
        }
        self.sign = self.B;
        self.res = (self.res & 0x100) | self.sign;
        self.cl += 2;
    }

    const fn ASLA(&mut self) {
        self.m1 = self.A;
        self.m2 = self.A;
        self.A <<= 1;
        self.ovfl = self.A;
        self.sign = self.A;
        self.res = self.A;
        self.A &= 0xFF;
        self.cl += 2;
    }

    const fn ASLB(&mut self) {
        self.m1 = self.B;
        self.m2 = self.B;
        self.B <<= 1;
        self.ovfl = self.B;
        self.sign = self.B;
        self.res = self.B;
        self.B &= 0xFF;
        self.cl += 2;
    }

    fn ASL(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = val;
        self.m2 = val;
        val <<= 1;
        mem.write(adr, val);
        self.ovfl = val;
        self.sign = val;
        self.res = val;
        self.cl += c;
    }

    const fn ASRA(&mut self) {
        self.res = (self.A & 1) << 8;
        self.A = (self.A >> 1) | (self.A & 0x80);
        self.sign = self.A;
        self.res |= self.sign;
        self.cl += 2;
    }

    const fn ASRB(&mut self) {
        self.res = (self.B & 1) << 8;
        self.B = (self.B >> 1) | (self.B & 0x80);
        self.sign = self.B;
        self.res |= self.sign;
        self.cl += 2;
    }

    fn ASR(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.res = (val & 1) << 8;
        val = (val >> 1) | (val & 0x80);
        mem.write(adr, val);
        self.sign = val;
        self.res |= self.sign;
        self.cl += c;
    }

    const fn LSRA(&mut self) {
        self.res = (self.A & 1) << 8;
        self.A >>= 1;
        self.sign = 0;
        self.res |= self.A;
        self.cl += 2;
    }

    const fn LSRB(&mut self) {
        self.res = (self.B & 1) << 8;
        self.B >>= 1;
        self.sign = 0;
        self.res |= self.B;
        self.cl += 2;
    }

    fn LSR(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.res = (val & 1) << 8;
        val >>= 1;
        mem.write(adr, val);
        self.sign = 0;
        self.res |= val;
        self.cl += c;
    }

    const fn ROLA(&mut self) {
        self.m1 = self.A;
        self.m2 = self.A;
        self.A = (self.A << 1) | ((self.res & 0x100) >> 8);
        self.ovfl = self.A;
        self.sign = self.A;
        self.res = self.A;
        self.A &= 0xFF;
        self.cl += 2;
    }

    const fn ROLB(&mut self) {
        self.m1 = self.B;
        self.m2 = self.B;
        self.B = (self.B << 1) | ((self.res & 0x100) >> 8);
        self.ovfl = self.B;
        self.sign = self.B;
        self.res = self.B;
        self.B &= 0xFF;
        self.cl += 2;
    }

    fn ROL(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val = mem.read(adr);
        self.m1 = val;
        self.m2 = val;
        val = (val << 1) | ((self.res & 0x100) >> 8);
        mem.write(adr, val);
        self.ovfl = val;
        self.sign = val;
        self.res = val;
        self.cl += c;
    }

    const fn RORA(&mut self) {
        let i = self.A;
        self.A = (self.A | (self.res & 0x100)) >> 1;
        self.sign = self.A;
        self.res = ((i & 1) << 8) | self.sign;
        self.cl += 2;
    }

    const fn RORB(&mut self) {
        let i = self.B;
        self.B = (self.B | (self.res & 0x100)) >> 1;
        self.sign = self.B;
        self.res = ((i & 1) << 8) | self.sign;
        self.cl += 2;
    }

    fn ROR(&mut self, adr: int, c: int, mem: &mut Memory) {
        let mut val;
        let i = mem.read(adr);
        val = i;
        val = (val | (self.res & 0x100)) >> 1;
        mem.write(adr, val);
        self.sign = val;
        self.res = ((i & 1) << 8) | self.sign;
        self.cl += c;
    }

    fn BRA(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        self.PC += signedChar(m);
        self.cl += 3;
    }

    fn LBRA(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        self.PC = (self.PC + off) & 0xFFFF;
        self.cl += 5;
    }

    fn JMPd(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        self.PC = (self.DP << 8) | m;
        self.cl += 3;
    }

    fn JMPe(&mut self, mem: &mut Memory) {
        self.PC = self.ETEND(mem);
        self.cl += 4;
    }

    fn JMPx(&mut self, mem: &mut Memory) {
        self.PC = self.INDEXE(mem);
        self.cl += 3;
    }

    fn BSR(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.PC += signedChar(m);
        self.cl += 7;
    }

    fn LBSR(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.PC = (self.PC + off) & 0xFFFF;
        self.cl += 9;
    }

    fn JSRd(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.PC = (self.DP << 8) | m;
        self.cl += 7;
    }

    fn JSRe(&mut self, mem: &mut Memory) {
        let adr = self.ETEND(mem);
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.PC = adr;
        self.cl += 8;
    }

    fn JSRx(&mut self, mem: &mut Memory) {
        let adr = self.INDEXE(mem);
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.PC = adr;
        self.cl += 7;
    }

    fn BRN(&mut self, mem: &mut Memory) {
        mem.read(self.PC);
        self.PC += 1;
        self.cl += 3;
    }

    fn LBRN(&mut self, mem: &mut Memory) {
        mem.read(self.PC);
        self.PC += 1;
        mem.read(self.PC);
        self.PC += 1;
        self.cl += 5;
    }

    const fn NOP(&mut self) {
        self.cl += 2;
    }

    fn RTS(&mut self, mem: &mut Memory) {
        self.PC = mem.read_16(self.S);
        self.S += 2;
        self.cl += 5;
    }

    /* Branchements conditionnels */

    fn BCC(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.res & 0x100) != 0x100 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBCC(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.res & 0x100) != 0x100 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BCS(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.res & 0x100) == 0x100 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBCS(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.res & 0x100) == 0x100 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BEQ(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.res & 0xff) == 0x00 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBEQ(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.res & 0xff) == 0x00 {
            self.PC = (self.PC + off) & 0xFFFF;
        }
        self.cl += 6;
    }

    fn BNE(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.res & 0xff) != 0 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBNE(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.res & 0xff) != 0 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BGE(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) == 0 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBGE(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) == 0 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BLE(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.res & 0xff) == 0)
            || (((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) != 0)
        {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBLE(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.res & 0xff) == 0)
            || (((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) != 0)
        {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BLS(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.res & 0x100) != 0) || ((self.res & 0xff) == 0) {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBLS(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.res & 0x100) != 0) || ((self.res & 0xff) == 0) {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BGT(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.res & 0xff) != 0)
            && (((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) == 0)
        {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBGT(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.res & 0xff) != 0)
            && (((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) == 0)
        {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BLT(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) != 0 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBLT(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.sign ^ ((!(self.m1 ^ self.m2)) & (self.m1 ^ self.ovfl))) & 0x80) != 0 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BHI(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if ((self.res & 0x100) == 0) && ((self.res & 0xff) != 0) {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBHI(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if ((self.res & 0x100) == 0) && ((self.res & 0xff) != 0) {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        }
        self.cl += 5;
    }

    fn BMI(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.sign & 0x80) != 0 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBMI(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.sign & 0x80) != 0 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BPL(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (self.sign & 0x80) == 0 {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBPL(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (self.sign & 0x80) == 0 {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BVS(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (((self.m1 ^ self.m2) & 0x80) == 0) && (((self.m1 ^ self.ovfl) & 0x80) != 0) {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBVS(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (((self.m1 ^ self.m2) & 0x80) == 0) && (((self.m1 ^ self.ovfl) & 0x80) != 0) {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn BVC(&mut self, mem: &mut Memory) {
        let m = mem.read(self.PC);
        self.PC += 1;
        if (((self.m1 ^ self.m2) & 0x80) != 0) || (((self.m1 ^ self.ovfl) & 0x80) == 0) {
            self.PC += signedChar(m);
        }
        self.cl += 3;
    }

    fn LBVC(&mut self, mem: &mut Memory) {
        let mut m = mem.read(self.PC);
        self.PC += 1;
        let mut off = m << 8;
        m = mem.read(self.PC);
        self.PC += 1;
        off |= m;
        if (((self.m1 ^ self.m2) & 0x80) != 0) || (((self.m1 ^ self.ovfl) & 0x80) == 0) {
            self.PC = (self.PC + off) & 0xFFFF;
            self.cl += 6;
        } else {
            self.cl += 5;
        }
    }

    fn SWI(&mut self, mem: &mut Memory) {
        self.getcc();
        self.CC |= 0x80; /* bit E � 1 */
        self.setcc(self.CC);
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.S -= 1;
        mem.write(self.S, self.U & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.U >> 8);
        self.S -= 1;
        mem.write(self.S, self.Y & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.Y >> 8);
        self.S -= 1;
        mem.write(self.S, self.X & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.X >> 8);
        self.S -= 1;
        mem.write(self.S, self.DP);
        self.S -= 1;
        mem.write(self.S, self.B);
        self.S -= 1;
        mem.write(self.S, self.A);
        self.S -= 1;
        mem.write(self.S, self.CC);

        self.PC = mem.read_16(0xFFFA);
        self.cl += 19;
    }

    fn RTI(&mut self, mem: &mut Memory) {
        self.CC = mem.read(self.S);
        self.setcc(self.CC);
        self.S += 1;
        if (self.CC & 0x80) == 0x80 {
            self.A = mem.read(self.S);
            self.S += 1;
            self.B = mem.read(self.S);
            self.S += 1;
            self.DP = mem.read(self.S);
            self.S += 1;
            self.X = mem.read_16(self.S);
            self.S += 2;
            self.Y = mem.read_16(self.S);
            self.S += 2;
            self.U = mem.read_16(self.S);
            self.S += 2;
            self.cl += 15;
        } else {
            self.cl += 6;
        }

        self.PC = mem.read_16(self.S);
        self.S += 2;
    }

    pub(crate) fn IRQ(&mut self, mem: &mut Memory) {
        /* mise � 1 du bit E sur le CC */
        self.getcc();
        self.CC |= 0x80;
        self.setcc(self.CC);
        self.S -= 1;
        mem.write(self.S, self.PC & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.PC >> 8);
        self.S -= 1;
        mem.write(self.S, self.U & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.U >> 8);
        self.S -= 1;
        mem.write(self.S, self.Y & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.Y >> 8);
        self.S -= 1;
        mem.write(self.S, self.X & 0x00FF);
        self.S -= 1;
        mem.write(self.S, self.X >> 8);
        self.S -= 1;
        mem.write(self.S, self.DP);
        self.S -= 1;
        mem.write(self.S, self.B);
        self.S -= 1;
        mem.write(self.S, self.A);
        self.S -= 1;
        mem.write(self.S, self.CC);
        self.PC = mem.read_16(0xFFF8);
        self.CC |= 0x10;
        self.setcc(self.CC);
        self.cl += 19;
    }

    const fn DAA(&mut self) {
        let mut i = self.A + (self.res & 0x100);
        if ((self.A & 15) > 9) || ((self.h1 & 15) + (self.h2 & 15) > 15) {
            i += 6;
        }
        if i > 0x99 {
            i += 0x60;
        }
        self.res = i;
        self.sign = i;
        self.A = i & 255;
        self.cl += 2;
    }

    fn CWAI(&mut self, mem: &mut Memory) {
        self.getcc();
        self.CC &= mem.read(self.PC);
        self.setcc(self.CC);
        self.PC += 1;
        self.cl += 20;
    }

    pub(crate) fn FetchUntil(
        &mut self,
        clock: int,
        mem: &mut Memory,
        screen: &mut Screen,
        sound: &mut Sound,
    ) -> int {
        while self.cl < clock {
            self.Fetch(mem, screen, sound);
        }
        self.cl -= clock;
        self.cl
    }

    fn Fetch(&mut self, mem: &mut Memory, screen: &mut Screen, sound: &mut Sound) {
        let opcode = mem.read(self.PC);
        self.PC += 1;
        // 	Sound emulation process
        if self.sound_buffer.push(mem.sound_mem) {
            sound.play_sound(self);
        }

        match opcode {
            // the mystery undocumented opcode
            0x01 => {
                self.PC += 1;
                self.cl += 2;
                // PER (instruction d'emulation de périphérique)
            }
            0x02 => mem.periph(self.PC, self.S, self.A, screen), // LDA
            0x86 => {
                let M = self.IMMED8();
                self.A = self.LD8(M, 2, mem);
            }
            0x96 => {
                let M = self.DIREC(mem);
                self.A = self.LD8(M, 4, mem);
            }
            0xB6 => {
                let M = self.ETEND(mem);
                self.A = self.LD8(M, 5, mem);
            }
            0xA6 => {
                let M = self.INDEXE(mem);
                self.A = self.LD8(M, 4, mem);
                // LDB
            }
            0xC6 => {
                let M = self.IMMED8();
                self.B = self.LD8(M, 2, mem);
            }
            0xD6 => {
                let M = self.DIREC(mem);
                self.B = self.LD8(M, 4, mem);
            }
            0xF6 => {
                let M = self.ETEND(mem);
                self.B = self.LD8(M, 5, mem);
            }
            0xE6 => {
                let M = self.INDEXE(mem);
                self.B = self.LD8(M, 4, mem);
            }
            // LDD
            0xCC => {
                let M = self.IMMED16();
                self.D = self.LD16(M, 3, mem);
                self.CALCAB();
            }
            0xDC => {
                let M = self.DIREC(mem);
                self.D = self.LD16(M, 5, mem);
                self.CALCAB();
            }
            0xFC => {
                let M = self.ETEND(mem);
                self.D = self.LD16(M, 6, mem);
                self.CALCAB();
            }
            0xEC => {
                let M = self.INDEXE(mem);
                self.D = self.LD16(M, 5, mem);
                self.CALCAB();
            }
            // LDU
            0xCE => {
                let M = self.IMMED16();
                self.U = self.LD16(M, 3, mem);
            }
            0xDE => {
                let M = self.DIREC(mem);
                self.U = self.LD16(M, 5, mem);
            }
            0xFE => {
                let M = self.ETEND(mem);
                self.U = self.LD16(M, 6, mem);
            }
            0xEE => {
                let M = self.INDEXE(mem);
                self.U = self.LD16(M, 5, mem);
            }
            // LDX
            0x8E => {
                let M = self.IMMED16();
                self.X = self.LD16(M, 3, mem);
            }
            0x9E => {
                let M = self.DIREC(mem);
                self.X = self.LD16(M, 5, mem);
            }
            0xBE => {
                let M = self.ETEND(mem);
                self.X = self.LD16(M, 6, mem);
            }
            0xAE => {
                let i = self.INDEXE(mem);
                self.X = self.LD16(i, 5, mem);
            }
            // STA
            0x97 => {
                let M = self.DIREC(mem);
                self.ST8(self.A, M, 4, mem);
            }
            0xB7 => {
                let M = self.ETEND(mem);
                self.ST8(self.A, M, 5, mem);
            }
            0xA7 => {
                let M = self.INDEXE(mem);
                self.ST8(self.A, M, 4, mem);
            }
            // STB
            0xD7 => {
                let M = self.DIREC(mem);
                self.ST8(self.B, M, 4, mem);
            }
            0xF7 => {
                let M = self.ETEND(mem);
                self.ST8(self.B, M, 5, mem);
            }
            0xE7 => {
                let M = self.INDEXE(mem);
                self.ST8(self.B, M, 4, mem);
            }
            // STD
            0xDD => {
                self.CALCD();
                let M = self.DIREC(mem);
                self.ST16(self.D, M, 5, mem);
            }
            0xFD => {
                self.CALCD();
                let M = self.ETEND(mem);
                self.ST16(self.D, M, 6, mem);
            }
            0xED => {
                self.CALCD();
                let M = self.INDEXE(mem);
                self.ST16(self.D, M, 6, mem);
            }
            // STU
            0xDF => {
                let adr = self.DIREC(mem);
                self.ST16(self.U, adr, 5, mem);
            }
            0xFF => {
                let adr = self.ETEND(mem);
                self.ST16(self.U, adr, 6, mem);
            }
            0xEF => {
                let adr = self.INDEXE(mem);
                self.ST16(self.U, adr, 5, mem);
            }
            // STX
            0x9F => {
                let M = self.DIREC(mem);
                self.ST16(self.X, M, 5, mem);
            }
            0xBF => {
                let M = self.ETEND(mem);
                self.ST16(self.X, M, 6, mem);
            }
            0xAF => {
                let M = self.INDEXE(mem);
                self.ST16(self.X, M, 5, mem);
            }
            // LEAS
            0x32 => self.S = self.INDEXE(mem),
            // LEAU
            0x33 => self.U = self.INDEXE(mem),
            // LEAX
            0x30 => self.X = self.LEA(mem),
            // LEAY
            0x31 => self.Y = self.LEA(mem),
            // CLRA
            0x4F => {
                self.A = 0;
                self.m1 = self.ovfl;
                self.sign = 0;
                self.res = 0;
                self.cl += 2;
            }
            // CLRB
            0x5F => {
                self.B = 0;
                self.m1 = self.ovfl;
                self.sign = 0;
                self.res = 0;
                self.cl += 2;
            }
            // CLR
            0x0F => {
                let M = self.DIREC(mem);
                self.CLR(M, 6, mem);
            }
            0x7F => {
                let M = self.ETEND(mem);
                self.CLR(M, 7, mem);
            }
            0x6F => {
                let M = self.INDEXE(mem);
                self.CLR(M, 6, mem);
            }
            0x1E => self.EXG(mem),
            0x1F => self.TFR(mem),
            // PSH/PUL
            0x34 => self.PSHS(mem),
            0x36 => self.PSHU(mem),
            0x35 => self.PULS(mem),
            0x37 => self.PULU(mem),
            // INC
            0x4C => self.INCA(),
            0x5C => self.INCB(),
            0x7C => {
                let M = self.ETEND(mem);
                self.INC(M, 7, mem);
            }
            0x0C => {
                let M = self.DIREC(mem);
                self.INC(M, 6, mem);
            }
            0x6C => {
                let M = self.INDEXE(mem);
                self.INC(M, 6, mem);
            }
            // DEC
            0x4A => self.DECA(),
            0x5A => self.DECB(),
            0x7A => {
                let M = self.ETEND(mem);
                self.DEC(M, 7, mem);
            }
            0x0A => {
                let M = self.DIREC(mem);
                self.DEC(M, 6, mem);
            }
            0x6A => {
                let M = self.INDEXE(mem);
                self.DEC(M, 6, mem);
            }
            // BIT
            0x85 => {
                let M = self.IMMED8();
                self.BIT(self.A, M, 2, mem);
            }
            0x95 => {
                let M = self.DIREC(mem);
                self.BIT(self.A, M, 4, mem);
            }
            0xB5 => {
                let M = self.ETEND(mem);
                self.BIT(self.A, M, 5, mem);
            }
            0xA5 => {
                let M = self.INDEXE(mem);
                self.BIT(self.A, M, 4, mem);
            }
            0xC5 => {
                let M = self.IMMED8();
                self.BIT(self.B, M, 2, mem);
            }
            0xD5 => {
                let M = self.DIREC(mem);
                self.BIT(self.B, M, 4, mem);
            }
            0xF5 => {
                let M = self.ETEND(mem);
                self.BIT(self.B, M, 5, mem);
            }
            0xE5 => {
                let M = self.INDEXE(mem);
                self.BIT(self.B, M, 4, mem);
            }
            // CMP
            0x81 => {
                let M = self.IMMED8();
                self.CMP8(self.A, M, 2, mem);
            }
            0x91 => {
                let M = self.DIREC(mem);
                self.CMP8(self.A, M, 4, mem);
            }
            0xB1 => {
                let M = self.ETEND(mem);
                self.CMP8(self.A, M, 5, mem);
            }
            0xA1 => {
                let M = self.INDEXE(mem);
                self.CMP8(self.A, M, 4, mem);
            }
            0xC1 => {
                let M = self.IMMED8();
                self.CMP8(self.B, M, 2, mem);
            }
            0xD1 => {
                let M = self.DIREC(mem);
                self.CMP8(self.B, M, 4, mem);
            }
            0xF1 => {
                let M = self.ETEND(mem);
                self.CMP8(self.B, M, 5, mem);
            }
            0xE1 => {
                let M = self.INDEXE(mem);
                self.CMP8(self.B, M, 4, mem);
            }
            0x8C => {
                let M = self.IMMED16();
                self.CMP16(self.X, M, 5, mem);
            }
            0x9C => {
                let M = self.DIREC(mem);
                self.CMP16(self.X, M, 7, mem);
            }
            0xBC => {
                let M = self.ETEND(mem);
                self.CMP16(self.X, M, 8, mem);
            }
            0xAC => {
                let M = self.INDEXE(mem);
                self.CMP16(self.X, M, 7, mem);
            }
            // TST
            0x4D => self.TSTAi(),
            0x5D => self.TSTBi(),
            0x0D => {
                let M = self.DIREC(mem);
                self.TST(M, 6, mem);
            }
            0x7D => {
                let M = self.ETEND(mem);
                self.TST(M, 7, mem);
            }
            0x6D => {
                let M = self.INDEXE(mem);
                self.TST(M, 6, mem);
            }
            // AND
            0x84 => {
                let M = self.IMMED8();
                self.ANDA(M, 2, mem);
            }
            0x94 => {
                let M = self.DIREC(mem);
                self.ANDA(M, 4, mem);
            }
            0xB4 => {
                let M = self.ETEND(mem);
                self.ANDA(M, 5, mem);
            }
            0xA4 => {
                let M = self.INDEXE(mem);
                self.ANDA(M, 4, mem);
            }
            0xC4 => {
                let M = self.IMMED8();
                self.ANDB(M, 2, mem);
            }
            0xD4 => {
                let M = self.DIREC(mem);
                self.ANDB(M, 4, mem);
            }
            0xF4 => {
                let M = self.ETEND(mem);
                self.ANDB(M, 5, mem);
            }
            0xE4 => {
                let M = self.INDEXE(mem);
                self.ANDB(M, 4, mem);
            }
            0x1C => {
                let M = self.IMMED8();
                self.ANDCC(M, 3, mem);
            }
            // OR
            0x8A => {
                let M = self.IMMED8();
                self.ORA(M, 2, mem);
            }
            0x9A => {
                let M = self.DIREC(mem);
                self.ORA(M, 4, mem);
            }
            0xBA => {
                let M = self.ETEND(mem);
                self.ORA(M, 5, mem);
            }
            0xAA => {
                let M = self.INDEXE(mem);
                self.ORA(M, 4, mem);
            }
            0xCA => {
                let M = self.IMMED8();
                self.ORB(M, 2, mem);
            }
            0xDA => {
                let M = self.DIREC(mem);
                self.ORB(M, 4, mem);
            }
            0xFA => {
                let M = self.ETEND(mem);
                self.ORB(M, 5, mem);
            }
            0xEA => {
                let M = self.INDEXE(mem);
                self.ORB(M, 4, mem);
            }
            0x1A => {
                let M = self.IMMED8();
                self.ORCC(M, 3, mem);
            }
            // EOR
            0x88 => {
                let M = self.IMMED8();
                self.EORA(M, 2, mem);
            }
            0x98 => {
                let M = self.DIREC(mem);
                self.EORA(M, 4, mem);
            }
            0xB8 => {
                let M = self.ETEND(mem);
                self.EORA(M, 5, mem);
            }
            0xA8 => {
                let M = self.INDEXE(mem);
                self.EORA(M, 4, mem);
            }
            0xC8 => {
                let M = self.IMMED8();
                self.EORB(M, 2, mem);
            }
            0xD8 => {
                let M = self.DIREC(mem);
                self.EORB(M, 4, mem);
            }
            0xF8 => {
                let M = self.ETEND(mem);
                self.EORB(M, 5, mem);
            }
            0xE8 => {
                let M = self.INDEXE(mem);
                self.EORB(M, 4, mem);
            }
            // COM
            0x43 => self.COMA(),
            0x53 => self.COMB(),
            0x03 => {
                let M = self.DIREC(mem);
                self.COM(M, 6, mem);
            }
            0x73 => {
                let M = self.ETEND(mem);
                self.COM(M, 7, mem);
            }
            0x63 => {
                let M = self.INDEXE(mem);
                self.COM(M, 6, mem);
            }
            // NEG
            0x40 => self.NEGA(),
            0x50 => self.NEGB(),
            0x00 => {
                let M = self.DIREC(mem);
                self.NEG(M, 6, mem);
            }
            0x70 => {
                let M = self.ETEND(mem);
                self.NEG(M, 7, mem);
            }
            0x60 => {
                let M = self.INDEXE(mem);
                self.NEG(M, 6, mem);
            }
            0x3A => self.ABX(),
            //ADD
            0x8B => {
                let M = self.IMMED8();
                self.ADDA(M, 2, mem);
            }
            0x9B => {
                let M = self.DIREC(mem);
                self.ADDA(M, 4, mem);
            }
            0xBB => {
                let M = self.ETEND(mem);
                self.ADDA(M, 5, mem);
            }
            0xAB => {
                let M = self.INDEXE(mem);
                self.ADDA(M, 4, mem);
            }
            0xCB => {
                let M = self.IMMED8();
                self.ADDB(M, 2, mem);
            }
            0xDB => {
                let M = self.DIREC(mem);
                self.ADDB(M, 4, mem);
            }
            0xFB => {
                let M = self.ETEND(mem);
                self.ADDB(M, 5, mem);
            }
            0xEB => {
                let M = self.INDEXE(mem);
                self.ADDB(M, 4, mem);
            }
            0xC3 => {
                let M = self.IMMED16();
                self.ADDD(M, 4, mem);
            }
            0xD3 => {
                let M = self.DIREC(mem);
                self.ADDD(M, 6, mem);
            }
            0xF3 => {
                let M = self.ETEND(mem);
                self.ADDD(M, 7, mem);
            }
            0xE3 => {
                let M = self.INDEXE(mem);
                self.ADDD(M, 6, mem);
            }
            // ADC
            0x89 => {
                let M = self.IMMED8();
                self.ADCA(M, 2, mem);
            }
            0x99 => {
                let M = self.DIREC(mem);
                self.ADCA(M, 4, mem);
            }
            0xB9 => {
                let M = self.ETEND(mem);
                self.ADCA(M, 5, mem);
            }
            0xA9 => {
                let M = self.INDEXE(mem);
                self.ADCA(M, 4, mem);
            }
            0xC9 => {
                let M = self.IMMED8();
                self.ADCB(M, 2, mem);
            }
            0xD9 => {
                let M = self.DIREC(mem);
                self.ADCB(M, 4, mem);
            }
            0xF9 => {
                let M = self.ETEND(mem);
                self.ADCB(M, 5, mem);
            }
            0xE9 => {
                let M = self.INDEXE(mem);
                self.ADCB(M, 4, mem);
            }
            0x3D => self.MUL(),
            // SBC
            0x82 => {
                let M = self.IMMED8();
                self.SBCA(M, 2, mem);
            }
            0x92 => {
                let M = self.DIREC(mem);
                self.SBCA(M, 4, mem);
            }
            0xB2 => {
                let M = self.ETEND(mem);
                self.SBCA(M, 5, mem);
            }
            0xA2 => {
                let M = self.INDEXE(mem);
                self.SBCA(M, 4, mem);
            }
            0xC2 => {
                let M = self.IMMED8();
                self.SBCB(M, 2, mem);
            }
            0xD2 => {
                let M = self.DIREC(mem);
                self.SBCB(M, 4, mem);
            }
            0xF2 => {
                let M = self.ETEND(mem);
                self.SBCB(M, 5, mem);
            }
            0xE2 => {
                let M = self.INDEXE(mem);
                self.SBCB(M, 4, mem);
            }
            //SUB
            0x80 => {
                let M = self.IMMED8();
                self.SUBA(M, 2, mem);
            }
            0x90 => {
                let M = self.DIREC(mem);
                self.SUBA(M, 4, mem);
            }
            0xB0 => {
                let M = self.ETEND(mem);
                self.SUBA(M, 5, mem);
            }
            0xA0 => {
                let M = self.INDEXE(mem);
                self.SUBA(M, 4, mem);
            }
            0xC0 => {
                let M = self.IMMED8();
                self.SUBB(M, 2, mem);
            }
            0xD0 => {
                let M = self.DIREC(mem);
                self.SUBB(M, 4, mem);
            }
            0xF0 => {
                let M = self.ETEND(mem);
                self.SUBB(M, 5, mem);
            }
            0xE0 => {
                let M = self.INDEXE(mem);
                self.SUBB(M, 4, mem);
            }
            0x83 => {
                let M = self.IMMED16();
                self.SUBD(M, 4, mem);
            }
            0x93 => {
                let M = self.DIREC(mem);
                self.SUBD(M, 6, mem);
            }
            0xB3 => {
                let M = self.ETEND(mem);
                self.SUBD(M, 7, mem);
            }
            0xA3 => {
                let M = self.INDEXE(mem);
                self.SUBD(M, 6, mem);
            }
            0x1D => self.SEX(),
            // ASL
            0x48 => self.ASLA(),
            0x58 => self.ASLB(),
            0x08 => {
                let M = self.DIREC(mem);
                self.ASL(M, 6, mem);
            }
            0x78 => {
                let M = self.ETEND(mem);
                self.ASL(M, 7, mem);
            }
            0x68 => {
                let M = self.INDEXE(mem);
                self.ASL(M, 6, mem);
            }
            // ASR
            0x47 => self.ASRA(),
            0x57 => self.ASRB(),
            0x07 => {
                let M = self.DIREC(mem);
                self.ASR(M, 6, mem);
            }
            0x77 => {
                let M = self.ETEND(mem);
                self.ASR(M, 7, mem);
            }
            0x67 => {
                let M = self.INDEXE(mem);
                self.ASR(M, 6, mem);
            }
            // LSR
            0x44 => self.LSRA(),
            0x54 => self.LSRB(),
            0x04 => {
                let M = self.DIREC(mem);
                self.LSR(M, 6, mem);
            }
            0x74 => {
                let M = self.ETEND(mem);
                self.LSR(M, 7, mem);
            }
            0x64 => {
                let M = self.INDEXE(mem);
                self.LSR(M, 6, mem);
            }
            // ROL
            0x49 => self.ROLA(),
            0x59 => self.ROLB(),
            0x09 => {
                let M = self.DIREC(mem);
                self.ROL(M, 6, mem);
            }
            0x79 => {
                let M = self.ETEND(mem);
                self.ROL(M, 7, mem);
            }
            0x69 => {
                let M = self.INDEXE(mem);
                self.ROL(M, 6, mem);
            }
            // ROR
            0x46 => self.RORA(),
            0x56 => self.RORB(),
            0x06 => {
                let M = self.DIREC(mem);
                self.ROR(M, 6, mem);
            }
            0x76 => {
                let M = self.ETEND(mem);
                self.ROR(M, 7, mem);
            }
            0x66 => {
                let M = self.INDEXE(mem);
                self.ROR(M, 6, mem);
            }
            // BRA
            0x20 => self.BRA(mem),
            0x16 => self.LBRA(mem),
            // JMP
            0x0E => self.JMPd(mem),
            0x7E => self.JMPe(mem),
            0x6E => self.JMPx(mem),
            // BSR
            0x8D => self.BSR(mem),
            0x17 => self.LBSR(mem),
            // JSR
            0x9D => self.JSRd(mem),
            0xBD => self.JSRe(mem),
            0xAD => self.JSRx(mem),
            0x12 => self.NOP(),
            0x39 => self.RTS(mem),
            // Bxx
            0x21 => self.BRN(mem),
            0x24 => self.BCC(mem),
            0x25 => self.BCS(mem),
            0x27 => self.BEQ(mem),
            0x26 => self.BNE(mem),
            0x2C => self.BGE(mem),
            0x2F => self.BLE(mem),
            0x23 => self.BLS(mem),
            0x2E => self.BGT(mem),
            0x2D => self.BLT(mem),
            0x22 => self.BHI(mem),
            0x2B => self.BMI(mem),
            0x2A => self.BPL(mem),
            0x28 => self.BVC(mem),
            0x29 => self.BVS(mem),
            0x3F => self.SWI(mem),
            0x3B => self.RTI(mem),
            0x19 => self.DAA(),
            0x3C => self.CWAI(mem),
            // extended mode
            0x10 => {
                let opcode0x10 = mem.read(self.PC);
                self.PC += 1;
                match opcode0x10 {
                    // LDS
                    0xCE => {
                        let M = self.IMMED16();
                        self.S = self.LD16(M, 3, mem);
                    }
                    0xDE => {
                        let M = self.DIREC(mem);
                        self.S = self.LD16(M, 5, mem);
                    }
                    0xFE => {
                        let M = self.ETEND(mem);
                        self.S = self.LD16(M, 6, mem);
                    }
                    0xEE => {
                        let M = self.INDEXE(mem);
                        self.S = self.LD16(M, 5, mem);
                    }
                    // LDY
                    0x8E => {
                        let M = self.IMMED16();
                        self.Y = self.LD16(M, 3, mem);
                    }
                    0x9E => {
                        let M = self.DIREC(mem);
                        self.Y = self.LD16(M, 5, mem);
                    }
                    0xBE => {
                        let M = self.ETEND(mem);
                        self.Y = self.LD16(M, 6, mem);
                    }
                    0xAE => {
                        let M = self.INDEXE(mem);
                        self.Y = self.LD16(M, 5, mem);
                    }
                    // STS
                    0xDF => {
                        let M = self.DIREC(mem);
                        self.ST16(self.S, M, 5, mem);
                    }
                    0xFF => {
                        let M = self.ETEND(mem);
                        self.ST16(self.S, M, 6, mem);
                    }
                    0xEF => {
                        let M = self.INDEXE(mem);
                        self.ST16(self.S, M, 5, mem);
                    }
                    // STY
                    0x9F => {
                        let M = self.DIREC(mem);
                        self.ST16(self.Y, M, 5, mem);
                    }
                    0xBF => {
                        let M = self.ETEND(mem);
                        self.ST16(self.Y, M, 6, mem);
                    }
                    0xAF => {
                        let M = self.INDEXE(mem);
                        self.ST16(self.Y, M, 5, mem);
                    }
                    // CMP
                    0x83 => {
                        self.CALCD();
                        let M = self.IMMED16();
                        self.CMP16(self.D, M, 5, mem);
                    }
                    0x93 => {
                        self.CALCD();
                        let M = self.DIREC(mem);
                        self.CMP16(self.D, M, 7, mem);
                    }
                    0xB3 => {
                        self.CALCD();
                        let M = self.ETEND(mem);
                        self.CMP16(self.D, M, 8, mem);
                    }
                    0xA3 => {
                        self.CALCD();
                        let M = self.INDEXE(mem);
                        self.CMP16(self.D, M, 7, mem);
                    }
                    0x8C => {
                        let M = self.IMMED16();
                        self.CMP16(self.Y, M, 5, mem);
                    }
                    0x9C => {
                        let M = self.DIREC(mem);
                        self.CMP16(self.Y, M, 7, mem);
                    }
                    0xBC => {
                        let M = self.ETEND(mem);
                        self.CMP16(self.Y, M, 8, mem);
                    }
                    0xAC => {
                        let M = self.INDEXE(mem);
                        self.CMP16(self.Y, M, 7, mem);
                    }
                    // Bxx
                    0x21 => self.LBRN(mem),
                    0x24 => self.LBCC(mem),
                    0x25 => self.LBCS(mem),
                    0x27 => self.LBEQ(mem),
                    0x26 => self.LBNE(mem),
                    0x2C => self.LBGE(mem),
                    0x2F => self.LBLE(mem),
                    0x23 => self.LBLS(mem),
                    0x2E => self.LBGT(mem),
                    0x2D => self.LBLT(mem),
                    0x22 => self.LBHI(mem),
                    0x2B => self.LBMI(mem),
                    0x2A => self.LBPL(mem),
                    0x28 => self.LBVC(mem),
                    0x29 => self.LBVS(mem),
                    _ => {
                        eprintln!("opcode 10 {opcode0x10:02X} not implemented");
                        eprintln!("{}", self.print_state());
                    }
                } // of case opcode0x10
            }
            0x11 => {
                let opcode0x11 = mem.read(self.PC);
                self.PC += 1;

                match opcode0x11 {
                    // CMP
                    0x8C => {
                        let M = self.IMMED16();
                        self.CMP16(self.S, M, 5, mem);
                    }
                    0x9C => {
                        let M = self.DIREC(mem);
                        self.CMP16(self.S, M, 7, mem);
                    }
                    0xBC => {
                        let M = self.ETEND(mem);
                        self.CMP16(self.S, M, 8, mem);
                    }
                    0xAC => {
                        let M = self.INDEXE(mem);
                        self.CMP16(self.S, M, 7, mem);
                    }
                    0x83 => {
                        let M = self.IMMED16();
                        self.CMP16(self.U, M, 5, mem);
                    }
                    0x93 => {
                        let M = self.DIREC(mem);
                        self.CMP16(self.U, M, 7, mem);
                    }
                    0xB3 => {
                        let M = self.ETEND(mem);
                        self.CMP16(self.U, M, 8, mem);
                    }
                    0xA3 => {
                        let M = self.INDEXE(mem);
                        self.CMP16(self.U, M, 7, mem);
                    }
                    _ => {
                        eprintln!("opcode 11{opcode0x11:02X} not implemented");
                        eprintln!("{}", self.print_state());
                    }
                } // of case opcode 0x11
            }
            _ => {
                eprintln!("opcode {opcode:02X} not implemented");
                eprintln!("{}", self.print_state());
            }
        } // of case opcode
    } // of method fetch()

    // UNASSEMBLE/DEBUG PART
    pub(crate) fn print_state(&mut self) -> String {
        self.CC = self.getcc();
        let s = format!(
            "A={:02X} B={:02X} X={:04X} Y={:04X}\nPC={:04X} DP={:02X} U={:04X} S={:04X} CC={:02X}",
            self.A, self.B, self.X, self.Y, self.PC, self.DP, self.U, self.S, self.CC
        );
        s
    }
}

// force sign extension in a portable but ugly maneer
const fn signedChar(v: int) -> int {
    if (v & 0x80) == 0 {
        return v & 0xFF;
    }
    let mut delta = -1; // delta is 0xFFFF.... independently of 32/64bits
    delta = (delta >> 8) << 8; // force last 8bits to 0
    (v & 0xFF) | delta // result is now signed
}

// force sign extension in a portable but ugly maneer
const fn signed16bits(v: int) -> int {
    if (v & 0x8000) == 0 {
        return v & 0xFFFF;
    }
    let mut delta = -1; // delta is 0xFFFF.... independently of 32/64bits
    delta = (delta >> 16) << 16; // force last 16bits to 0
    (v & 0xFFFF) | delta // result is now signed
}

const fn regx(m: int) -> &'static str {
    const MASK: i32 = 0x60;
    match m & MASK {
        0x00 => "?X",
        0x20 => "?X",
        0x40 => "?U",
        0x60 => "?S",
        _ => "?",
    }
}

fn r_tfr(m: int) -> String {
    let mut output = String::with_capacity(5);
    match m & 0xF0 {
        0x80 => output.push_str("A,"),
        0x90 => output.push_str("B,"),
        0xA0 => output.push_str("CC,"),
        0x00 => output.push_str("D,"),
        0xB0 => output.push_str("DP,"),
        0x50 => output.push_str("PC,"),
        0x40 => output.push_str("S,"),
        0x30 => output.push_str("U,"),
        0x10 => output.push_str("X,"),
        0x20 => output.push_str("Y,"),
        _ => {}
    };
    match m & 0x0F {
        0x8 => output.push('A'),
        0x9 => output.push('B'),
        0xA => output.push_str("CC"),
        0x0 => output.push('D'),
        0xB => output.push_str("DP"),
        0x5 => output.push_str("PC"),
        0x4 => output.push('S'),
        0x3 => output.push('U'),
        0x1 => output.push('X'),
        0x2 => output.push('Y'),
        _ => {}
    }
    output
}

fn r_pile(m: int) -> String {
    let mut output = String::new();
    if (m & 0x80) != 0 {
        output.push_str("PC,");
    }
    if (m & 0x40) != 0 {
        output.push_str("U/S,");
    }
    if (m & 0x20) != 0 {
        output.push_str("Y,");
    }
    if (m & 0x10) != 0 {
        output.push_str("X,");
    }
    if (m & 0x08) != 0 {
        output.push_str("DP,");
    }
    if (m & 0x04) != 0 {
        output.push_str("B,");
    }
    if (m & 0x02) != 0 {
        output.push_str("A,");
    }
    if (m & 0x01) != 0 {
        output.push_str("CC");
    }
    output
}

pub(crate) fn unassemble(start: int, maxLines: int, mem: &Memory) -> String {
    let mut MNEMO = ["ILL -"; 256];
    let mut MNEMO10 = ["ILL -"; 256];
    let mut MNEMO11 = ["ILL -"; 256];

    /* LDA opcode */
    MNEMO[0x86] = "LDA i";
    MNEMO[0x96] = "LDA d";
    MNEMO[0xB6] = "LDA e";
    MNEMO[0xA6] = "LDA x";

    /* LDB opcode */
    MNEMO[0xC6] = "LDB i";
    MNEMO[0xD6] = "LDB d";
    MNEMO[0xF6] = "LDB e";
    MNEMO[0xE6] = "LDB x";

    /* LDD opcode */
    MNEMO[0xCC] = "LDD I";
    MNEMO[0xDC] = "LDD d";
    MNEMO[0xFC] = "LDD e";
    MNEMO[0xEC] = "LDD x";

    /* LDU opcode */
    MNEMO[0xCE] = "LDU I";
    MNEMO[0xDE] = "LDU d";
    MNEMO[0xFE] = "LDU e";
    MNEMO[0xEE] = "LDU x";

    /* LDX opcode */
    MNEMO[0x8E] = "LDX I";
    MNEMO[0x9E] = "LDX d";
    MNEMO[0xBE] = "LDX e";
    MNEMO[0xAE] = "LDX x";

    /* LDS opcode */
    MNEMO10[0xCE] = "LDS I";
    MNEMO10[0xDE] = "LDS d";
    MNEMO10[0xFE] = "LDS e";
    MNEMO10[0xEE] = "LDS x";

    /* LDY opcode */
    MNEMO10[0x8E] = "LDY I";
    MNEMO10[0x9E] = "LDY d";
    MNEMO10[0xBE] = "LDY e";
    MNEMO10[0xAE] = "LDY x";

    /* STA opcode */
    MNEMO[0x97] = "STA d";
    MNEMO[0xB7] = "STA e";
    MNEMO[0xA7] = "STA x";

    /* STB opcode */
    MNEMO[0xD7] = "STB d";
    MNEMO[0xF7] = "STB e";
    MNEMO[0xE7] = "STB x";

    /* STD opcode */
    MNEMO[0xDD] = "STD d";
    MNEMO[0xFD] = "STD e";
    MNEMO[0xED] = "STD x";

    /* STS opcode */
    MNEMO10[0xDF] = "STS d";
    MNEMO10[0xFF] = "STS e";
    MNEMO10[0xEF] = "STS x";

    /* STU opcode */
    MNEMO[0xDF] = "STU d";
    MNEMO[0xFF] = "STU e";
    MNEMO[0xEF] = "STU x";

    /* STX opcode */
    MNEMO[0x9F] = "STX d";
    MNEMO[0xBF] = "STX e";
    MNEMO[0xAF] = "STX x";

    /* STY opcode */
    MNEMO10[0x9F] = "STY d";
    MNEMO10[0xBF] = "STY e";
    MNEMO10[0xAF] = "STY x";

    /* LEA opcode */
    MNEMO[0x32] = "LEASx";
    MNEMO[0x33] = "LEAUx";
    MNEMO[0x30] = "LEAXx";
    MNEMO[0x31] = "LEAYx";

    /* CLR opcode */
    MNEMO[0x0F] = "CLR d";
    MNEMO[0x7F] = "CLR e";
    MNEMO[0x6F] = "CLR x";
    MNEMO[0x4F] = "CLRA-";
    MNEMO[0x5F] = "CLRB-";

    /* EXG */
    MNEMO[0x1E] = "EXG r";

    /* TFR */
    MNEMO[0x1F] = "TFR r";

    /* PSH */
    MNEMO[0x34] = "PSHSR";
    MNEMO[0x36] = "PSHUR";

    /* PUL */
    MNEMO[0x35] = "PULSR";
    MNEMO[0x37] = "PULUR";

    /* INC */
    MNEMO[0x4C] = "INCA-";
    MNEMO[0x5C] = "INCB-";
    MNEMO[0x7C] = "INC e";
    MNEMO[0x0C] = "INC d";
    MNEMO[0x6C] = "INC x";

    /* DEC */
    MNEMO[0x4A] = "DECA-";
    MNEMO[0x5A] = "DECB-";
    MNEMO[0x7A] = "DEC e";
    MNEMO[0x0A] = "DEC d";
    MNEMO[0x6A] = "DEC x";

    /* BIT */
    MNEMO[0x85] = "BITAi";
    MNEMO[0x95] = "BITAd";
    MNEMO[0xB5] = "BITAe";
    MNEMO[0xA5] = "BITAx";
    MNEMO[0xC5] = "BITBi";
    MNEMO[0xD5] = "BITBd";
    MNEMO[0xF5] = "BITBe";
    MNEMO[0xE5] = "BITBx";

    /* CMP */
    MNEMO[0x81] = "CMPAi";
    MNEMO[0x91] = "CMPAd";
    MNEMO[0xB1] = "CMPAe";
    MNEMO[0xA1] = "CMPAx";
    MNEMO[0xC1] = "CMPBi";
    MNEMO[0xD1] = "CMPBd";
    MNEMO[0xF1] = "CMPBe";
    MNEMO[0xE1] = "CMPBx";
    MNEMO10[0x83] = "CMPDI";
    MNEMO10[0x93] = "CMPDd";
    MNEMO10[0xB3] = "CMPDe";
    MNEMO10[0xA3] = "CMPDx";
    MNEMO11[0x8C] = "CMPSI";
    MNEMO11[0x9C] = "CMPSd";
    MNEMO11[0xBC] = "CMPSe";
    MNEMO11[0xAC] = "CMPSx";
    MNEMO11[0x83] = "CMPUI";
    MNEMO11[0x93] = "CMPUd";
    MNEMO11[0xB3] = "CMPUe";
    MNEMO11[0xA3] = "CMPUx";
    MNEMO[0x8C] = "CMPXI";
    MNEMO[0x9C] = "CMPXd";
    MNEMO[0xBC] = "CMPXe";
    MNEMO[0xAC] = "CMPXx";
    MNEMO10[0x8C] = "CMPYI";
    MNEMO10[0x9C] = "CMPYd";
    MNEMO10[0xBC] = "CMPYe";
    MNEMO10[0xAC] = "CMPYx";

    /* TST */
    MNEMO[0x4D] = "TSTA-";
    MNEMO[0x5D] = "TSTB-";
    MNEMO[0x0D] = "TST d";
    MNEMO[0x7D] = "TST e";
    MNEMO[0x6D] = "TST x";

    /* AND */
    MNEMO[0x84] = "ANDAi";
    MNEMO[0x94] = "ANDAd";
    MNEMO[0xB4] = "ANDAe";
    MNEMO[0xA4] = "ANDAx";
    MNEMO[0xC4] = "ANDBi";
    MNEMO[0xD4] = "ANDBd";
    MNEMO[0xF4] = "ANDBe";
    MNEMO[0xE4] = "ANDBx";
    MNEMO[0x1C] = "& CCi";

    /* OR */
    MNEMO[0x8A] = "ORA i";
    MNEMO[0x9A] = "ORA d";
    MNEMO[0xBA] = "ORA e";
    MNEMO[0xAA] = "ORA x";
    MNEMO[0xCA] = "ORB i";
    MNEMO[0xDA] = "ORB d";
    MNEMO[0xFA] = "ORB e";
    MNEMO[0xEA] = "ORB x";
    MNEMO[0x1A] = "ORCCi";

    /* EOR */
    MNEMO[0x88] = "EORAi";
    MNEMO[0x98] = "EORAd";
    MNEMO[0xB8] = "EORAe";
    MNEMO[0xA8] = "EORAx";
    MNEMO[0xC8] = "EORBi";
    MNEMO[0xD8] = "EORBd";
    MNEMO[0xF8] = "EORBe";
    MNEMO[0xE8] = "EORBx";

    /* COM */
    MNEMO[0x03] = "COM d";
    MNEMO[0x73] = "COM e";
    MNEMO[0x63] = "COM x";
    MNEMO[0x43] = "COMA-";
    MNEMO[0x53] = "COMB-";

    /* NEG */
    MNEMO[0x00] = "NEG d";
    MNEMO[0x70] = "NEG e";
    MNEMO[0x60] = "NEG x";
    MNEMO[0x40] = "NEGA-";
    MNEMO[0x50] = "NEGB-";

    /* ABX */
    MNEMO[0x3A] = "ABX -";

    /* ADC */
    MNEMO[0x89] = "ADCAi";
    MNEMO[0x99] = "ADCAd";
    MNEMO[0xB9] = "ADCAe";
    MNEMO[0xA9] = "ADCAx";
    MNEMO[0xC9] = "ADCBi";
    MNEMO[0xD9] = "ADCBd";
    MNEMO[0xF9] = "ADCBe";
    MNEMO[0xE9] = "ADCBx";

    /* ADD */
    MNEMO[0x8B] = "ADDAi";
    MNEMO[0x9B] = "ADDAd";
    MNEMO[0xBB] = "ADDAe";
    MNEMO[0xAB] = "ADDAx";
    MNEMO[0xCB] = "ADDBi";
    MNEMO[0xDB] = "ADDBd";
    MNEMO[0xFB] = "ADDBe";
    MNEMO[0xEB] = "ADDBx";
    MNEMO[0xC3] = "ADDDI";
    MNEMO[0xD3] = "ADDDd";
    MNEMO[0xF3] = "ADDDe";
    MNEMO[0xE3] = "ADDDx";

    /* MUL */
    MNEMO[0x3D] = "MUL -";

    /* SBC */
    MNEMO[0x82] = "SBCAi";
    MNEMO[0x92] = "SBCAd";
    MNEMO[0xB2] = "SBCAe";
    MNEMO[0xA2] = "SBCAx";
    MNEMO[0xC2] = "SBCBi";
    MNEMO[0xD2] = "SBCBd";
    MNEMO[0xF2] = "SBCBe";
    MNEMO[0xE2] = "SBCBx";

    /* SUB */
    MNEMO[0x80] = "SUBAi";
    MNEMO[0x90] = "SUBAd";
    MNEMO[0xB0] = "SUBAe";
    MNEMO[0xA0] = "SUBAx";
    MNEMO[0xC0] = "SUBBi";
    MNEMO[0xD0] = "SUBBd";
    MNEMO[0xF0] = "SUBBe";
    MNEMO[0xE0] = "SUBBx";
    MNEMO[0x83] = "SUBDI";
    MNEMO[0x93] = "SUBDd";
    MNEMO[0xB3] = "SUBDe";
    MNEMO[0xA3] = "SUBDx";

    /* SEX */
    MNEMO[0x1D] = "SEX -";

    /* ASL */
    MNEMO[0x08] = "ASL d";
    MNEMO[0x78] = "ASL e";
    MNEMO[0x68] = "ASL x";
    MNEMO[0x48] = "ASLA-";
    MNEMO[0x58] = "ASLB-";

    /* ASR */
    MNEMO[0x07] = "ASR d";
    MNEMO[0x77] = "ASR e";
    MNEMO[0x67] = "ASR x";
    MNEMO[0x47] = "ASRA-";
    MNEMO[0x57] = "ASRB-";

    /* LSR */
    MNEMO[0x04] = "LSR d";
    MNEMO[0x74] = "LSR e";
    MNEMO[0x64] = "LSR x";
    MNEMO[0x44] = "LSRA-";
    MNEMO[0x54] = "LSRB-";

    /* ROL */
    MNEMO[0x09] = "ROL d";
    MNEMO[0x79] = "ROL e";
    MNEMO[0x69] = "ROL x";
    MNEMO[0x49] = "ROLA-";
    MNEMO[0x59] = "ROLB-";

    /* ROR */
    MNEMO[0x06] = "ROR d";
    MNEMO[0x76] = "ROR e";
    MNEMO[0x66] = "ROR x";
    MNEMO[0x46] = "RORA-";
    MNEMO[0x56] = "RORB-";

    /* BRA */
    MNEMO[0x20] = "BRA o";
    MNEMO[0x16] = "LBRAO";

    /* JMP */
    MNEMO[0x0E] = "JMP d";
    MNEMO[0x7E] = "JMP e";
    MNEMO[0x6E] = "JMP x";

    /* BSR */
    MNEMO[0x8D] = "BSR o";
    MNEMO[0x17] = "LBSRO";

    /* JSR */
    MNEMO[0x9D] = "JSR d";
    MNEMO[0xBD] = "JSR e";
    MNEMO[0xAD] = "JSR x";

    /* BRN */
    MNEMO[0x21] = "BRN o";
    MNEMO10[0x21] = "LBRNO";

    /* NOP */
    MNEMO[0x12] = "NOP -";

    /* RTS */
    MNEMO[0x39] = "RTS -";

    /* BCC */
    MNEMO[0x24] = "BCC o";
    MNEMO10[0x24] = "LBCCO";

    /* BCS */
    MNEMO[0x25] = "BCS o";
    MNEMO10[0x25] = "LBCSO";

    /* BEQ */
    MNEMO[0x27] = "BEQ o";
    MNEMO10[0x27] = "LBEQO";

    /* BNE */
    MNEMO[0x26] = "BNE o";
    MNEMO10[0x26] = "LBNEO";

    /* BGE */
    MNEMO[0x2C] = "BGE o";
    MNEMO10[0x2C] = "LBGEO";

    /* BLE */
    MNEMO[0x2F] = "BLE o";
    MNEMO10[0x2F] = "LBLEO";

    /* BLS */
    MNEMO[0x23] = "BLS o";
    MNEMO10[0x23] = "LBLSO";

    /* BGT */
    MNEMO[0x2E] = "BGT o";
    MNEMO10[0x2E] = "LBGTO";

    /* BLT */
    MNEMO[0x2D] = "BLT o";
    MNEMO10[0x2D] = "LBLTO";

    /* BHI */
    MNEMO[0x22] = "BHI o";
    MNEMO10[0x22] = "LBHIO";

    /* BMI */
    MNEMO[0x2B] = "BMI o";
    MNEMO10[0x2B] = "LBMIO";

    /* BPL */
    MNEMO[0x2A] = "BPL o";
    MNEMO10[0x2A] = "LBPLO";

    /* BVC */
    MNEMO[0x28] = "BVC o";
    MNEMO10[0x28] = "LBVCO";

    /* BVS */
    MNEMO[0x29] = "BVS o";
    MNEMO10[0x29] = "LBVSO";

    /* SWI1&3 */
    MNEMO[0x3F] = "SWI i";
    MNEMO11[0x3F] = "SWI3-";

    /* RTI */
    MNEMO[0x3B] = "RTI -";

    let mut _where = start;

    let mut output = String::new();
    for _ in 0..maxLines {
        let mut mm = mem.read(_where);
        _where += 1;

        let mut output1 = format!("{:04X}.{:02X} ", _where - 1, mm);
        let mut output2 = String::new();

        let mnemo;
        if mm == 0x10 {
            mm = mem.read(_where);
            _where += 1;
            mnemo = MNEMO10[mm as usize];
            output1.push_str(&format!("{mm:02X} "));
            output2.push_str(&mnemo[0..4]);
            output2.push(' ');
        } else if mm == 0x11 {
            mm = mem.read(_where);
            _where += 1;
            mnemo = MNEMO11[mm as usize];
            output1.push_str(&format!("{mm:02X} "));
            output2.push_str(&mnemo[0..4]);
            output2.push(' ');
        } else {
            mnemo = MNEMO[mm as usize];
            output2.push_str(&mnemo[0..4]);
            output2.push(' ');
        }
        match mnemo.chars().nth(4).unwrap() {
            'I' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("#x{mm:02X}"));
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("{mm:02X}"));
            }
            'i' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("#x{mm:02X}"));
            }
            'e' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("x{mm:02X}"));
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("{mm:02X}"));
            }
            'd' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!("x{mm:02X}"));
            }
            'o' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&format!(
                    "{} (x{:04X})",
                    signedChar(mm),
                    (_where + signedChar(mm)) & 0xFFFF
                ));
            }
            'O' => {
                mm = mem.read(_where) << 8;
                _where += 1;
                mm |= mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:04X} "));
                output2.push_str(&format!(
                    "{} (=x{:04X})",
                    signed16bits(mm),
                    (_where + signed16bits(mm)) & 0xFFFF
                ));
            }
            'x' => {
                let mmx = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                if (mmx & 0x80) == 0 {
                    if (mmx & 0x10) != 0 {
                        output2.push('-');
                    }
                    output2.push_str(&format!("{},{}", mmx & 0x0F, regx(mmx)));
                } else {
                    match mmx & 0x1F {
                        0x04 => {
                            output2.push_str(&format!(",{}", regx(mmx)));
                            break;
                        }
                        0x14 => {
                            output2.push_str(&format!("[,{}]", regx(mmx)));
                            break;
                        }
                        0x08 => {
                            mm = mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:02X} "));
                            output2.push_str(&format!("{},{}", signedChar(mm), regx(mmx)));
                            break;
                        }
                        0x18 => {
                            mm = mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:02X} "));
                            output2.push_str(&format!("[{},{}]", signedChar(mm), regx(mmx)));
                            break;
                        }
                        0x09 => {
                            mm = mem.read(_where) << 8;
                            _where += 1;
                            mm |= mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:04X} "));
                            output2.push_str(&format!("{},{}", signed16bits(mm), regx(mmx)));
                            break;
                        }
                        0x19 => {
                            mm = mem.read(_where) << 8;
                            _where += 1;
                            mm |= mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:04X} "));
                            output2.push_str(&format!("[{},{}]", signed16bits(mm), regx(mmx)));
                            break;
                        }
                        0x06 => {
                            output2.push_str(&format!("A,{}", regx(mmx)));
                            break;
                        }
                        0x16 => {
                            output2.push_str(&format!("[A,{}]", regx(mmx)));
                            break;
                        }
                        0x05 => {
                            output2.push_str(&format!("B,{}", regx(mmx)));
                            break;
                        }
                        0x15 => {
                            output2.push_str(&format!("[B,{}]", regx(mmx)));
                            break;
                        }
                        0x0B => {
                            output2.push_str(&format!("D,{}", regx(mmx)));
                            break;
                        }
                        0x1B => {
                            output2.push_str(&format!("[D,{}]", regx(mmx)));
                            break;
                        }
                        0x00 => {
                            output2.push_str(&format!(",{}+", regx(mmx)));
                            break;
                        }
                        0x01 => {
                            output2.push_str(&format!(",{}++", regx(mmx)));
                            break;
                        }
                        0x11 => {
                            output2.push_str(&format!("[,{}++", regx(mmx)));
                            break;
                        }
                        0x02 => {
                            output2.push_str(&format!(",-{}", regx(mmx)));
                            break;
                        }
                        0x03 => {
                            output2.push_str(&format!(",--{}", regx(mmx)));
                            break;
                        }
                        0x13 => {
                            output2.push_str(&format!("[,--{}]", regx(mmx)));
                            break;
                        }
                        0x0C => {
                            mm = mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:02X} "));
                            output2.push_str(&format!("{},PC", signedChar(mm)));
                            break;
                        }
                        0x1C => {
                            mm = mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:02X} "));
                            output2.push_str(&format!("[{},PC]", signedChar(mm)));
                            break;
                        }
                        0x0D => {
                            mm = mem.read(_where) << 8;
                            _where += 1;
                            mm |= mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:04X} "));
                            output2.push_str(&format!("{},PC]", signed16bits(mm)));
                            break;
                        }
                        0x1D => {
                            mm = mem.read(_where) << 8;
                            _where += 1;
                            mm |= mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:04X} "));
                            output2.push_str(&format!("[{},PC]", signed16bits(mm)));
                            break;
                        }
                        0x1F => {
                            mm = mem.read(_where) << 8;
                            _where += 1;
                            mm |= mem.read(_where);
                            _where += 1;
                            output1.push_str(&format!("{mm:02X} {mm:04X} "));
                            output2.push_str(&format!("[x{mm:04X}]"));
                            break;
                        }
                        _ => output2.push_str("Illegal !"),
                    }
                }
            }
            'r' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&r_tfr(mm));
            }
            'R' => {
                mm = mem.read(_where);
                _where += 1;
                output1.push_str(&format!("{mm:02X} "));
                output2.push_str(&r_pile(mm));
            }
            _ => {}
        }

        let lll = output1.len();
        for _ in 0..32 - lll {
            output1.push(' ');
        }
        output.push_str(&output1);
        output.push_str(&output2);
        output.push('\n');
    } // of for ... maxLines
    output
}

#[derive(Debug)]
pub(crate) struct SoundBuffer {
    buffer: [u8; SOUND_SIZE],
    pos: usize,
}

impl Default for SoundBuffer {
    fn default() -> Self {
        Self {
            buffer: [0; SOUND_SIZE],
            pos: 0,
        }
    }
}

impl SoundBuffer {
    const fn push(&mut self, value: u8) -> bool {
        self.buffer[self.pos] = value;
        self.pos = (self.pos + 1) % SOUND_SIZE;
        self.pos == 0
    }
}

impl Index<usize> for SoundBuffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        self.buffer.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0)]
    // Valeurs positives dans les limites de 16 bits signés
    #[case(1, 1)]
    #[case(127, 127)]
    #[case(32767, 32767)] // 2^15 - 1, valeur max positive
    // Valeurs négatives représentées en 16 bits signés
    #[case(0xFFFF, -1)] // Tous les bits à 1 = -1 en complément à 2
    #[case(0xFFFE, -2)]
    #[case(0xFF00, -256)]
    #[case(0x8000, -32768)] // 2^15, valeur min négative
    fn test_signed16bits_zero(#[case] input: u16, #[case] expected: i16) {
        // Le zéro devrait rester zéro après conversion
        assert_eq!(signed16bits(input as int), expected.into());
    }

    #[test]
    fn test_indexe_5bit_offset() {
        let mem_val = Memory::default();
        let mut mem = mem_val;
        let mut cpu = M6809::new(&mem);

        // Test X + 5 (0x05: 000 00101 -> X, +5)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x05);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x1005);
        assert_eq!(cpu.PC, 0x2001);

        // Test Y - 1 (0x3F: 001 11111 -> Y, -1 because 0x1F is -1 in 5-bit signed)
        // 5-bit signed: 0x10 is -16, 0x1F is -1
        cpu.Y = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x3F);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x0FFF);

        // Test U + 15 (0x4F: 010 01111 -> U, +15)
        cpu.U = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x4F);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x100F);

        // Test S - 16 (0x70: 011 10000 -> S, -16)
        cpu.S = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x70);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x0FF0);
    }

    #[test]
    fn test_indexe_auto_inc_dec() {
        let mut mem = Memory::default();
        let mut cpu = M6809::new(&mem);

        // Post-increment X by 1 (0x80)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x80);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x1000);
        assert_eq!(cpu.X, 0x1001);

        // Post-increment X by 2 (0x81)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x81);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x1000);
        assert_eq!(cpu.X, 0x1002);

        // Pre-decrement X by 1 (0x82)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x82);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x0FFF);
        assert_eq!(cpu.X, 0x0FFF);

        // Pre-decrement X by 2 (0x83)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x83);
        let addr = cpu.INDEXE(&mut mem);
        assert_eq!(addr, 0x0FFE);
        assert_eq!(cpu.X, 0x0FFE);
    }

    #[test]
    fn test_indexe_accumulator_offset() {
        let mut mem = Memory::default();
        let mut cpu = M6809::new(&mem);

        // B offset (0x85)
        cpu.X = 0x1000;
        cpu.B = 0x05;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x85);
        assert_eq!(cpu.INDEXE(&mut mem), 0x1005);

        // A offset (0x86)
        cpu.X = 0x1000;
        cpu.A = 0xFF; // -1
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x86);
        assert_eq!(cpu.INDEXE(&mut mem), 0x0FFF);

        // D offset (0x8B)
        cpu.X = 0x1000;
        cpu.A = 0x00;
        cpu.B = 0x10;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x8B);
        assert_eq!(cpu.INDEXE(&mut mem), 0x1010);
    }

    #[test]
    fn test_indexe_constant_offset() {
        let mut mem = Memory::default();
        let mut cpu = M6809::new(&mem);

        // 8-bit offset (0x88)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x88);
        mem.write(0x2001, 0x12);
        assert_eq!(cpu.INDEXE(&mut mem), 0x1012);
        assert_eq!(cpu.PC, 0x2002);

        // 16-bit offset (0x89)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x89);
        mem.write(0x2001, 0x12);
        mem.write(0x2002, 0x34);
        assert_eq!(cpu.INDEXE(&mut mem), 0x1000 + 0x1234);
        assert_eq!(cpu.PC, 0x2003);
    }

    #[test]
    fn test_indexe_pc_relative() {
        let mut mem = Memory::default();
        let mut cpu = M6809::new(&mem);

        // PC 8-bit offset (0x8C)
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x8C);
        mem.write(0x2001, 0x05); // Offset +5 from PC after reading offset byte
        // After reading 0x8C, PC is 0x2001.
        // Inside INDEXE for 0x8C: m = mem.read(0x2001) [0x05], PC = 0x2002.
        // Result is PC + signedChar(m) = 0x2002 + 5 = 0x2007.
        assert_eq!(cpu.INDEXE(&mut mem), 0x2007);
        assert_eq!(cpu.PC, 0x2002);

        // PC 16-bit offset (0x8D)
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x8D);
        mem.write(0x2001, 0x10);
        mem.write(0x2002, 0x00);
        // After reading 0x8D, PC is 0x2001.
        // Inside INDEXE for 0x8D: M = read_16(0x2001) [0x1000], PC = 0x2003.
        // Result is PC + signed16bits(M) = 0x2003 + 0x1000 = 0x3003.
        assert_eq!(cpu.INDEXE(&mut mem), 0x3003);
        assert_eq!(cpu.PC, 0x2003);
    }

    #[test]
    fn test_indexe_indirect() {
        let mut mem = Memory::default();
        let mut cpu = M6809::new(&mem);

        // Indirect with 8-bit offset from X (0x98)
        cpu.X = 0x1000;
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x98);
        mem.write(0x2001, 0x02); // Offset 2 -> 0x1002
        mem.write(0x1002, 0xAB);
        mem.write(0x1003, 0xCD); // [0x1002] contains 0xABCD
        assert_eq!(cpu.INDEXE(&mut mem), 0xABCD);

        // Indirect Extended (0x9F)
        cpu.PC = 0x2000;
        mem.write(0x2000, 0x9F);
        mem.write(0x2001, 0x30);
        mem.write(0x2002, 0x00); // Address 0x3000
        mem.write(0x3000, 0xCA);
        mem.write(0x3001, 0xFE); // [0x3000] contains 0xCAFE
        assert_eq!(cpu.INDEXE(&mut mem), 0xCAFE);
    }
}
