use crate::{SoftwareError, GlobalError};


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
/// - Renvoie 0 si les pointeurs sont invalides ou dans le mauvais ordre.
/// 
/// ## Exemple
/// ```
/// let start_ptr = 0x1000 as *const u8; // 4096
/// let end_ptr = 0x2000 as *const u8; // 8192
/// 
/// let size = ptr_range_size(start_ptr, end_ptr);
/// 
/// assert_eq!(size, 0x1000); // 4096
/// ```
pub unsafe fn ptr_range_size(start: *const u8, end: *const u8) -> usize {
    if start.is_null() || end.is_null() || end < start {
        return 0;
    }
    end.offset_from(start) as usize
}