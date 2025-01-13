#[derive(Default, Debug)]
pub struct Modifiers {
    pub(crate) ctrl: bool,
    pub(crate) alt: bool,
    pub(crate) shift: bool,
    pub(crate) logo: bool,
}

impl From<egui::Modifiers> for Modifiers {
    fn from(value: egui::Modifiers) -> Self {
        Self {
            ctrl: value.ctrl,
            alt: value.alt,
            shift: value.shift,
            logo: value.command || value.mac_cmd,
        }
    }
}
