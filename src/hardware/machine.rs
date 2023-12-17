use std::path::Path;
use std::thread::sleep;
use std::time::Duration;
use chrono::{DateTime, Local};
use crate::hardware::keyboard::Keyboard;
use crate::hardware::M6809::{M6809, unassemble};
use crate::hardware::memory::Memory;
use crate::hardware::screen::Screen;
use crate::hardware::sound::Sound;
use crate::int;

#[derive(Debug)]
pub(crate) struct Machine {
    // Emulation Objects
    pub(crate) mem: Memory,
    pub(crate) micro: M6809,
    pub(crate) screen: Screen,
    pub(crate) keyboard: Keyboard,
    pub(crate) testtimer:int,
    pub(crate) irq: bool,
    pub(crate) last_time: DateTime<Local>,
    pub(crate) keys: Vec<int>,
    pub(crate) keytimer:int,
    pub(crate) keypos:int,
    pub(crate) typetext: Option<String>
}

impl Default for Machine {
    fn default() -> Self {
        println!("Machine::new()");
        let screen = Screen::new();
        println!("Machine::screen()");
        let mut mem = Memory::default();
        mem.reset();
        let sound = Sound::new();
        let micro = M6809::new(&mem);
        Machine {
            mem,
            micro,
            screen,
            keyboard: Keyboard::default(),
            testtimer: 0,
            last_time: Local::now(),
            keys: Vec::new(),
            keytimer: 0,
            keypos: 0,
            typetext: None,
            irq: false,
        }
    }
}

impl Machine {
    pub(crate) fn run(&mut self) {
        self.full_speed();
        self.synchronize();
    }

    // the emulator main loop
    fn full_speed(&mut self) {
        // let cl;

        self.screen.repaint(); // Mise a jour de l'affichage

        // Mise a jour du crayon optique a partir des donnée de la souris souris
        self.mem.light_pen_clic = self.screen.mouse_clic;
        self.mem.light_pen_x = self.screen.mouse_x;
        self.mem.light_pen_y = self.screen.mouse_y;

        self.mem.set(0xA7E7, 0x00);
        self.mem.GA3 = 0x00;
        /* 3.9 ms haut �cran (+0.3 irq)*/
        if self.irq {
            self.irq = false;
            self.micro.FetchUntil(3800, &mut self.mem, &mut self.screen);
        } else {
            self.micro.FetchUntil(4100, &mut self.mem, &mut self.screen);
        }

        /* 13ms fenetre */
        self.mem.set(0xA7E7, 0x80);
        self.mem.GA3 = 0x80;
        self.micro.FetchUntil(13100, &mut self.mem, &mut self.screen);

        self.mem.set(0xA7E7, 0x00);
        self.mem.GA3 = 0x00;
        self.micro.FetchUntil(2800, &mut self.mem, &mut self.screen);

        if (self.mem.CRB & 0x01) == 0x01 {
            self.irq = true;
            /* Positionne le bit 7 de CRB */
            self.mem.CRB |= 0x80;
            self.mem.set(0xA7C3, self.mem.CRB);
            let cc = self.micro.readCC();
            if (cc & 0x10) == 0 {
                self.micro.IRQ(&mut self.mem);
            }
            /* 300 cycles sous interrupt */
            self.micro.FetchUntil(300, &mut self.mem, &mut self.screen);
            self.mem.CRB &= 0x7F;
            self.mem.set(0xA7C3, self.mem.CRB);
        }
    }

    fn auto_type(&mut self, input: &String) {
        let input = input.replace("\"", "zxz");

        self.keys = Vec::new();
        for (i, c) in input.char_indices() {
            self.keys.push(c as int);
            println!("{}", self.keys[i]);
        }
        self.keytimer = 1;
    }

    fn synchronize(&mut self) {
        if self.testtimer != 0 && self.typetext.is_some() {
            self.testtimer += 1;
            if self.testtimer == 100 {
                let typetext = self.typetext.clone().unwrap();
                self.auto_type(&typetext);
                self.testtimer = 0;
            }
        }
        if self.keytimer != 0 {
            self.keytimer += 1;
            if self.keytimer == 2 {
                self.keyboard.press(self.keys[self.keypos as usize], &mut self.mem);
            }
            if self.keytimer == 3 {
                self.keyboard.release(self.keys[self.keypos as usize], &mut self.mem);
                self.keypos += 1;
                self.keytimer = 1;
                if self.keypos >= self.keys.len() as i32 {
                    self.keypos = 0;
                    self.keytimer = 0;
                    self.keys = Vec::new();
                }
            }
        }
        let real_time_millis:i64 = Local::now().timestamp_millis() - self.last_time.timestamp_millis();

        let sleep_millis:i64 = 20i64 - real_time_millis - 1;
        if sleep_millis < 0 {
            self.last_time = Local::now();
            return;
        }

        sleep(Duration::from_millis(sleep_millis as u64));
        self.last_time = Local::now();
    }

    pub(crate) fn set_k7_file(&mut self, k7: &Path) -> bool {
        return self.mem.set_k7file(k7);
    }

    // soft reset method ("reinit prog" button on original MO5)
    pub(crate) fn reset_soft(&mut self) {
        self.micro.reset(&self.mem);
    }

    // hard reset (match off and on)
    pub(crate) fn reset_hard(&mut self) {
        for i in 0x2000..0x3000 {
            self.mem.set(i, 0);
        }
        self.micro.reset(&self.mem);
    }

    // Debug Methods
    fn dump_registers(&mut self) -> String {
        self.micro.print_state()
    }

    fn unassemble_from_pc(&self, nblines: int, mem: &mut Memory) -> String {
        unassemble(self.micro.PC, nblines, mem)
    }
}

