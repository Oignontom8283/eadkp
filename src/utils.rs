
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


/// Représente une version au format "major.minor.patch", par exemple "1.2.3".
/// 
/// ## Exemple :
/// ```
/// let v =     Version::parse("21.5.6").unwrap();
/// let v_min = Version::new(17, 1, 0);
/// let v_max = Version::new(22, 4, 5);
/// 
/// assert!(v >= v_min);
/// assert!(v <= v_max);
/// assert!(v >= v_min && v <= v_max);
/// 
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    /// Crée une nouvelle instance de Version à partir des numéros de version fournis.
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Version { major, minor, patch }
    }
    
    /// Créer une instance de Version à partir d'une chaîne de caractères au format "major.minor.patch".
    /// - Retourne None si la chaîne n'est pas au format attendu.
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split('.');

        let major = parts.next()?.parse::<u8>().ok()?;
        let minor = parts.next()?.parse::<u8>().ok()?;
        let patch = parts.next()?.parse::<u8>().ok()?;

        // Si la chaîne ne contient pas exactement trois parties, retourner None
        if parts.next().is_none() {
            return None;
        }

        Some(Self { major, minor, patch })
    }
}


/// Convertit un buffer d'octets brut en chain de caractères Rust (`&str`)
/// - S'arrête au premier octet nul (`\0`) rencontré.
pub fn str_from_fixed_buffer(buffer: &[u8]) -> &str {
    unsafe {
        let ptr = buffer.as_ptr();
        let mut len = 0;

        // On cherche le \0 sans jamais dépasser la taille réelle du buffer
        while len < buffer.len() && *ptr.add(len) != 0 {
            len += 1;
        }

        // Création de la slice et conversion brute sans overhead
        let slice = slice::from_raw_parts(ptr, len);
        str::from_utf8_unchecked(slice)
    }
}