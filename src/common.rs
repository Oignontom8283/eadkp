
use crate::{SoftwareError, GlobalError};

#[path = "color.rs"]
mod color;
pub use color::*;

#[path = "image.rs"]
mod image;
pub use image::*;


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


/// Représente la taille d'une caractère d'une police de caractères.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct FontSize {
    /// Width (axe X) of one character in pixels (fr: Largeur)
    pub width: u16,
    /// Height (axe Y) of one character in pixels (fr: Hauteur)
    pub height: u16,
}


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
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl Version {
    /// Crée une nouvelle instance de Version à partir des numéros de version fournis.
    #[allow(dead_code)]
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Version { major, minor, patch }
    }
    
    /// Créer une instance de Version à partir d'une chaîne de caractères au format "major.minor.patch".
    /// - Retourne None si la chaîne n'est pas au format attendu.
    #[allow(dead_code)]
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split('.');

        let major = parts.next()?.parse::<u8>().ok()?;
        let minor = parts.next()?.parse::<u8>().ok()?;
        let patch = parts.next()?.parse::<u8>().ok()?;

        // Si la chaîne ne contient pas exactement trois parties, retourner None
        if parts.next().is_some() {
            return None;
        }

        Some(Self { major, minor, patch })
    }
}


/// Type de reset de l'appareil.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResetType {
    /// POWER on / NRST pin
    Hardware,
    /// crash ou Reset::core()
    Software,
}


/// Niveau d'autorisation du firmware en cours d'exécution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClearanceLevel {
    NumWorks                  = 0,
    NumWorksAndThirdPartyApps = 1,
    ThirdParty                = 2,
}