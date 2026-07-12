
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


/// Représentation des drapeaux de compilation du kernel et du userland.
/// 
/// - Stocker sous leur forme d'origine (un entier 32 bits).
/// 
/// ## Warning
/// 
/// ⚠️ Malgré le nom, certains flags sont dynamiques (runtime) :
///   - [`CompilationFlags::is_slot_a()`] : état hardware au moment de l'appel
///   - [`CompilationFlags::third_party_allowed()`] : dépend des apps installées
///   - [`CompilationFlags::security_level()`] : niveau hardware courant
/// 
/// Les autres sont de vrais flags de compilation (connus au compile-time) :
///   - `DEBUG`, `ASSERTIONS`, `API_LEVEL`, `IN_FACTORY`, `EMBED_EXTRA_DATA`
/// 
/// ---
/// 
/// | Bit   | Nom                     | Type   | Description                                                          |
/// | ----- | ----------------------- | ------ | -------------------------------------------------------------------- |
/// | 0     | debug userland          | `bool` | Activer les messages de debug en userland                            |
/// | 1     | assertions userland     | `bool` | Activer les assertions en userland                                   |
/// | 2     | third-party autorisé    | `bool` | Autoriser les applications tierces                                   |
/// | 3     | slot A actif            | `bool` | Vérifier si le slot A est actif                                      |
/// | 4-7   | external apps API level | `u4`   | Niveau d'API pour les applications externes                          |
/// | 8-11  | security level          | `u4`   | Niveau de sécurité hardware                                          |
/// | 12-15 | unused                  | `None` | **Espace non utilisé**                                               |
/// | 16    | debug kernel            | `bool` | Activer les messages de debug en kernel                              |
/// | 17    | assertions kernel       | `bool` | Activer les assertions en kernel                                     |
/// | 18    | in_factory              | `bool` | Indique si le device est en mode usine                               |
/// | 19    | embed_extra_data        | `bool` | Indique si le kernel contient des données supplémentaires embarquées |
/// | 20-31 | unused                  | `None` | **Espace non utilisé**                                               |
/// ```
/// 
/// CompilationFlasg : 32 bits (total)
///       - UserLand : 12 bits (bas)
///       - Unused   : 4 bits (milieu) 
///       - Kernel   : 4 bits (milieu)
///       - Unused   : 12 bits (haut)
/// 
/// ┌───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
/// │                                                     UserLand (0-11)                                                   │
/// ├────────────────┬─────────────────────┬──────────────────────┬──────────────┬─────────────────────────┬────────────────┤ ➔ ...
/// │     bits 0     │       bites 1       │        bites 2       │    bites 3   │        bites 4-7        │    bites 8-11  │
/// ├────────────────┼─────────────────────┼──────────────────────┼──────────────┼─────────────────────────┼────────────────┤
/// │ debug userland │ assertions userland │ third-party autorisé │ slot A actif │ external apps API level │ security level │
/// └────────────────┴─────────────────────┴──────────────────────┴──────────────┴─────────────────────────┴────────────────┘
/// ┌────────────────┐
/// │ Unused (12-15) │
/// ├────────────────┤ ➔ ...
/// │     4 bits     │
/// ├────────────────┤
/// │    No Data     │
/// └────────────────┘
/// ┌──────────────────────────────────────────────────────────────────┐
/// │                          Kernel (16-19)                          │
/// ├──────────────┬───────────────────┬────────────┬──────────────────┤ ➔ ...
/// │    bits 16   │      bites 17     │  bites 18  │      bites 19    │
/// ├──────────────┼───────────────────┼────────────┼──────────────────┤
/// │ debug kernel │ assertions kernel │ in_factory │ embed_extra_data │
/// └──────────────┴───────────────────┴────────────┴──────────────────┘
/// ┌────────────────┐
/// │ Unused (12-15) │
/// ├────────────────┤
/// │     4 bits     │
/// ├────────────────┤
/// │    No Data     │
/// └────────────────┘
/// ```
/// 
/// ## source
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/src/device/userland/drivers/compilation_flags.cpp#L18-L29
/// - https://github.com/numworks/epsilon/blob/72c8306f4fe3adf3bfc9c79802a39b80afb8e988/shared/ion/src/device/userland/drivers/compilation_flags.cpp#L18-L29
#[derive(Debug, Clone, Copy)]
pub struct CompilationFlags(pub u32);

impl CompilationFlags {

