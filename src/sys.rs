
use crate::{utils, epsilon, common::Version, utils::str_from_fixed_buffer, utils::ptr_range_size};

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
    utils::str_from_fixed_buffer(hash_buffer_raw).unwrap()
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
    unsafe { ptr_range_size(start_ptr, end_ptr) }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn ext_app_ram_size() -> usize {
    100 * 1024 // 100 Ko (arbitraire)
}

