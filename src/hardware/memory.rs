#![allow(non_snake_case)]

use std::fs::File;
use std::io::{BufWriter, Write};

use crate::hardware::k7::K7;
use crate::hardware::screen::Screen;
use crate::{bios, int};
use chrono::Local;
use log::{debug, info};

#[derive(Debug)]
pub(crate) struct Memory {
    // Lightpen parameters
    pub(crate) light_pen_clic: bool,
    pub(crate) light_pen_x: int,
    pub(crate) light_pen_y: int,

    // 0 1 			POINT 	2
    // 2 3 			COLOR 	2
    // 4 5 6 7   	RAM1 	4
    // 8 9 10 11 	RAM2 	4
    // 12			LINEA 	1
    // 13 			LINEB 	1
    // 14 15 16 17 	ROM 	4
    mem: Vec<Vec<int>>,
    mapper: [int; 16],
    key: Vec<bool>,
    dirty: Vec<bool>,

    /* Registres du 6821 */
    ORA: int,
    ORB: int,
    DDRA: int,
    DDRB: int,
    CRA: int,
    pub(crate) CRB: int,
    pub(crate) sound_mem: u8,

    /* Registre du Gate Array */
    GA0: int,
    GA1: int,
    GA2: int,
    pub(crate) GA3: int,

    k7_bit: u8,
    k7_char: u8,

    k7_in: Option<K7>,
    k7_out: Option<BufWriter<File>>,
    is_file_opened_out: bool,
    k7_out_name: Option<String>,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            light_pen_clic: false,
            light_pen_x: 0,
            light_pen_y: 0,
            mem: vec![vec![0; 0x1000]; 18],
            mapper: [0, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17],
            key: vec![false; 256],
            dirty: vec![false; 200],
            ORA: 0,
            ORB: 0,
            DDRA: 0,
            DDRB: 0,
            CRA: 0,
            CRB: 0,
            sound_mem: 0,
            GA0: 0,
            GA1: 0,
            GA2: 0,
            GA3: 0,
            k7_bit: 0,
            k7_char: 0,
            k7_in: None,
            k7_out: None,
            is_file_opened_out: false,
            k7_out_name: None,
        }
    }
}

impl Memory {
    // read with io
    pub(crate) fn read(&self, address: int) -> int {
        let page = ((address & 0xF000) >> 12) as usize;
        self.mem[self.mapper[page] as usize][(address & 0xFFF) as usize]
    }

    pub(crate) fn read_16(&self, address: int) -> int {
        let b1 = self.read(address);
        let b2 = self.read(address + 1);
        (b1 << 8) + b2
    }

    // write with io
    pub(crate) fn write(&mut self, address: int, value: int) {
        let page = ((address & 0xF000) >> 12) as usize;

        if (self.mapper[page] >= 14) && (self.mapper[page] <= 17) {
            return; // Protection en écriture de la ROM
        }

        if address < 0x1F40 {
            self.dirty[(address / 40) as usize] = true;
        }
        if page == 0x0A {
            self.hardware(address, value);
        } else {
            self.mem[self.mapper[page] as usize][(address & 0xFFF) as usize] = value & 0xFF;
        }
    }

    // write with io without Protection
    fn write_p(&mut self, address: int, value: int) {
        if address < 0x1F40 {
            self.dirty[(address / 40) as usize] = true;
        }
        let page = (address & 0xF000) >> 12;
        if page == 0x0A {
            self.hardware(address, value);
        } else {
            self.mem[self.mapper[page as usize] as usize][(address & 0xFFF) as usize] =
                value & 0xFF;
        }
    }

    // read without io
    fn get(&mut self, address: int) -> int {
        let page = (address & 0xF000) >> 12;
        self.mem[self.mapper[page as usize] as usize][(address & 0xFFF) as usize]
    }

    pub(crate) fn set(&mut self, address: int, value: int) {
        let page = (address & 0xF000) >> 12;
        self.mem[self.mapper[page as usize] as usize][(address & 0xFFF) as usize] = value & 0xFF;
    }

    pub(crate) fn POINT(&mut self, address: int) -> int {
        let page = (address & 0xF000) >> 12;
        self.mem[page as usize][(address & 0xFFF) as usize]
    }

    pub(crate) fn COLOR(&mut self, address: int) -> int {
        let page = (address & 0xF000) >> 12;
        self.mem[(page + 2) as usize][(address & 0xFFF) as usize]
    }

    pub(crate) fn is_dirty(&mut self, line: usize) -> bool {
        let ret = self.dirty[line];
        self.dirty[line] = false;
        ret
    }

    pub(crate) fn set_all_dirty(&mut self) {
        self.dirty.fill(true);
    }

