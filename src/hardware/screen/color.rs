pub(crate) const COLOR_DEPTH: usize = 3;
pub(super) type Color = [u8; COLOR_DEPTH];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palette_has_16_distinct_colors() {
        for (i, a) in PALETTE.iter().enumerate() {
            for b in &PALETTE[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }

    #[test]
    fn palette_black_and_white() {
        assert_eq!(PALETTE[0], [0, 0, 0]);
        assert_eq!(PALETTE[7], [0xF0, 0xF0, 0xF0]);
    }

    #[test]
    fn color_depth_matches_color_size() {
        assert_eq!(COLOR_DEPTH, PALETTE[0].len());
    }
}
