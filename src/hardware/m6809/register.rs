use std::ops::{AddAssign, Deref};

#[derive(Default, Debug)]
pub struct Register<T> {
    value: T,
}

impl Deref for Register<u16> {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Register<u16> {
    pub const fn lower_byte(&self) -> u8 {
        self.value as u8
    }

    pub const fn upper_byte(&self) -> u8 {
        (self.value >> 8) as u8
    }
}

impl<T> Register<T> {
    pub fn set<V: Into<T>>(&mut self, value: V) {
        self.value = value.into();
    }
}

impl AddAssign<u8> for Register<u16> {
    fn add_assign(&mut self, rhs: u8) {
        self.value = self.value.saturating_add(rhs as u16);
    }
}

impl AddAssign<u16> for Register<u16> {
    fn add_assign(&mut self, rhs: u16) {
        self.value = self.value.saturating_add(rhs);
    }
}