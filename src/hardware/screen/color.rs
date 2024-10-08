pub(super) type Color = [u8; 3];

pub(crate) static PALETTE: [Color; 16] = [
    [0x00, 0x00, 0x00],
    [0xF0, 0x00, 0x00],
    [0x00, 0xF0, 0x00],
    [0xF0, 0xF0, 0x00],
    [0x00, 0x00, 0xF0],
    [0xF0, 0x00, 0xF0],
    [0x00, 0xF0, 0xF0],
    [0xF0, 0xF0, 0xF0],
    [0x63, 0x63, 0x63],
    [0xF0, 0x63, 0x63],
    [0x63, 0xF0, 0x63],
    [0xF0, 0xF0, 0x63],
    [0x00, 0x63, 0xF0],
    [0xF0, 0x63, 0xF0],
    [0x63, 0xF0, 0xF0],
    [0xF0, 0x63, 0x00],
];
