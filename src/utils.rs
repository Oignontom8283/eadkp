use crate::{SoftwareError, GlobalError, common};


/// Convertit un buffer d'octets brut en chain de caractères Rust (`&str`)
/// - S'arrête au premier octet nul (`\0`) rencontré.
/// - Renvoie une erreur si aucun octet nul n'est trouvé dans le buffer.
/// ```
/// //                          S'arrête ici
/// //                          ↓
/// let buffer = b"Hello, world!\0Extra data";
/// let result = str_from_fixed_buffer(buffer);
/// 
/// assert_eq!(result.unwrap(), "Hello, world!");
/// ```
pub fn str_from_fixed_buffer(buffer: &[u8]) -> Result<&str, GlobalError> {
    
    // Obtenir la position du premier octet nul dans le buffer
    let len = buffer
        .iter()
        .position(|&b| b == 0)
        .ok_or(SoftwareError::NoNullTerminator)?;

    let slice = &buffer[..len];

    // Convertir le slice en &str sans vérifier la validité UTF-8 (unsafe)
    Ok(unsafe { str::from_utf8_unchecked(slice) })
}


/// Calculer la taille d'une plage de mémoire définie par deux pointeurs.
/// 
/// ## Exemple
/// ```
/// let start_ptr = 0x1000 as *const u8; // 4096
/// let end_ptr = 0x2000 as *const u8; // 8192
/// 
/// let size = ptr_range_size(start_ptr, end_ptr).unwrap();
/// 
/// assert_eq!(size, 0x1000); // 4096
/// ```
pub fn ptr_range_size(start: *const u8, end: *const u8) -> Result<usize, GlobalError> {

    // Vérifier que les pointeurs sont valides et dans le bon ordre
    if start.is_null() || end.is_null() {
        return Err(SoftwareError::NullPointer.into());
    }

    // Vérifier que les pointeurs sont dans le bon ordre (start <= end)
    if end < start {
        return Err(SoftwareError::InvalidPointerRange { start, end }.into());
    }
    
    // Calculer la taille de la plage en utilisant offset_from
    unsafe { Ok(ptr_range_size_unchecked(start, end)) }
}

/// Calculer la taille d'une plage de mémoire définie par deux pointeurs sans vérifier leur validité.
///     
/// Référez-vous à la documentation de [`ptr_range_size`] pour plus d'informations sur l'utilisation de cette fonction.
/// 
/// ## Safety
/// - Les pointeurs doivent être valides
/// - Les pointeurs doivent être dans le bon ordre (start <= end)
pub unsafe fn ptr_range_size_unchecked(start: *const u8, end: *const u8) -> usize {
    // Calculer la taille de la plage en utilisant offset_from
    end.offset_from(start) as usize
}

/// Faire un appel SVC et récupérer la valeur de retour dans `r0`.
/// - (`u32` / `bool` / `enum` `#[repr(u32)]`)
/// - Le type de retour doit être spécifié explicitement lors de l'appel de la macro.
/// - Miroir de SVC_RETURNING_R0 du C++
#[macro_export]
macro_rules! svc_r0 {
    ($num:expr, $ty:ty) => {{
        #[cfg(target_os = "none")]
        {
            let result: u32;
            unsafe {
                core::arch::asm!(
                    "svc {num}",
                    "mov {out}, r0",
                    num = const $num,
                    out = out(reg) result,
                    lateout("r0") _,
                    lateout("r1") _,
                    lateout("r2") _,
                    lateout("r3") _,
                    options(nostack),
                );
            }
            result as $ty
        }
        #[cfg(not(target_os = "none"))]
        {
            panic!("SVC not supported on Simulator");
        }
    }};
}

