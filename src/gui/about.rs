use egui::{Response, Ui, Widget};

#[derive(Debug)]
pub(super) struct About;

impl About {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl Widget for About {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.label(
            "     Maurice

            (C) G.Fetis 1997-1998-2006
            (C) DevilMarkus http://cpc.devilmarkus.de 2006
            (C) M.Le Goff 2014
            (C) Matthieu Casanova 2023-2025

            Rust conversion of Marcel o Cinq MO5 Emulator (Java version)",
        )
    }
}
