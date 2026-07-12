
use crate::{SoftwareError, GlobalError};
use core::fmt;

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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
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

/// Flags de compilation du kernel.
/// 
/// - Utilisé [`Self::is_production_build()`] au préalable pour vérifier si vous avez accès aux flags de debug et d'assertion du kernel.
/// - Stocké sous leurs forme d'origne (un entier 16 bits).
///
/// ## Warning
/// - ⚠️ A moins que vous ne soyez sur un OS compilé en debug mode, ces flags ne seront pas accessibles !
/// - Seuls les flags kernel sont disponibles depuis une app externe,
/// les flags userland (bits `0-15` du `u32` complet) ne sont pas accessibles via SVC.
///
/// ---
/// 
/// Layout du `u16` retourné (avant décalage `<<16` dans le `u32` complet) :
/// 
/// | Bit    | Nom               | Type   | Description                                                          |
/// | ------ | ----------------- | ------ | -------------------------------------------------------------------- |
/// | `0`    | debug kernel      | `bool` | Activer les messages de debug en kernel                              |
/// | `1`    | assertions kernel | `bool` | Activer les assertions en kernel                                     |
/// | `2`    | in_factory        | `bool` | Indique si le device est en mode usine                               |
/// | `3`    | embed_extra_data  | `bool` | Indique si le kernel contient des données supplémentaires embarquées |
/// | `4-15` | unused            | `None` | **Espace non utilisé**                                               |
/// 
/// ## source
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/src/device/userland/drivers/compilation_flags.cpp#L18-L29
/// - https://github.com/numworks/epsilon/blob/72c8306f4fe3adf3bfc9c79802a39b80afb8e988/shared/ion/src/device/userland/drivers/compilation_flags.cpp#L18-L29
#[derive(Debug, Clone, Copy)]
pub struct KernelFlags(pub u16);

impl KernelFlags {
    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Vérifie si l'appareil est en mode production (aucun flag de debug ou d'assertion activé = `0x0000`).
    /// - `true` = mode production
    /// - `false` = mode debug ou assertions activées.
    pub fn is_production_build(&self) -> bool {
        self.0 == 0
    }

    pub fn debug(&self)            -> bool { self.0 & (1 << 0) != 0 }
    pub fn assertions(&self)       -> bool { self.0 & (1 << 1) != 0 }
    pub fn in_factory(&self)       -> bool { self.0 & (1 << 2) != 0 }
    pub fn embed_extra_data(&self) -> bool { self.0 & (1 << 3) != 0 }
}

/// Ruleset du mode examen.
/// 
/// ## source
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/include/ion/exam_mode.h
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Ruleset {
    Off           = 0,
    Standard      = 1,
    Dutch         = 2,
    IBTest        = 3,
    PressToTest   = 4,
    Portuguese    = 5,
    English       = 6,
    STAAR         = 7,
    Pennsylvania  = 8,
    SouthCarolina = 9,
    NorthCarolina = 10,
    SAT           = 11,
    Uninitialized = 12,
}

impl TryFrom<u16> for Ruleset {
    type Error = GlobalError;

    fn try_from(data: u16) -> Result<Self, Self::Error> {
        match data {
            0  => Ok(Self::Off),
            1  => Ok(Self::Standard),
            2  => Ok(Self::Dutch),
            3  => Ok(Self::IBTest),
            4  => Ok(Self::PressToTest),
            5  => Ok(Self::Portuguese),
            6  => Ok(Self::English),
            7  => Ok(Self::STAAR),
            8  => Ok(Self::Pennsylvania),
            9  => Ok(Self::SouthCarolina),
            10 => Ok(Self::NorthCarolina),
            11 => Ok(Self::SAT),
            12 => Ok(Self::Uninitialized),
            _  => Err(SoftwareError::InvalidFormat { details: "unknown Ruleset value" }.into()),
        }
    }
}


/// Configuration du mode examen décodée depuis le secteur ExamBytes en flash.
#[derive(Debug, Clone, Copy)]
pub struct ExamMode {
    pub ruleset: Ruleset,
    /// `true` = PressToTest avec flags custom, `false` = Ruleset prédéfini
    pub configurable: bool,
    pub active: bool,
}

impl TryFrom<u16> for ExamMode {

    /// Parse un raw u16 depuis le secteur ExamBytes en flash.
    /// 
    /// ## Encodage
    /// - bit 0 = configurable, bits 1-14 = data (Ruleset ou flags PressToTest)
    /// 
    /// ## source
    /// - https://github.com/numworks/epsilon/blob/master/shared/ion/include/ion/exam_mode.h
    fn try_from(raw: u16) -> Result<Self, GlobalError> {
        let configurable = raw & 1 != 0;
        let data = (raw >> 1) & 0x3FFF;
        let ruleset = Ruleset::try_from(data)?;
        Ok(Self {
            ruleset,
            configurable,
            active: !matches!(ruleset, Ruleset::Off | Ruleset::Uninitialized),
        })
    }
    
    type Error = GlobalError;
}