
/// Magic number for EIF1 format. Magic number in hex `0x31464945`
pub const EIF1_MAGIC_NUMBER: u32 = u32::from_le_bytes(*b"EIF1"); 


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub x: u16,
    pub y: u16,

    /// Width of the rectangle in pixels (axe X) (fr: Largeur)
    pub width: u16,

    /// Height of the rectangle in pixels (axe Y) (fr: Hauteur)
    pub height: u16,
}

pub const SCREEN_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};

/// Character size for a font
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FontSize {
    /// Width (axe X) of one character in pixels (fr: Largeur)
    pub width: u16,
    /// Height (axe Y) of one character in pixels (fr: Hauteur)
    pub height: u16,
}

/// Size of SMALL font character
/// 
/// Width (axe X) of one character in pixels (fr: Largeur)
/// 
/// Height (axe Y) of one character in pixels (fr: Hauteur)
pub const SMALL_FONT: FontSize = FontSize {
    width: 7,
    height: 14,
};

/// Size of LARGE font character
/// 
/// Width (axe X) of one character in pixels (fr: Largeur)
/// 
/// Height (axe Y) of one character in pixels (fr: Hauteur)
pub const LARGE_FONT: FontSize = FontSize {
    width: 10,
    height: 18,
};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

