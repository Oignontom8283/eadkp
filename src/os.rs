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
    // Pour Epsilon (prévision)
    unsafe {
        let bufffer_before = epsilon::kernel_header().epsilon_version;
        let ptr = bufffer_before.as_ptr();
        let mut len = bufffer_before.len();

        while *ptr.add(len) != 0 {
            if len == 0 { panic!("Epsilon version string is not null-terminated"); } // Ou cas ou, éviter d'anlyser tout la mémoire, vaut mieux panic
            len -= 1;
        }

        let slice = slice::from_raw_parts(ptr, len);

        str::from_utf8_unchecked(slice)
    }
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn version() -> &'static str {
    "X.Y.Z" 
}


