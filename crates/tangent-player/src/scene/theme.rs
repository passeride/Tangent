use bevy::prelude::*;
#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: Color,
    pub panel: Color,
    pub ink: Color,
    pub muted: Color,
    pub staff: Color,
    pub white: Color,
    pub black: Color,
    pub colors: [Color; 4],
}
pub fn palette(index: usize) -> Theme {
    let (bg, panel, ink, muted, white, black, colors) = match index % 3 {
        1 => (
            0x211922,
            0x30222e,
            0xf4e5db,
            0xbba4ac,
            0xf1e8df,
            0x30232e,
            [0xefaf82, 0xd6b2e8, 0xe7ce8d, 0xe89caa],
        ),
        2 => (
            0xe0e5df,
            0xf3f2e8,
            0x2f4641,
            0x627b70,
            0xfaf8ef,
            0x283e3b,
            [0x407e74, 0x796aaf, 0xac723a, 0x477eaa],
        ),
        _ => (
            0x0b1720,
            0x142731,
            0xeaf2ee,
            0x8da4aa,
            0xe5e9e4,
            0x17252e,
            [0x8ce0c1, 0xa8b9f4, 0xe1c391, 0x8dcae9],
        ),
    };
    let color = |v: u32| Color::srgb_u8((v >> 16) as u8, (v >> 8) as u8, v as u8);
    Theme {
        bg: color(bg),
        panel: color(panel),
        ink: color(ink),
        muted: color(muted),
        staff: color(muted).with_alpha(0.35),
        white: color(white),
        black: color(black),
        colors: colors.map(color),
    }
}
