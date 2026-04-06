use alloc::{slice, str};
use crate::epsilon;


/// Obtenir la version du Sytem d'exploitation en cours d'utilisation.
/// 
/// Exemple :
/// ```
/// let version = eadkp::os::version();
/// assert_eq!(version, "23.12.0"); // OK
/// ```
#[cfg(target_os = "none")]
pub fn version() -> &'static str {
    unsafe {
        let version_buffer = &epsilon::kernel_header().epsilon_version; // slice de 8 bytes
        let ptr = version_buffer.as_ptr();

        let mut len = 0;

        // Trouver la taille de la chaine
        while len < version_buffer.len() && *ptr.add(len) != 0 {
            len += 1;
        }

        let slice = slice::from_raw_parts(ptr, len);

        // Convertir en str et retourner
        str::from_utf8_unchecked(slice)
    }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn version() -> &'static str {
    "X.Y.Z" 
}


/// Obtenir le hash du commit de compilation du noyau de l'os en cours d'utilisation.
/// 
/// Exemple :
/// ```
/// let hash = eadkp::os::hash_commit();
/// assert_eq!(hash, "abcdef12"); // OK
/// ```
#[cfg(target_os = "none")]
pub fn hash_commit() -> &'static str {
    unsafe {
        let hash_slice_raw = &epsilon::kernel_header().commit_hash; // slice de 8 bytes
        
        // Ne pas prendre le dernier bytes (le null terminator)
        let hash_slice = &hash_slice_raw[..hash_slice_raw.len() - 1];

        str::from_utf8_unchecked(hash_slice) // envoyer la str
    }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn hash_commit() -> &'static str {
    "abcdef12"
}