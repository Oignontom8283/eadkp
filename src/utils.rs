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