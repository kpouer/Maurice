use std::collections::HashMap;

use crate::hardware::keyboard::modifiers::Modifiers;
use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use crate::hardware::memory::Memory;
use crate::int;

pub(crate) mod modifiers;
pub(crate) mod vkey;

#[derive(Debug)]
pub(crate) struct Keyboard {
    // translation table from scancode to java keycodes VK_
    ftable: HashMap<char, Key>,
    shiftpressed: int,
    pub(crate) modifiers: Modifiers,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            ftable: build_ftable(),
            shiftpressed: 0,
            modifiers: Modifiers::default(),
        }
    }
}

impl Keyboard {
    fn key_translator(&mut self, vk: MO5VirtualKeyCode, press: bool, mem: &mut Memory) {
        match vk {
            MO5VirtualKeyCode::Backspace => key_memory(0x6c, press, mem),
            MO5VirtualKeyCode::Delete => key_memory(0x12, press, mem),
            MO5VirtualKeyCode::Return => key_memory(0x68, press, mem),
            MO5VirtualKeyCode::Insert => key_memory(0x12, press, mem),
            MO5VirtualKeyCode::Up => key_memory(0x62, press, mem),
            MO5VirtualKeyCode::Left => key_memory(0x52, press, mem),
            MO5VirtualKeyCode::Right => key_memory(0x32, press, mem),
            MO5VirtualKeyCode::Down => key_memory(0x42, press, mem),
            MO5VirtualKeyCode::LControl => key_memory(0x6a, press, mem),
            MO5VirtualKeyCode::Escape => key_memory(0x66, press, mem),
            MO5VirtualKeyCode::LShift => key_memory(0x70, press, mem), //Sh
            MO5VirtualKeyCode::F11 => key_memory(0x72, press, mem),    // Ba
            MO5VirtualKeyCode::Key1 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x5e, press, mem);
            }
            MO5VirtualKeyCode::Key2 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x4e, press, mem);
            }
            MO5VirtualKeyCode::Key3 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x3e, press, mem);
            }
            MO5VirtualKeyCode::Key4 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x2e, press, mem);
            }
            MO5VirtualKeyCode::Key5 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x1e, press, mem);
            }
            MO5VirtualKeyCode::Key6 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x0e, press, mem);
            }
            MO5VirtualKeyCode::Key7 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x0c, press, mem);
            }
            MO5VirtualKeyCode::Key8 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x1c, press, mem);
            }
            MO5VirtualKeyCode::Key9 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x2c, press, mem);
            }
            MO5VirtualKeyCode::Key0 => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x3c, press, mem);
            }
            MO5VirtualKeyCode::Minus => {
                if self.modifiers.shift {
                    key_memory(0x70, press, mem);
                }
                key_memory(0x4c, press, mem);
            }
            MO5VirtualKeyCode::A => key_memory(0x5a, press, mem),
            MO5VirtualKeyCode::Z => key_memory(0x4a, press, mem),
            MO5VirtualKeyCode::E => key_memory(0x3a, press, mem),
            MO5VirtualKeyCode::R => key_memory(0x2a, press, mem),
            MO5VirtualKeyCode::T => key_memory(0x1a, press, mem),
            MO5VirtualKeyCode::Y => key_memory(0x0a, press, mem),
            MO5VirtualKeyCode::U => key_memory(0x08, press, mem),
            MO5VirtualKeyCode::I => key_memory(0x18, press, mem),
            MO5VirtualKeyCode::O => key_memory(0x28, press, mem),
            MO5VirtualKeyCode::P => key_memory(0x38, press, mem),
            MO5VirtualKeyCode::Caret => key_memory(0x48, press, mem),
            // Base(VirtualKeyCode::Dollar) => { key_memory(0x58, press, mem); }
            MO5VirtualKeyCode::Q => key_memory(0x56, press, mem),
            MO5VirtualKeyCode::S => key_memory(0x46, press, mem),
            MO5VirtualKeyCode::D => key_memory(0x36, press, mem),
            MO5VirtualKeyCode::F => key_memory(0x26, press, mem),
            MO5VirtualKeyCode::G => key_memory(0x16, press, mem),
            MO5VirtualKeyCode::H => key_memory(0x06, press, mem),
            MO5VirtualKeyCode::J => key_memory(0x04, press, mem),
            MO5VirtualKeyCode::K => key_memory(0x14, press, mem),
            MO5VirtualKeyCode::L => key_memory(0x24, press, mem),
            MO5VirtualKeyCode::M => key_memory(0x34, press, mem),
            // Base(VirtualKeyCode::Asterisk) => { key_memory(0x58, press, mem); }
            MO5VirtualKeyCode::W => key_memory(0x60, press, mem),
            MO5VirtualKeyCode::X => key_memory(0x50, press, mem),
            MO5VirtualKeyCode::C => key_memory(0x64, press, mem),
            MO5VirtualKeyCode::V => key_memory(0x54, press, mem),
            MO5VirtualKeyCode::B => key_memory(0x44, press, mem),
            MO5VirtualKeyCode::N => key_memory(0x00, press, mem),
            MO5VirtualKeyCode::Comma => key_memory(0x10, press, mem),
            MO5VirtualKeyCode::Period => key_memory(0x20, press, mem),
            MO5VirtualKeyCode::At => key_memory(0x30, press, mem),
            MO5VirtualKeyCode::Asterisk => key_memory(0x58, press, mem),
            MO5VirtualKeyCode::Space => key_memory(0x40, press, mem),
        }
    }

    pub(crate) fn key_pressed(&mut self, virtual_key_code: MO5VirtualKeyCode, mem: &mut Memory) {
        mem.rem_key_slice(0, 127);
        self.key_translator(virtual_key_code, true, mem);
    }

    pub(crate) fn key_released(&mut self, virtual_key_code: MO5VirtualKeyCode, mem: &mut Memory) {
        self.key_translator(virtual_key_code, false, mem);
    }

    pub(crate) fn press(&mut self, tmp: int, mem: &mut Memory) {
        let mut tmp = tmp;
        if tmp == 'z' as int {
            self.shiftpressed += 1;
            tmp = 16;
        }
        if tmp == 'x' as int {
            tmp = 50;
        }
        if self.shiftpressed == 2 {
            self.shiftpressed = 0;
            return;
        }

        if let Some(index) = self.ftable.get(&(tmp as u8 as char)) {
            mem.set_key(index.key as usize);
            if let Some(key2) = index.key2 {
                mem.set_key(key2 as usize);
            }
        }
    }

    pub(crate) fn release(&self, tmp: int, mem: &mut Memory) {
        let mut tmp = tmp;
        if tmp == 'z' as int {
            if self.shiftpressed == 1 {
                return;
            }
            tmp = 16;
        }
        if tmp == 'x' as int {
            tmp = 50;
        }
        if let Some(index) = self.ftable.get(&(tmp as u8 as char)) {
            mem.rem_key(index.key as usize);
            if let Some(key2) = index.key2 {
                mem.rem_key(key2 as usize);
            }
        }
    }
}