    /// Parse un pointeur vers une string hex en [`CompilationFlags`].
    ///
    /// - Renvoie une erreur si le pointeur est null, si : **la string n'est pas au format hex**, ou si **aucun caractère nul n'est trouvé**.
    /// - Parse la string hex en un entier 32 bits (format d'origine utiliser par Epsilon).
    /// 
    /// # Safety
    /// - `ptr` doit être non-null
    /// - `ptr` doit pointer vers une string ASCII hex valide, null-terminée
    /// - En pratique : toujours le buffer RAM statique retourné par le SVC 56
    pub unsafe fn from_hex_ptr(ptr: *const u8) -> Result<Self, GlobalError> {
        let mut val: u32 = 0;
        let mut null_found = false;

        // Parcourir les 8 caractères hex de la string
        for i in 0..8usize {
            let b = *ptr.add(i);

            if b == 0 {
                null_found = true;
                break;
            }

            let nibble = match b {
                b'0'..=b'9' => b - b'0',
                b'a'..=b'f' => b - b'a' + 10,
                b'A'..=b'F' => b - b'A' + 10,
                // Si le caractère n'est pas un hex valide, retourner une erreur
                _ => return Err(SoftwareError::InvalidFormat { details: "expected hex string" }.into()),
            };

            // Décaler la valeur précédente de 4 bits et ajouter le nibble actuel
            val = (val << 4) | nibble as u32;
        }

        // Vérifier si un caractère nul a été trouvé dans la string
        if !null_found {
            return Err(SoftwareError::NoNullTerminator.into());
        }

        Ok(Self(val))
    }

    /// Vérifie si le flag de compilation `userland_debug` est activé.
    /// 
    /// Build avec symboles de debug et comportements verbeux pour le userland. En prod = 0.
    pub fn userland_debug(&self)      -> bool { self.0 & (1 << 0)  != 0 }

    /// Vérifie si le flag de compilation `userland_assertions` est activé.
    /// 
    /// Les `assert()` pour le userland sont actifs. Si une assertion fail -> crash contrôlé plutôt que comportement indéfini. En prod = 0.
    pub fn userland_assertions(&self) -> bool { self.0 & (1 << 1)  != 0 }

    /// Vérifie si le flag de compilation `third_party_allowed` est activé.
    /// 
    /// `ExternalApps::allowThirdParty()`: les apps tierces sont autorisées à tourner. Si 0, l'app ne se lance même pas.
    pub fn third_party_allowed(&self) -> bool { self.0 & (1 << 2)  != 0 }

    /// Vérifie si le flag de compilation `slot_a_active` est activé.
    /// 
    /// `Device::Board::isRunningSlotA()`: la NumWorks a deux slots flash (A et B) pour les mises à jour OTA atomiques. Ce bit dit lequel est actif.
    pub fn is_slot_a(&self)           -> bool { self.0 & (1 << 3)  != 0 }

    /// Récupère le niveau d'API
    /// 
    /// Version de l'API externe. l'application externe déclare l'API level qu'elle requiert, le kernel refuse de la lancer si ça ne correspond pas. C'est le mécanisme de compatibilité.
    pub fn api_level(&self)           -> u32  { (self.0 >> 4) & 0xF     }

    /// Récupère le niveau de sécurité
    /// 
    /// Niveau de sécurité du firmware. Détermine ce que le kernel autorise (accès DFU, debug, etc.).
    pub fn security_level(&self)      -> u32  { (self.0 >> 8) & 0xF     }

    /// Vérifie si le flag de compilation `kernel_debug` est activé.
    /// 
    /// Build avec symboles de debug et comportements verbeux pour le kernel. En prod = 0.²
    pub fn kernel_debug(&self)        -> bool { self.0 & (1 << 16) != 0 }

    /// Vérifie si le flag de compilation `kernel_assertions` est activé.
    /// 
    /// Les `assert()` pour le kernel sont actifs. Si une assertion fail -> crash contrôlé plutôt que comportement indéfini. En prod = 0.
    pub fn kernel_assertions(&self)   -> bool { self.0 & (1 << 17) != 0 }

    /// Vérifie si le flag de compilation `in_factory` est activé.
    /// 
    /// Le device est en mode usine, probablement utilisé par NumWorks pendant la fabrication/calibration. Active des fonctions internes de test.
    pub fn in_factory(&self)          -> bool { self.0 & (1 << 18) != 0 }

    /// Vérifie si le flag de compilation `embed_extra_data` est activé.
    /// 
    /// Le kernel contient des données supplémentaires embarquées (tables de calibration, certificats, etc.).
    pub fn embed_extra_data(&self)    -> bool { self.0 & (1 << 19) != 0 }
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