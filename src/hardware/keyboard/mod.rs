use crate::hardware::keyboard::modifiers::Modifiers;
use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use crate::hardware::memory::Memory;

pub(crate) mod modifiers;
pub(crate) mod vkey;

#[derive(Debug, Default)]
pub(crate) struct Keyboard {
    pub(crate) modifiers: Modifiers,
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
            MO5VirtualKeyCode::Key1 => self.handle_shiftable_key(0x5e, press, mem),
            MO5VirtualKeyCode::Key2 => self.handle_shiftable_key(0x4e, press, mem),
            MO5VirtualKeyCode::Key3 => self.handle_shiftable_key(0x3e, press, mem),
            MO5VirtualKeyCode::Key4 => self.handle_shiftable_key(0x2e, press, mem),
            MO5VirtualKeyCode::Key5 => self.handle_shiftable_key(0x1e, press, mem),
            MO5VirtualKeyCode::Key6 => self.handle_shiftable_key(0x0e, press, mem),
            MO5VirtualKeyCode::Key7 => self.handle_shiftable_key(0x0c, press, mem),
            MO5VirtualKeyCode::Key8 => self.handle_shiftable_key(0x1c, press, mem),
            MO5VirtualKeyCode::Key9 => self.handle_shiftable_key(0x2c, press, mem),
            MO5VirtualKeyCode::Key0 => self.handle_shiftable_key(0x3c, press, mem),
            MO5VirtualKeyCode::Quote => key_memory(0x2e, press, mem),
            MO5VirtualKeyCode::Minus => self.handle_shiftable_key(0x4c, press, mem),
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

    fn handle_shiftable_key(&mut self, key: usize, press: bool, mem: &mut Memory) {
        if self.modifiers.shift {
            key_memory(0x70, press, mem);
        }
        key_memory(key, press, mem);
    }

    pub(crate) fn key_pressed(&mut self, virtual_key_code: MO5VirtualKeyCode, mem: &mut Memory) {
        mem.rem_key_slice(0, 127);
        self.key_translator(virtual_key_code, true, mem);
    }

    pub(crate) fn key_released(&mut self, virtual_key_code: MO5VirtualKeyCode, mem: &mut Memory) {
        self.key_translator(virtual_key_code, false, mem);
    }
}

fn key_memory(key: usize, press: bool, mem: &mut Memory) {
    if press {
        mem.set_key(key);
    } else {
        mem.rem_key(key);
    }
}
