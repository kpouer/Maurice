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
    pub(crate) IRQ: bool,
    pub(crate) running: bool,
    pub(crate) lastTime: DateTime<Local>,
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
            lastTime: Local::now(),
            keys: Vec::new(),
            keytimer: 0,
            keypos: 0,
            typetext: None,
            IRQ: false,
            running: false,
        }
    }
}

impl Machine {
    // todo :activate
    // fn start(&mut self, mem: &mut Memory, screen: &mut Screen) {
    //     if !self.running {
    //         self.running = true;
    //         thread::spawn(move || {
    //             self.run(mem, screen);
    //         });
    //     }
    // }

    fn stop(&mut self) {
        self.running = false;
    }

    pub(crate) fn run(&mut self) {
        self.full_speed();
        self.synchronize();
    }

    // the emulator main loop
    fn full_speed(&mut self) {
        // let cl;

        self.screen.repaint(); // Mise a jour de l'affichage

        // Mise a jour du crayon optique a partir des donnée de la souris souris
        self.mem.LightPenClic = self.screen.mouse_clic;
        self.mem.LightPenX = self.screen.mouse_X;
        self.mem.LightPenY = self.screen.mouse_Y;

        self.mem.set(0xA7E7, 0x00);
        self.mem.GA3 = 0x00;
        /* 3.9 ms haut �cran (+0.3 irq)*/
        if self.IRQ {
            self.IRQ = false;
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
            self.IRQ = true;
            /* Positionne le bit 7 de CRB */
            self.mem.CRB |= 0x80;
            self.mem.set(0xA7C3, self.mem.CRB);
            let CC = self.micro.readCC();
            if (CC & 0x10) == 0 {
                self.micro.IRQ(&mut self.mem);
            }
            /* 300 cycles sous interrupt */
            self.micro.FetchUntil(300, &mut self.mem, &mut self.screen);
            self.mem.CRB &= 0x7F;
            self.mem.set(0xA7C3, self.mem.CRB);
        }
    }

    fn AutoType(&mut self, input: &String) {
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
                self.AutoType(&typetext);
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
        let real_time_millis:i64 = Local::now().timestamp_millis() - self.lastTime.timestamp_millis();

        let sleep_millis:i64 = 20i64 - real_time_millis - 1;
        if sleep_millis < 0 {
            self.lastTime = Local::now();
            return;
        }

        sleep(Duration::from_millis(sleep_millis as u64));
        self.lastTime = Local::now();
    }

    fn setK7FileFromUrl(&mut self, k7: &String) -> bool {
        return self.mem.setK7FileFromUrl(k7);
    }

    pub(crate) fn setK7File(&mut self, k7: &String) -> bool {
        return self.mem.setK7File(k7);
    }

    // soft reset method ("reinit prog" button on original MO5)
    fn resetSoft(&mut self, mem: &Memory) {
        self.micro.reset(mem);
    }

    // hard reset (match off and on)
    fn resetHard(&mut self, mem: &Memory) {
        for i in 0x2000..0x3000 {
            self.mem.set(i, 0);
        }
        self.micro.reset(mem);
    }

    // Debug Methods
    fn dumpRegisters(&mut self, mem: &mut Memory) -> String {
        return self.micro.printState(mem);
    }

    fn unassembleFromPC(&self, nblines: int, mem: &mut Memory) -> String {
        return unassemble(self.micro.PC, nblines, mem);
    }

    fn dumpSystemStack(&self, nblines: int) -> String {
        return "00".to_string();
    }
} // of class