    pub(crate) fn reset(&mut self) {
        for i in 0..0xFFFF {
            self.set(i, 0x00);
        }
        self.load_rom();
        self.CRA = 0x00;
        self.CRB = 0x00;
        self.DDRA = 0x5F;
        self.DDRB = 0x7F;

        self.mem[0xA + 2][0x7CC] = 0xFF;
        self.mem[0xA + 2][0x7CD] = 0xFF;
        self.mem[0xA + 2][0x7CE] = 0xFF;
        self.mem[0xA + 2][0x7CF] = 0xFF;

        self.patch_k7();
    }

    #[cfg(not(target_family = "wasm"))]
    fn load_rom(&mut self) {
        let embedded_bios = bios::BIOS;
        let starting_address = 0xC000;
        for i in starting_address..0x10000 {
            self.write_p(i, embedded_bios[(i - starting_address) as usize] as int);
        }
        //
        // let u = "bios/mo5.rom";
        // match fs::read(u) {
        //     Ok(bytes) => {
        //         let starting_address = 0xC000;
        //         for i in starting_address..0x10000 {
        //             self.write_p(i, bytes[(i - starting_address) as usize] as int);
        //         }
        //     }
        //     Err(error) => {
        //         //todo : dialog
        //         eprintln!("Error : mo5.rom file is missing {}", error);
        //     }
        // }
    }

    #[cfg(target_arch = "wasm32")]
    fn load_rom(&mut self) {
        let starting_address = 0xC000;
        for i in starting_address..0x10000 {
            self.write_p(i, bios::BIOS[(i - starting_address) as usize] as int);
        }
    }

    fn hardware(&mut self, ADR: int, mut OP: int) {
        /* 6821 système */
        /* acces à ORA ou DDRA */
        if ADR == 0xA7C0 {
            if (self.CRA & 0x04) == 0x04
            /* Accès à ORA */
            {
                if (OP & 0x01) == 0x01 {
                    self.mapper[0] = 0;
                    self.mapper[1] = 1;
                } else {
                    self.mapper[0] = 2;
                    self.mapper[1] = 3;
                }
                /* Mise à jour de ORA selon le masque DDRA */
                OP |= 0x80 + 0x20; // gestion de ,l'inter optique
                self.ORA = (self.ORA & (self.DDRA ^ 0xFF)) | (OP & self.DDRA);
                if self.light_pen_clic {
                    self.mem[0xA + 2][0x7C0] = self.ORA | 0x20;
                } else {
                    self.mem[0xA + 2][0x7C0] = self.ORA & (0xFF - 0x20);
                }
            } else {
                self.DDRA = OP;
                self.mem[0xA + 2][0x7C0] = OP;
            }
        } else if ADR == 0xA7C1
        /* accès à ORB ou DDRB */
        {
            if (self.CRB & 0x04) == 0x04
            /* Accès à ORB */
            {
                self.ORB = (self.ORB & (self.DDRB ^ 0xFF)) | (OP & self.DDRB);

                /* GESTION HARD DU CLAVIER */

                if self.key[(self.ORB & 0x7E) as usize] {
                    self.ORB &= 0x7F;
                } else {
                    self.ORB |= 0x80;
                }

                self.mem[0xA + 2][0x7C1] = self.ORB;
                self.sound_mem = ((self.ORB & 1) << 5) as u8;
            } else {
                self.DDRB = OP;
                self.mem[0xA + 2][0x7C1] = OP;
            }
        } else if ADR == 0xA7C2 {
            /* accès à CRA */
            self.CRA = (self.CRA & 0xD0) | (OP & 0x3F);
            self.mem[0xA + 2][0x7C2] = self.CRA;
        } else if ADR == 0xA7C3 {
            /* accès à CRB */
            self.CRB = (self.CRB & 0xD0) | (OP & 0x3F);
            self.mem[0xA + 2][0x7C3] = self.CRB;
        }
    }

    pub(crate) fn set_key(&mut self, i: usize) {
        if !self.key[i] {
            debug!("key down:{}", i);
            self.key[i] = true;
        }
    }

    pub(crate) fn rem_key(&mut self, i: usize) {
        if self.key[i] {
            debug!("key up:{}", i);
            self.key[i] = false;
        }
    }

    pub(crate) fn rem_key_slice(&mut self, start: usize, end: usize) {
        debug!("key up from {start} to {}", end - 1);
        self.key[start..end].fill(false);
    }

    pub(crate) fn rewind_k7(&mut self) {
        info!("rewind");
        if let Some(k7fis) = self.k7_in.as_mut() {
            k7fis.reset();
            self.k7_bit = 0;
            self.k7_char = 0;
        }
    }

