
/*!
Ce sous-module fournit des fonctions pour interagir directement avec le système d'exploitation (OS) de l'appareil :
version, numéro de série, drapeaux de compilation, type de reset, niveau d'autorisation, etc.

## Warning
- Tout les éléments fournis ici n'ont pas tous but à être utiles, ils existent au cas ou ils seraient utiles par qui que ce soit.
- Tenez compte que la casi totalité des fonctions de ce sous module n'ont pas de version Dummy, et renvoient donc une erreur [`SoftwareError::SimulatorNotSupported`] si vous les appelez depuis un simulateur (OS hôte).
*/

use crate::{
    utils::{ptr_range_size_unchecked, str_from_fixed_buffer},
    common::{Version, ResetType, ClearanceLevel, KernelFlags, ExamMode},
    epsilon::{kernel_header, userland_header},
    GlobalError, SoftwareError,
    constant,
    svc_buf, svc_r0
};
use alloc::{ string::{String} };


/// Obtenir la version du Sytem d'exploitation en cours d'utilisation (version du kernel).
/// 
/// ## Exemple
/// ```
/// let version = eadkp::sys::version();
/// assert!(!version.is_empty()); // ex: 24.2.3
/// ```
#[cfg(target_os = "none")]
pub fn version() -> Result<Version, GlobalError> {
    
    // Obtenir le buffer de version du kernel
    let version_buffer_raw = &kernel_header().unwrap().epsilon_version; // 8 bytes

    // Extraire la chaine de caractères
    let version_str = str_from_fixed_buffer(version_buffer_raw)?;

    // Convertir la chaine de caractères en obj Version
    Version::parse(version_str).ok_or(SoftwareError::InvalidFormat { details: "expected version format" }.into())
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn version() -> Result<Version, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir le hash du commit de compilation du noyau de l'os en cours d'utilisation.
/// 
/// ## Exemple
/// ```
/// let hash = eadkp::sys::hash_commit();
/// assert!(!hash.is_empty()); // ex: abcdef12
/// ```
#[cfg(target_os = "none")]
pub fn hash_commit() -> Result<&'static str, GlobalError> {

    // Obternir le buffer du hash du commit du kernel
    let hash_buffer_raw = &kernel_header().unwrap().commit_hash; // 8 bytes

    // Extraire la chaine de caractères
    str_from_fixed_buffer(hash_buffer_raw)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn hash_commit() -> Result<&'static str, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir la version du kernel attendue par le UserLand.
/// - Donnée interne utilisée par le UserLand pour vérifier ça compatibilité avec le kernel. Utile pour les UserLand customisés principalement.
#[cfg(target_os = "none")]
pub fn expected_version() -> Result<Version, GlobalError> {

    // Obtenir le buffer de la version attendue du kernel
    let expected_version_buffer_raw = &userland_header().expected_epsilon_version; // 8 bytes

    // Extraire la chaine de caractères
    let expected_version_str = str_from_fixed_buffer(expected_version_buffer_raw)?;

    // Convertir la chaine de caractères en obj Version
    Version::parse(expected_version_str).ok_or(SoftwareError::InvalidFormat { details: ("expected version format") }.into())
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn expected_version() -> Result<Version, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}



/// Obtenir la taille du système de fichiers (storage).
/// - Taille en bytes (octets).
/// ## Warning
/// - ⚠️ Comprend **TOUT** la zone du FS, y compris les zone non utilisables pour stocker des fichiers (ex: magic number).
#[cfg(target_os = "none")]
pub fn filesystem_size() -> Result<usize, GlobalError> {
    Ok(userland_header().storage_size_ram as usize)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn filesystem_size() -> Result<usize, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir l'adresse de début de la zone mémoire allouée aux applications externes (RAM).
#[cfg(target_os = "none")]
pub fn ext_app_ram_start() -> Result<*const u8, GlobalError> {
    Ok(userland_header().external_apps_ram_start)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_ram_start() -> Result<*const u8, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir l'adresse de fin de la zone mémoire allouée aux applications externes (RAM).
#[cfg(target_os = "none")]
pub fn ext_app_ram_end() -> Result<*const u8, GlobalError> {
    Ok(userland_header().external_apps_ram_end)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_ram_end() -> Result<*const u8, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir la taille de la zone mémoire allouée aux applications externes (RAM).
/// - Taille en bytes (octets).
#[cfg(target_os = "none")]
pub fn ext_app_ram_size() -> Result<usize, GlobalError> {
    let start_ptr = ext_app_ram_start()?;
    let end_ptr = ext_app_ram_end()?;

    // Calculer la taille de la plage mémoire
    Ok(unsafe { ptr_range_size_unchecked(start_ptr, end_ptr) })
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_ram_size() -> Result<usize, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir l'adresse de début de la zone flash allouée aux binaires des applications externes.
#[cfg(target_os = "none")]
pub fn ext_app_flash_start() -> Result<*const u8, GlobalError> {
    Ok(userland_header().external_apps_flash_start)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_flash_start() -> Result<*const u8, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir l'adresse de fin de la zone flash allouée aux binaires des applications externes.
#[cfg(target_os = "none")]
pub fn ext_app_flash_end() -> Result<*const u8, GlobalError> {
    Ok(userland_header().external_apps_flash_end)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_flash_end() -> Result<*const u8, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir la taille de la zone de mémoire flash allouée aux stockages des binaires des applications externes.
/// - Taille en bytes (octets).
#[cfg(target_os = "none")]
pub fn ext_app_flash_size() -> Result<usize, GlobalError> {
    let start_ptr = ext_app_flash_start()?;
    let end_ptr = ext_app_flash_end()?;

    // Calculer la taille de la plage mémoire
    Ok(unsafe { ptr_range_size_unchecked(start_ptr, end_ptr) })
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_flash_size() -> Result<usize, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


// ! TEMPORARILY DISABLE DEVICE_NAME() PENDING RAM/FLASH INVESTIGATIONZZ
// /// Obtenir le nom de l'appareil (device) en cours d'utilisation.
// /// - Renvoie "Unknown Device" si le nom n'est pas disponible ou invalide.
// /// 
// /// ## Attention
// /// - Il est fortement possible que vous n'arriviez pas a obtenir le nom de l'appareil, pour des raisons qui m'échappent.
// #[cfg(target_os = "none")]
// pub fn device_name() -> Result<&'static str, GlobalError> {
    
//     // Obtenir les pointeurs vers le début et la fin du nom de l'appareil
//     let name_start_ptr = userland_header().device_name_flash_start;
//     let name_end_ptr = userland_header().device_name_flash_end;

//     // Calculer la taille et on s'assure que les pointeurs sont valides
//     let len = ptr_range_size(name_start_ptr, name_end_ptr)?;

//     // On s'assure que la chain n'est pas vide
//     if len == 0 {
//         return Err(SoftwareError::EmptyValue.into());
//     }
    
//     // Convertire les pointeurs en un buffer
//     let name_buffer = unsafe {
//         core::slice::from_raw_parts(name_start_ptr, len)
//     };

//     // Convertire le buffer en une chaine de caractères UTF-8
//     str_from_fixed_buffer(name_buffer)
// }

// #[cfg(not(target_os = "none"))] // Version dummy
// pub fn device_name() -> Result<&'static str, GlobalError> {
//     Err(SoftwareError::SimulatorNotSupported.into())
// }



/// Longueur du numéro de série (sans le nul terminator)
const SERIAL_NUMBER_LENGTH: usize = 16;

/// Obtenir le numéro de série de l'appareil (device) en cours d'utilisation.
/// - Renvoie une chaine de caractères de 16 caractères en Base64.
/// - Renvoie `None` si sur **Simulateur**, numéro indisponible/invalide, ou en cas d'erreur.
/// 
/// > TIP: Pour obtenir les 12 octets bruts de l'UID hardware,
/// > décodez le résultat avec un parseur Base64 comme la crate `base64`.
#[cfg(target_os = "none")]
pub fn serial_number() -> Result<String, GlobalError> {

    // Taille du buffer pour le numéro de série (16 caractères + 1 caractère nul)
    const SERIAL_NUMBER_BUFFER_SIZE: usize = SERIAL_NUMBER_LENGTH + 1;

    // Obtenir le buffer du numéro de série via SVC
    let serial_buffer = svc_buf!(constant::SVC_SERIAL_NUMBER_COPY, SERIAL_NUMBER_BUFFER_SIZE);

    // Convertir le buffer en une chaine de caractères String et gérer les erreurs
    str_from_fixed_buffer(&serial_buffer).map(String::from)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn serial_number() -> Result<String, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir les flags de compilation du kernel.
/// - Utilisez [`KernelFlags::is_production_build()`] pour vérifier que vous êtes sur un build standard de production (sans debug ni assertions activées).
#[cfg(target_os = "none")]
pub fn kernel_flags() -> Result<KernelFlags, GlobalError> {
    let raw = svc_r0!(constant::SVC_COMPILATION_FLAGS, u32) as u16;
    Ok(KernelFlags::from_raw(raw))
}

#[cfg(not(target_os = "none"))]
pub fn kernel_flags() -> Result<KernelFlags, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir le FCC ID de l'appareil en cours d'utilisation.
/// - Renvoie [`SoftwareError::NotAvailable`] si l'appareil n'a pas de certification FCC ou indisponible.
/// 
/// ## source
/// - `FCC ID max length: grantee (max 5) + product code (max 14) = 19 chars + \0` -- Ref: [47 CFR § 2.926](https://www.law.cornell.edu/cfr/text/47/2.926)
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/src/device/userland/drivers/fcc_id.cpp
#[cfg(target_os = "none")]
pub fn fcc_id() -> Result<&'static str, GlobalError> {
    
    // Obtenir le pointeur vers le FCC ID via SVC
    let ptr = svc_r0!(constant::SVC_FCC_ID, u32) as *const u8;

    // Vérifier que le pointeur n'est pas null
    if ptr.is_null() {
        return Err(SoftwareError::NullPointer.into());
    }

    // FCC ID max = 19 chars + \0 = 20 bytes
    const FCC_ID_MAX_BUFFER: usize = 20;
    let buffer: &'static [u8] = unsafe {
        core::slice::from_raw_parts(ptr, FCC_ID_MAX_BUFFER)
    };

    // Convertire le buffer en une chaine de caractères
    let fcc_str = str_from_fixed_buffer(buffer)?;

    // "NA" = pas de certification FCC sur cet appareil
    if fcc_str == "NA" {
        return Err(SoftwareError::NotAvailable { details: "No FCC certification available for this device" }.into());
    }

    Ok(fcc_str)
}

#[cfg(not(target_os = "none"))]
pub fn fcc_id() -> Result<&'static str, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir la version du PCB de l'appareil.
#[cfg(target_os = "none")]
pub fn pcb_version() -> Result<u32, GlobalError> {
    // Appel SVC 58 pour obtenir la version du PCB
    Ok(svc_r0!(constant::SVC_PCB_VERSION, u32))
}

#[cfg(not(target_os = "none"))]
pub fn pcb_version() -> Result<u32, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir le type du dernier reset de l'appareil.
/// 
/// ## source
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/include/ion/reset.h
#[cfg(target_os = "none")]
pub fn last_reset_type() -> Result<ResetType, GlobalError> {
    // Appel SVC 59 pour obtenir le type du dernier reset
    match svc_r0!(constant::SVC_RESET_LAST_RESET_TYPE, u32) {
        0 => Ok(ResetType::Hardware),
        1 => Ok(ResetType::Software),
        _ => Err(SoftwareError::InvalidFormat { details: "unexpected ResetType value" }.into()),
    }
}

#[cfg(not(target_os = "none"))]
pub fn last_reset_type() -> Result<ResetType, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Obtenir le niveau d'autorisation du firmware en cours d'exécution.
///
/// ## source
/// - https://github.com/numworks/epsilon/blob/master/shared/ion/include/ion/authentication.h
#[cfg(target_os = "none")]
pub fn clearance_level() -> Result<ClearanceLevel, GlobalError> {
    // Appel SVC 60 pour obtenir le niveau d'autorisation du firmware
    match svc_r0!(constant::SVC_AUTHENTICATION_CLEARANCE_LEVEL, u32) {
        0 => Ok(ClearanceLevel::NumWorks),
        1 => Ok(ClearanceLevel::NumWorksAndThirdPartyApps),
        2 => Ok(ClearanceLevel::ThirdParty),
        _ => Err(SoftwareError::InvalidFormat { details: "unexpected ClearanceLevel value" }.into()),
    }
}

#[cfg(not(target_os = "none"))]
pub fn clearance_level() -> Result<ClearanceLevel, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


// TODO: Remplacer par une detetion grace a epsilon::CalculatorModel
// /// Vérifie si le slot A est actif.
// /// - Extrait du bit 3 des compilation flags.
// /// 
// /// ## source
// /// - https://github.com/numworks/epsilon/blob/master/shared/ion/src/device/userland/drivers/compilation_flags.cpp#L18-L29
// pub fn is_slot_a() -> Result<bool, GlobalError> {
//     Ok(compilation_flags()?.is_slot_a())
// }


/// Obtenir la configuration du mode examen actif.
///
/// ExamBytes n'a pas de SVC, lecture directe du secteur flash via le UserlandHeader.
/// ```
/// ┌───────────────────────────────────────┐
/// │ Layout PersistingBytes (64 kB total)  │
/// ├───────────────────╥───────────────────┤
/// │ DeviceName (1 kB) ║ ExamBytes (63 kB) │
/// └───────────────────╨───────────────────┘
/// ```
/// Le secteur ExamBytes est en append-only (on scanne de la fin vers le début)
/// pour trouver la dernière valeur écrite (non 0xFFFF).
///
/// Encodage 16 bits (exam_mode.h) :
/// ```d
///   bit 0      : configurable (PressToTest avec flags custom)
///   bits 1-14  : data (index Ruleset OU flags PressToTest)
///   bit 15     : cleared (toujours 0 pour une config valide)
/// ```
/// ## source
/// - layout: https://github.com/numworks/epsilon/blob/master/shared/ion/src/device/userland/drivers/persisting_bytes.h
/// - encodage: https://github.com/numworks/epsilon/blob/master/shared/ion/include/ion/exam_mode.h
#[cfg(target_os = "none")]
pub fn exam_mode() -> Result<ExamMode, GlobalError> {
    let sector_start = userland_header().device_name_flash_end;

    if sector_start.is_null() {
        return Err(SoftwareError::NullPointer.into());
    }

    // Secteur ExamBytes = 63 kB (persisting_bytes.h)
    const EXAM_BYTES_SECTOR_LEN: usize = (63 * 1024) / 2; // en u16

    // &[u16] → le compilateur peut vectoriser le find()
    let sector: &[u16] = unsafe {
        core::slice::from_raw_parts(sector_start as *const u16, EXAM_BYTES_SECTOR_LEN)
    };

    // Secteur append-only : la valeur courante est la dernière non-0xFFFF
    let raw = sector.iter()
        .rev()
        .copied()
        .find(|&v| v != 0xFFFF)
        .unwrap_or(0); // secteur vierge → k_defaultValue = 0 = Ruleset::Off

    ExamMode::try_from(raw)
}

#[cfg(not(target_os = "none"))]
pub fn exam_mode() -> Result<ExamMode, GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}