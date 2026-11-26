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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_no_modifier() {
        let m = Modifiers::default();
        assert!(!m.ctrl && !m.alt && !m.shift && !m.logo);
    }

    #[test]
    fn from_egui_maps_fields() {
        let m = Modifiers::from(egui::Modifiers {
            ctrl: true,
            shift: true,
            ..Default::default()
        });
        assert!(m.ctrl && m.shift && !m.alt && !m.logo);
    }

    #[test]
    fn logo_set_by_command_or_mac_cmd() {
        let cmd = egui::Modifiers {
            command: true,
            ..Default::default()
        };
        let mac = egui::Modifiers {
            mac_cmd: true,
            ..Default::default()
        };
        assert!(Modifiers::from(cmd).logo);
        assert!(Modifiers::from(mac).logo);
    }
}