    // pub(crate) fn set_k7(&mut self, k7: K7) {
    //     self.k7_fis = Some(k7);
    //     self.is_file_opened = true;
    // }

    pub(crate) fn set_k7(&mut self, k7: K7) {
        info!("Opened K7 {} of length {}", k7.name(), k7.len());
        self.k7_in = Some(k7);
        self.k7_bit = 0;
        self.k7_char = 0;
    }

    fn create_k7file(&mut self) -> bool {
        if self.k7_out_name.is_some() {
            return self.is_file_opened_out;
        }

        let aujourdhui = Local::now();

        let kout_name = aujourdhui.format("%Y-%m-%d-%H_%M_%S.k7").to_string();
        println!("Creating:{}", &kout_name);
        self.k7_out_name = Some(kout_name);
        if self.k7_out.is_none() {
            self.is_file_opened_out = false;
        }
        if self.is_file_opened_out {
            // todo : check this
            // self.K7fos.close(&mut self);
        }

        let k7out_name = &self.k7_out_name.clone().unwrap();
        if let Ok(k7fos) = File::open(k7out_name) {
            let buf = BufWriter::new(k7fos);
            self.k7_out = Some(buf);
            self.is_file_opened_out = true;
            // todo : dialog
            // JOptionPane.showMessageDialog(null, "Information : new file " + k7out_name);
        } else {
            // todo : dialog
            // JOptionPane.showMessageDialog(null, "Error : file not created " + e);
            return self.is_file_opened_out;
        }

        self.k7_bit = 0;
        self.k7_char = 0;

        self.is_file_opened_out
    }

    fn readbit(&mut self, screen: &mut Screen) -> int {
        if self.k7_in.is_none() {
            return 0;
        }

        /* doit_on lire un caractere ? */
        if self.k7_bit == 0x00 {
            if self.k7_in.is_some() {
                self.k7_char = self.k7_in.as_mut().unwrap().read().unwrap();
            } else {
                return 0;
            }

            self.k7_bit = 0x80;
        }
        let mut octet = self.get(0x2045);

        if (self.k7_char & self.k7_bit) == 0 {
            octet <<= 1;
            // A=0x00;
            self.set(0xF16A, 0x00);
        } else {
            octet = (octet << 1) | 0x01;
            // A=0xFF;
            self.set(0xF16A, 0xFF);
        }
        /* positionne l'octet dans la page 0 du moniteur */
        self.set(0x2045, octet & 0xFF);
        screen.led = (octet & 0xff) as u8;
        screen.show_led = 10;
        self.k7_bit >>= 1;
        0
    }

    pub(crate) fn periph(&mut self, PC: int, S: int, A: int, screen: &mut Screen) {
        if PC == 0xF169 {
            self.readbit(screen);
        }
        /* Write K7 byte */
        /* Merci  Olivier Tardieu pour le dsassemblage de la routine en ROM */
        if PC == 0xF1B0 {
            self.create_k7file(); // To do if necessary

            if !self.is_file_opened_out {
                return;
            }

            let data_out = [A as u8];
            if let Some(k7fos) = &mut self.k7_out
                && let Err(result) = k7fos.write(&data_out)
            {
                eprintln!("Error writing to file: {}", result);
            }
        }

        /* Motor On/Off/Test */
        if PC == 0xF18C {
            /* Mise � 0 du bit C*/
            let mut c = self.get(S);
            c &= 0xFE;
            self.write(S, c);
            //println!("motor ");
        }
        if PC == 0xf549 {
            self.write(S + 6, self.light_pen_x >> 8);
            self.write(S + 7, self.light_pen_x & 255);
            self.write(S + 8, self.light_pen_y >> 8);
            self.write(S + 9, self.light_pen_y & 255);
        }
    }

    fn patch_k7(&mut self) {
        /*

            PATCH une partie des fonctions du moniteur

            la squence 02 39 correspond
            Illegal (instruction)
            NOP
            le TRAP active la gestion des
            priphriques, la valeur du
            PC  ce moment permet de determiner
            la fonction  effectuer

        */
        // Crayon optique
        self.set(0xf548, 0x02); // PER instruction émulateur
        self.set(0xf549, 0x39); // RTS

        self.set(0xF1AF, 0x02);
        self.set(0xF1B0, 0x39);

        self.set(0xF18B, 0x02);
        self.set(0xF18C, 0x39);

        self.set(0xF168, 0x02);

        // LDA immediate for return
        self.set(0xF169, 0x86); //RTS
        self.set(0xF16A, 0x00); // no opcode

        self.set(0xF16B, 0x39);
    }

    fn unpatch_k7(&mut self) {}
}
