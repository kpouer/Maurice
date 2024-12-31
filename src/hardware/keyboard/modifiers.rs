#[derive(Default, Debug)]
pub struct Modifiers {
    pub(crate) ctrl: bool,
    pub(crate) alt: bool,
    pub(crate) shift: bool,
    pub(crate) logo: bool,
}

#[cfg(feature = "speedy2d-display")]
impl From<speedy2d::window::ModifiersState> for Modifiers {
    fn from(value: speedy2d::window::ModifiersState) -> Self {
        Self {
            ctrl: value.ctrl(),
            alt: value.alt(),
            shift: value.shift(),
            logo: value.logo(),
        }
    }
}

#[cfg(feature = "egui-display")]
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
