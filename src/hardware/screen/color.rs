#[derive(Debug)]
pub(super) struct Color {
    color: [u8; 3],
}

impl AsRef<[u8; 3]> for Color {
    fn as_ref(&self) -> &[u8; 3] {
        &self.color
    }
}

pub(crate) static PALETTE: [Color; 16] = [
    Color {
        color: [0x00, 0x00, 0x00],
    },
    Color {
        color: [0xF0, 0x00, 0x00],
    },
    Color {
        color: [0x00, 0xF0, 0x00],
    },
    Color {
        color: [0xF0, 0xF0, 0x00],
    },
    Color {
        color: [0x00, 0x00, 0xF0],
    },
    Color {
        color: [0xF0, 0x00, 0xF0],
    },
    Color {
        color: [0x00, 0xF0, 0xF0],
    },
    Color {
        color: [0xF0, 0xF0, 0xF0],
    },
    Color {
        color: [0x63, 0x63, 0x63],
    },
    Color {
        color: [0xF0, 0x63, 0x63],
    },
    Color {
        color: [0x63, 0xF0, 0x63],
    },
    Color {
        color: [0xF0, 0xF0, 0x63],
    },
    Color {
        color: [0x00, 0x63, 0xF0],
    },
    Color {
        color: [0xF0, 0x63, 0xF0],
    },
    Color {
        color: [0x63, 0xF0, 0xF0],
    },
    Color {
        color: [0xF0, 0x63, 0x00],
    },
];
