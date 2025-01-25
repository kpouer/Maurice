use crate::hardware::machine::Machine;
use egui::{Response, Ui, Widget};

pub(super) struct Debug<'a> {
    machine: &'a mut Machine,
}

impl<'a> Debug<'a> {
    pub(super) fn new(machine: &'a mut Machine) -> Self {
        Self { machine }
    }
}

impl Widget for Debug<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let dbg = self.machine.dump_registers();
        let unassemble = self.machine.unassemble_from_pc(10, &self.machine.mem);

        ui.vertical(|ui| {
            ui.label(dbg);
            ui.label(unassemble);
        })
        .response
    }
}