/// Faire un appel SVC et récupérer la valeur de retour dans `s0`.
/// - (`f32`)
/// - Miroir de SVC_RETURNING_S0 du C++
#[macro_export]
macro_rules! svc_s0 {
    ($num:expr) => {{
        #[cfg(target_os = "none")]
        {
            let bits: u32;
            unsafe {
                core::arch::asm!(
                    "svc {num}",
                    "vmov {bits}, s0",
                    num  = const $num,
                    bits = out(reg) bits,
                    lateout("r0") _,
                    lateout("r1") _,
                    lateout("r2") _,
                    lateout("r3") _,
                    options(nostack),
                );
            }
            f32::from_bits(bits)
        }
        #[cfg(not(target_os = "none"))]
        {
            panic!("SVC not supported on Simulator");
        }
    }};
}
/// Faire un appel SVC et récupérer la valeur de retour 64 bits dans `r0:r1`.
/// - (`u64` / `enum` `#[repr(u64)]`)
/// - Miroir de SVC_RETURNING_R0R1 du C++
#[macro_export]
macro_rules! svc_r0r1 {
    ($num:expr) => {{
        #[cfg(target_os = "none")]
        {
            let lo: u32;
            let hi: u32;
            unsafe {
                core::arch::asm!(
                    "svc {num}",
                    "mov {lo}, r0",
                    "mov {hi}, r1",
                    num = const $num,
                    lo  = out(reg) lo,
                    hi  = out(reg) hi,
                    lateout("r0") _,
                    lateout("r1") _,
                    lateout("r2") _,
                    lateout("r3") _,
                    options(nostack),
                );
            }
            ((hi as u64) << 32) | (lo as u64)
        }
        #[cfg(not(target_os = "none"))]
        {
            panic!("SVC not supported on Simulator");
        }
    }};
}

/// Faire un appel SVC et stocker l'adresse de destination dans `r0` avant le SVC.
/// - (`*mut T` / `*const T`)
/// - Miroir de SVC_RETURNING_STASH_ADDRESS_IN_R0 du C++
#[macro_export]
macro_rules! svc_addr_in_r0 {
    ($num:expr, $ptr:expr) => {{
        #[cfg(target_os = "none")]
        unsafe {
            core::arch::asm!(
                "mov r0, {ptr}",
                "svc {num}",
                ptr = in(reg) $ptr,
                num = const $num,
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
        #[cfg(not(target_os = "none"))]
        {
            panic!("SVC not supported on Simulator");
        }
    }};
}

/// Faire un appel SVC avec un buffer alloué par l'appelant (caller-allocated buffer).
/// - Le kernel reçoit une adresse dans `r0` et y écrit les données de retour (Notre buffer).
/// - Miroir de SVC_RETURNING_STASH_ADDRESS_IN_R0 du C++ **mais avec gestion du buffer intégrée.**
/// - Retourne `[u8; $len]` rempli par le kernel.
/// - Il s'agit d'une version automatisée de `svc_addr_in_r0!` qui gére le buffer pour vous.
/// 
/// ## Exemple
/// ```rust
/// // Appel SVC pour récupérer le numéro de série
/// let serial = svc_buf!(SVC_SERIAL_NUMBER_COPY, 16); // 16 octets — taille du buffer voulu
/// 
/// // Le buffer est rempli par le kernel
/// assert!(serial.iter().all(|&b| b != 0)); 
/// ```
#[macro_export]
macro_rules! svc_buf {
    ($num:expr, $len:expr) => {{
        #[cfg(target_os = "none")]
        {
            let mut buf = [0u8; $len];
            unsafe {
                core::arch::asm!(
                    "mov r0, {ptr}",
                    "svc {num}",
                    ptr = in(reg) buf.as_mut_ptr(),
                    num = const $num,
                    lateout("r0") _,
                    lateout("r1") _,
                    lateout("r2") _,
                    lateout("r3") _,
                    options(nostack),
                );
            }
            buf
        }
        #[cfg(not(target_os = "none"))]
        {
            panic!("SVC not supported on Simulator");
        }
    }};
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


