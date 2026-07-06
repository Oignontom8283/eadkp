
#[cfg(target_os = "none")]
use crate::{GlobalError, SoftwareError};
use crate::{
    utils::{ptr_range_size_unchecked, ptr_range_size, str_from_fixed_buffer},
    common::{self, Version},
    alloc::string::String,
    epsilon,
    svc_buf,
};

/// Obtenir la version du Sytem d'exploitation en cours d'utilisation (version du kernel).
/// ```
/// let version = eadkp::sys::version();
/// assert!(!version.is_empty()); // ex: 24.2.3
/// ```
#[cfg(target_os = "none")]
pub fn version() -> Version {
    
    // Obtenir le buffer de version du kernel
    let version_buffer_raw = &epsilon::kernel_header().epsilon_version; // 8 bytes

    // Extraire la chaine de caractères
    let version_str = str_from_fixed_buffer(version_buffer_raw).unwrap();

    // Convertir la chaine de caractères en obj Version
    Version::parse(version_str).unwrap()
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn version() -> Version {
    Version::parse("1.2.3").unwrap()
}


/// Obtenir le hash du commit de compilation du noyau de l'os en cours d'utilisation.
/// ```
/// let hash = eadkp::sys::hash_commit();
/// assert!(!hash.is_empty()); // ex: abcdef12
/// ```
#[cfg(target_os = "none")]
pub fn hash_commit() -> &'static str {

    // Obternir le buffer du hash du commit du kernel
    let hash_buffer_raw = &epsilon::kernel_header().commit_hash; // 8 bytes

    // Extraire la chaine de caractères
    str_from_fixed_buffer(hash_buffer_raw).unwrap()
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn hash_commit() -> &'static str {
    "abcdef12"
}


/// Obtenir la version du kernel attendue par le UserLand.
/// - Donnée interne utilisée par le UserLand pour vérifier ça compatibilité avec le kernel. Utile pour les UserLand customisés principalement.
#[cfg(target_os = "none")]
pub fn expected_version() -> Version {

    // Obtenir le buffer de la version attendue du kernel
    let expected_version_buffer_raw = &epsilon::userland_header().expected_epsilon_version; // 8 bytes

    // Extraire la chaine de caractères
    let expected_version_str = str_from_fixed_buffer(expected_version_buffer_raw).unwrap();

    // Convertir la chaine de caractères en obj Version
    Version::parse(expected_version_str).unwrap()
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn expected_version() -> Version {
    Version::parse("1.2.3").unwrap()
}



/// Obtenir la taille du système de fichiers (storage).
/// - ⚠️ Comprend **TOUT** la zone du FS, y compris les zone non utilisables pour stocker des fichiers (ex: magic number).
/// - Taille en bytes (octets).
#[cfg(target_os = "none")]
pub fn filesystem_size() -> usize {
    epsilon::userland_header().storage_size_ram as usize
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn filesystem_size() -> usize {
    42 * 1024 // 42 Ko (la taille du FS normalement, arbitraire)
}


/// Obtenir la taille de la zone mémoire allouée aux applications externes (RAM).
/// - Taille en bytes (octets).
#[cfg(target_os = "none")]
pub fn ext_app_ram_size() -> usize {
    let start_ptr = epsilon::userland_header().external_apps_ram_start;
    let end_ptr = epsilon::userland_header().external_apps_ram_end;

    // Calculer la taille de la plage mémoire
    unsafe { ptr_range_size_unchecked(start_ptr, end_ptr) }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_ram_size() -> usize {
    100 * 1024 // 100 Ko (arbitraire)
}


/// Obtenir la taille de la zone de mémoire flash allouée aux stockages des binaires des applications externes.
/// - Taille en bytes (octets).
#[cfg(target_os = "none")]
pub fn ext_app_flash_size() -> usize {
    let start_ptr = epsilon::userland_header().external_apps_flash_start;
    let end_ptr = epsilon::userland_header().external_apps_flash_end;

    // Calculer la taille de la plage mémoire
    unsafe { ptr_range_size_unchecked(start_ptr, end_ptr) }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_flash_size() -> usize {
    (2.5 * 1024.0 * 1024.0) as usize // 2.5 Mo (la taille sur ma calulartrice, arbitraire)
}


/// Obtenir le nom de l'appareil (device) en cours d'utilisation.
/// - Renvoie "Unknown Device" si le nom n'est pas disponible ou invalide.
/// 
/// ## Attention
/// Il est fortement possible que vous n'arriviez pas a obtenir le nom de l'appareil, pour des raisons qui m'échappent.
#[cfg(target_os = "none")]
pub fn device_name() -> Result<&'static str, GlobalError> {
    
    // Obtenir les pointeurs vers le début et la fin du nom de l'appareil
    let name_start_ptr = epsilon::userland_header().device_name_flash_start;
    let name_end_ptr = epsilon::userland_header().device_name_flash_end;

    // Calculer la taille et on s'assure que les pointeurs sont valides
    let len = ptr_range_size(name_start_ptr, name_end_ptr)?;

    // On s'assure que la chain n'est pas vide
    if len == 0 {
        return Err(SoftwareError::EmptyValue.into());
    }
    
    // Convertire les pointeurs en un buffer
    let name_buffer = unsafe {
        core::slice::from_raw_parts(name_start_ptr, len)
    };

    // Convertire le buffer en une chaine de caractères UTF-8
    str_from_fixed_buffer(name_buffer)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn device_name() -> Result<&'static str, GlobalError> {
    Ok("Simulated Device")
}



/// Longueur du numéro de série (sans le nul terminator)
const SERIAL_NUMBER_LENGTH: usize = 16;

/// Obtenir le numéro de série de l'appareil (device) en cours d'utilisation.
/// - Renvoie une chaine de caractères de 16 caractères en Base64.
/// - Renvoie `None` si sur **Simulateur**, numéro indisponible/invalide, ou en cas d'erreur.
/// 
/// > TIP: Pour obtenir les 12 octets bruts de l'UID hardware,
/// > décodez le résultat avec un parseur Base64 comme la crate `base64`.
#[cfg(target_os = "none")]
pub fn serial_number() -> Option<String> {

    // Taille du buffer pour le numéro de série (16 caractères + 1 caractère nul)
    const SERIAL_NUMBER_BUFFER_SIZE: usize = SERIAL_NUMBER_LENGTH + 1;

    // Obtenir le buffer du numéro de série via SVC
    let serial_buffer = svc_buf!(common::SVC_SERIAL_NUMBER_COPY, SERIAL_NUMBER_BUFFER_SIZE);

    // Convertir le buffer en une chaine de caractères String et gérer les erreurs
    str_from_fixed_buffer(&serial_buffer)
        .ok()
        .map(String::from)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn serial_number() -> Option<String> {
    None
}