fn key_memory(key: usize, press: bool, mem: &mut Memory) {
    if press {
        mem.set_key(key);
    } else {
        mem.rem_key(key);
    }
}

/**
 * Key translation table to MO5 keyboard
 */
#[derive(Debug)]
struct Key {
    key: u8,
    key2: Option<u8>,
}

impl Key {
    fn new(key: u8) -> Self {
        Key { key, key2: None }
    }

    fn new_with_2_keys(key: u8, key2: u8) -> Self {
        Key {
            key,
            key2: Some(key2),
        }
    }
}

fn build_ftable() -> HashMap<char, Key> {
    let mut ftable = HashMap::new();
    /* STOP */
    //ftable[0x6E]=0x29;
    /* 1 .. ACC */
    ftable.insert('&', Key::new_with_2_keys(0x70, 0x5E));
    ftable.insert('é', Key::new_with_2_keys(0x70, 0x4E));
    ftable.insert('"', Key::new_with_2_keys(0x70, 0x3E));
    ftable.insert('\'', Key::new_with_2_keys(0x70, 0x2E));
    ftable.insert('(', Key::new_with_2_keys(0x70, 0x1E));
    ftable.insert('è', Key::new_with_2_keys(0x70, 0x0C));
    ftable.insert('_', Key::new_with_2_keys(0x70, 0x1C));
    ftable.insert('ç', Key::new_with_2_keys(0x70, 0x2C));
    ftable.insert('à', Key::new_with_2_keys(0x70, 0x3C));

    ftable.insert('1', Key::new(0x5E));
    ftable.insert('2', Key::new(0x4E));
    ftable.insert('3', Key::new(0x3E));
    ftable.insert('4', Key::new(0x2E));
    ftable.insert('5', Key::new(0x1E));
    ftable.insert('6', Key::new(0x0E));
    ftable.insert('7', Key::new(0x0C));
    ftable.insert('8', Key::new(0x1C));
    ftable.insert('9', Key::new(0x2C));
    ftable.insert('0', Key::new(0x3C));
    ftable.insert('-', Key::new(0x4C));
    ftable.insert('+', Key::new(0x5C));
    //todo : restore
    // keyboard.ftable[0x6C] = KeyEvent.VK_BACK_SPACE + EVENT;
    /* A .. --> */
    ftable.insert('a', Key::new(0x5A));
    ftable.insert('z', Key::new(0x4A));
    ftable.insert('e', Key::new(0x3A));
    ftable.insert('r', Key::new(0x2A));
    ftable.insert('t', Key::new(0x1A));
    ftable.insert('y', Key::new(0x0A));
    ftable.insert('u', Key::new(0x08));
    ftable.insert('i', Key::new(0x18));
    ftable.insert('o', Key::new(0x28));
    ftable.insert('p', Key::new(0x38));
    ftable.insert('/', Key::new(0x48));
    ftable.insert(')', Key::new(0x58));
    /* Q .. enter */
    ftable.insert('q', Key::new(0x56));
    ftable.insert('s', Key::new(0x46));
    ftable.insert('d', Key::new(0x36));
    ftable.insert('f', Key::new(0x26));
    ftable.insert('g', Key::new(0x16));
    ftable.insert('h', Key::new(0x06));
    ftable.insert('j', Key::new(0x04));
    ftable.insert('k', Key::new(0x14));
    ftable.insert('l', Key::new(0x24));
    ftable.insert('m', Key::new(0x34));
    //todo : restore
    // keyboard.ftable[0x68] = KeyEvent.VK_ENTER + EVENT;
    /* W .. , */
    ftable.insert('w', Key::new(0x60));
    ftable.insert('x', Key::new(0x50));
    ftable.insert('c', Key::new(0x64));
    ftable.insert('v', Key::new(0x54));
    ftable.insert('b', Key::new(0x44));
    ftable.insert('n', Key::new(0x00));
    ftable.insert(',', Key::new(0x10));
    ftable.insert('.', Key::new(0x20));
    ftable.insert('@', Key::new(0x30));
    ftable.insert('*', Key::new(0x58));

    //todo : restore
    // keyboard.ftable[0x6E] = 145 + EVENT;//STOP

    /* Specials keys */
    //todo : restore
    // keyboard.ftable[0x12] = KeyEvent.VK_INSERT + EVENT;
    // keyboard.ftable[0x02] = KeyEvent.VK_DELETE + EVENT;
    // keyboard.ftable[0x22] = 36 + EVENT;// Back to top
    // keyboard.ftable[0x62] = KeyEvent.VK_UP + EVENT;
    // keyboard.ftable[0x52] = KeyEvent.VK_LEFT + EVENT;
    // keyboard.ftable[0x32] = KeyEvent.VK_RIGHT + EVENT;
    // keyboard.ftable[0x42] = KeyEvent.VK_DOWN + EVENT;
    /* espace */
    ftable.insert(' ', Key::new(0x40));
    ftable
}
