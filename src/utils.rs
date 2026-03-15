
/// Magic number for EIF1 format. Magic number in hex `0x31464945`
pub const EIF1_MAGIC_NUMBER: u32 = u32::from_le_bytes(*b"EIF1"); 

/// Objet rectangulaire représentant une zone de l'écran, défini par sa position (x, y) et sa taille (width, height).
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Rect {
    pub x: u16,
    pub y: u16,

    /// Width of the rectangle in pixels (axe X) (fr: Largeur)
    pub width: u16,

    /// Height of the rectangle in pixels (axe Y) (fr: Hauteur)
    pub height: u16,
}

/// Représente le rectangle de l'écran entier. (preset pour éviter de devoir le recréer à chaque fois)
#[allow(dead_code)]
pub const SCREEN_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};


/// Représente la taille d'une caractère d'une police de caractères.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct FontSize {
    /// Width (axe X) of one character in pixels (fr: Largeur)
    pub width: u16,
    /// Height (axe Y) of one character in pixels (fr: Hauteur)
    pub height: u16,
}

/// Taille d'un SMALL font character
#[allow(dead_code)]
pub const SMALL_FONT: FontSize = FontSize {
    width: 7,
    height: 14,
};


/// Taille d'un LARGE font character
#[allow(dead_code)]
pub const LARGE_FONT: FontSize = FontSize {
    width: 10,
    height: 18,
};

/// Représente un point dans l'espace 2D, défini par ses coordonnées x et y.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}
