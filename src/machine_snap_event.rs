use crate::raw_image::RawImage;

pub struct MachineSnapEvent {
    pub(crate) raw_image: Option<RawImage>,
    pub(crate) registers: String,
    pub(crate) unassembled: String,
}

impl MachineSnapEvent {
    pub(crate) fn new(raw_image: Option<RawImage>, registers: String, unassembled: String) -> Self {
        Self {
            raw_image,
            registers,
            unassembled,
        }
    }
}
