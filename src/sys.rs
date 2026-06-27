
use crate::{utils, epsilon, common::Version};

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
    let version_str = utils::str_from_fixed_buffer(version_buffer_raw).unwrap();

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