use crate::hardware::keyboard::Keyboard;

use crate::hardware::k7::K7;
use crate::hardware::memory::Memory;
use crate::hardware::screen::Screen;
use crate::hardware::sound::Sound;
use crate::hardware::M6809::{unassemble, M6809};
use crate::int;
use crate::raw_image::RawImage;
use chrono::{DateTime, Local};
use log::{debug, info};

pub struct Machine {
    // Emulation Objects
    pub(crate) mem: Memory,
    pub(crate) micro: M6809,
    pub(crate) screen: Screen,
    sound: Sound,
    pub(crate) keyboard: Keyboard,
    pub(crate) irq: bool,
    pub(crate) last_time: DateTime<Local>,
    pub(crate) keys: Vec<int>,
    pub(crate) running: bool,
    #[cfg(target_arch = "wasm32")]
    waiting: web_time::Instant,
    #[cfg(target_arch = "wasm32")]
    sleeptime: u128,
}

impl Default for Machine {
    fn default() -> Self {
        info!("Machine created");
        let screen = Screen::new(get_ratio());
        info!("Machine created");
        let mut mem = Memory::default();
        info!("Memory created");
        mem.reset();
        let micro = M6809::new(&mem);
        info!("CPU created");
        Self {
            mem,
            micro,
            screen,
            sound: Sound::new(),
            keyboard: Keyboard::default(),
            last_time: Local::now(),
            keys: Vec::new(),
            irq: false,
            running: true,
            #[cfg(target_arch = "wasm32")]
            waiting: web_time::Instant::now(),
            #[cfg(target_arch = "wasm32")]
            sleeptime: 0,
        }
    }
}

impl Machine {
    pub fn run_loop(&mut self) -> Option<RawImage> {
        #[cfg(debug_assertions)]
        debug!("run_loop");
        if self.running {
            self.run();
            self.screen.paint(&mut self.mem);
            let raw_image = self.screen.get_pixels();
            Some(raw_image)
        } else {
            None
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub(crate) fn run(&mut self) {
        self.full_speed();
        self.synchronize();
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn run(&mut self) {
        if self.waiting.elapsed().as_millis() > self.sleeptime {
            self.full_speed();
            self.synchronize();
        }
    }

    // the emulator main loop
    fn full_speed(&mut self) {
        // let cl;

        // Mise a jour du crayon optique a partir des donnée de la souris souris
        self.mem.light_pen_clic = self.screen.mouse_clic;
        self.mem.light_pen_x = self.screen.mouse_x;
        self.mem.light_pen_y = self.screen.mouse_y;

        self.mem.set(0xA7E7, 0x00);
        self.mem.GA3 = 0x00;
        /* 3.9 ms haut �cran (+0.3 irq)*/
        if self.irq {
            self.irq = false;
            self.micro
                .FetchUntil(3800, &mut self.mem, &mut self.screen, &mut self.sound);
        } else {
            self.micro
                .FetchUntil(4100, &mut self.mem, &mut self.screen, &mut self.sound);
        }

        /* 13ms fenetre */
        self.mem.set(0xA7E7, 0x80);
        self.mem.GA3 = 0x80;
        self.micro
            .FetchUntil(13100, &mut self.mem, &mut self.screen, &mut self.sound);

        self.mem.set(0xA7E7, 0x00);
        self.mem.GA3 = 0x00;
        self.micro
            .FetchUntil(2800, &mut self.mem, &mut self.screen, &mut self.sound);

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
            self.micro
                .FetchUntil(300, &mut self.mem, &mut self.screen, &mut self.sound);
            self.mem.CRB &= 0x7F;
            self.mem.set(0xA7C3, self.mem.CRB);
        }
    }

    fn synchronize(&mut self) {
        let real_time_millis: i64 =
            Local::now().timestamp_millis() - self.last_time.timestamp_millis();

        let sleep_millis: i64 = 20i64 - real_time_millis - 1;
        if sleep_millis < 0 {
            self.last_time = Local::now();
            return;
        }

        #[cfg(target_arch = "wasm32")]
        {
            self.sleeptime = sleep_millis as u128;
            self.waiting = web_time::Instant::now();
        }

        #[cfg(not(target_family = "wasm"))]
        std::thread::sleep(std::time::Duration::from_millis(sleep_millis as u64));
        self.last_time = Local::now();
    }

    pub fn set_k7(&mut self, k7: K7) {
        info!("Machine::set_k7_data()");
        self.mem.set_k7(k7);
    }

    pub(crate) fn rewind_k7(&mut self) {
        info!("Machine::rewind_k7()");
        self.mem.rewind_k7();
    }

    pub(crate) fn stop(&mut self) {
        info!("Machine::stop()");
        self.running = false;
    }

    pub(crate) fn start(&mut self) {
        info!("Machine::start()");
        self.running = true;
    }

    // soft reset method ("reinit prog" button on original MO5)
    pub(crate) fn reset_soft(&mut self) {
        info!("Machine::reset_soft()");
        self.running = false;
        self.micro.reset(&self.mem);
        self.running = true;
    }

    // hard reset (match off and on)
    pub(crate) fn reset_hard(&mut self) {
        info!("Machine::reset_hard()");
        self.running = false;
        for i in 0x2000..0x3000 {
            self.mem.set(i, 0);
        }
        self.micro.reset(&self.mem);
        self.running = true;
    }

    pub(crate) fn dump_registers(&mut self) -> String {
        self.micro.print_state()
    }

    pub(crate) fn unassemble_from_pc(&self, nblines: int, mem: &Memory) -> String {
        unassemble(self.micro.PC, nblines, mem)
    }
}

fn get_ratio() -> usize {
    crate::hardware::screen::DEFAULT_PIXEL_SIZE
}
