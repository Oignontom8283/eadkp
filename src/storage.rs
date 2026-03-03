
/*!
# Storage Management Module for Epsilon Applications on Numworks Calculators.

Provides functions to read, write, and manage files in the embedded storage.

This module is originally a Rust port of `storage.c` (MIT License) from the
**NumWorks Extapp Storage** project. However, several modifications and
feature additions have been made during development. As a result, the current
implementation may differ from the original source code, both structurally and
behaviorally.

## Important Notes

- This module is designed to work in a `no_std` environment on NumWorks
    calculators.

- Supported calculator models:
    - **N0115** (tested and functional)
    - **N0120** (untested, but theoretically supported)
    - **N0110** (untested, but theoretically supported)

## Credits

Original author: **[Yaya Cout](https://framagit.org/Yaya.Cout)**

Original source file: [numworks-extapp-storage/src/storage.c](https://framagit.org/Yaya.Cout/numworks-extapp-storage/-/blob/master/src/storage.c)

---

Rust port, adaptations, and modifications by: **[Oignontom8283](https://github.com/Oignontom8283)**

## Acknowledgments

Special thanks to Yaya Cout for his remarkable engineering work on storage
manipulation, without which this module would probably never have come to life.
*/

use super::*;

// Core
#[allow(unused_imports)]
use core::ffi::{c_char};
use core::ptr;
use core::slice;
use core::str;

// Alloc
extern crate alloc; 
use alloc::vec::Vec;
#[allow(unused_imports)]
use alloc::string::{String, ToString};
#[allow(unused_imports)]
use alloc::ffi::CString;

// ============================================================================
// STORAGE OPERATIONS  
// ============================================================================


/// Vérifie que le stockage semble valide. **Ne vérifie pas l'integrité des fichiers !**
#[cfg(target_os = "none")]
pub fn is_valid_storage() -> Result<(), GlobalError> {
    let storage = epsilon::filesystem();
    storage.is_valid().map_err(|_| SoftwareError::InvalidStorage.into())
}

/// Vérifie que la string est un c string valide (pas de null byte à l'intérieur) et pas vide (un nom de fichier vide n'est pas autorisé)
fn is_valid_cstring(s: &str) -> bool {
    !s.as_bytes().contains(&0) && !s.is_empty()
}

fn find_null_terminator(start:*const u8) -> Result<*const u8, StorageError> {
    let mut len = 0;
    unsafe {
        while *start.add(len) != 0 {
            len += 1;
            
            if len > epsilon::STORAGE_FILE_MAX_NAME_LEN { // Limite de longueur pour éviter de parcourir indéfiniment en cas de corruption
                return Err(StorageError::NullTerminatorNotFound { start: start});
            }
        }
        Ok(start.add(len))
    }
}

/// Trouve la prochaine position libre dans le stockage
/// 
/// Retourne un pointeur vers le début de la fin de l'espace utilisé (le prochain enregistrement vide).
/// Si le stockage est plein, retourne l'adresse de fin du stockage utilisable
/// 
/// @unchecked
#[cfg(target_os = "none")]
fn next_free() -> *const u8 {

    let storage = epsilon::filesystem();
    let usable_end_addr = storage.usable_end_addr;
    let mut offset = storage.usable_start_addr;

    while offset < usable_end_addr {
        let size = unsafe { ptr::read_unaligned(offset as *const u16) };
        if size == 0 {
            return offset;
        }
        offset = unsafe { offset.add(size as usize) };
    }

    usable_end_addr
}


/// Trouve tous les fichiers dont le nom se termine par un suffix donné et retourne une liste de leurs noms
/// 
/// ## Exemple
/// ```
/// fn display(file: &str) { ... } // Fonction d'affichage fictive
/// 
/// let txt_files = find_files_with_suffix(".txt")?;
/// for file in txt_files {
///     display(file); // Affiche tous les fichiers se terminant par .txt
/// }
/// ```
pub fn find_files_with_suffix(suffix: &str) -> Result<Vec<&str>, GlobalError> {

    // Vérifier que le suffix est un c string valide (pas de null byte à l'intérieur) et pas vide (un suffix vide correspondrait à tous les fichiers)
    if !is_valid_cstring(suffix) {
        return Err(SoftwareError::InvalidParameter { param_name: "suffix".to_string(), details: "suffix is empty or contains null bytes".to_string() }.into());
    }

    let suffix_slice = suffix.as_bytes();
    let suffix_len = suffix_slice.len();

    let storage = epsilon::filesystem();
    let storage_start = storage.usable_start_addr;
    let storage_end = storage.usable_end_addr;

    let mut offset = storage_start;
    let mut matching_files = Vec::new();

    unsafe {
        while offset < storage_end {
            let size = ptr::read_unaligned(offset as *const u16) as usize;
            if size == 0 { break; }

            let name_ptr = offset.add(2);
            
            let mut nt_ptr = name_ptr; // Trouver l'adr du nt
            while *nt_ptr != 0 {
                nt_ptr = nt_ptr.add(1);
            }

            let name_len = nt_ptr.offset_from(name_ptr) as usize;
            
            // Vérifier que le nom est assez long pour contenir le suffix
            if name_len >= suffix_len {
                let suffix_candidate_ptr = nt_ptr.sub(suffix_len);
                let suffix_candidate = slice::from_raw_parts(suffix_candidate_ptr, suffix_len);
                
                if suffix_candidate == suffix_slice {
                    // Si trouvé une correspondance, extraire le nom complet du fichier pour le push
                    let name_slice = slice::from_raw_parts(name_ptr, name_len);
                    let name_str = str::from_utf8_unchecked(name_slice);  
                    
                    matching_files.push(name_str);
                }
            }

            offset = offset.add(size);
        }
    }

    Ok(matching_files)
}


/// Calcule l'espace libre restant dans le stockage
/// 
/// Retourne la différence entre l'adresse de fin du stockage utilisable et l'adresse de la position libre actuelle.
/// Si 0, le stockage est plein.
/// 
/// @unchecked
#[cfg(target_os = "none")]
pub fn available_space() -> usize {

    let free_addr = next_free() as usize; // Adresse de la prochaine position libre
    let usable_end = epsilon::filesystem().usable_end_addr as usize; // Adresse fin stockage utilisable (adresse du footer)

    // Retourner l'espace libre restant, en soustrayant l'adresse de la prochaine position libre de l'adresse de fin du stockage utilisable
    usable_end - free_addr
}

/// Vérifie si un fichier peut être stocké en respectant les contraintes suivantes :
/// - Espace disponible suffisant
/// - Nom du fichier ≤ 255 bytes (limite Epsilon)
/// - Nom du fichier valide (pas de null byte à l'intérieur)
/// - Taille totale (header + nom + contenu) ≤ 65535 bytes (u16 max)
/// Retourne `true` si possible, sinon `false`.
/// 
/// ## Exemple
/// ```
/// let content = b"Hello, world!";
/// let filename1 = "greeting.txt";
/// assert!(can_store(content1, filename1).is_ok()); // Vérifie que le fichier peut être stocké
/// 
/// let filename2 = "invalid\0name.txt";
/// let content2 = b"Some content";
/// assert!(can_store(content2, filename2).is_err()); // Vérifie que le nom de fichier invalide est rejeté
/// ```
/// 
/// @unchecked
#[cfg(target_os = "none")]
pub fn can_store(content: &[u8], filename: &str) -> Result<(), GlobalError> {

    let filename_size = filename.len() + 1; // +1 pour le null terminator
    let content_size = content.len();
    let total_size = 2 + filename_size + content_size; // 2 bytes pour la taille du header

    // Check que le nom peut être une c string valide
    if is_valid_cstring(filename) {
        return Err(StorageError::StorageInvalidName { length: filename_size, string: filename.to_string() }.into());
    }

    // Check nom < 255 bytes (limitation Epsilon)
    if filename_size > u8::MAX as usize {
        return Err(StorageError::StorageInvalidName { length: filename_size, string: filename.to_string() }.into());
    }

    // Check que le content n'est pas vide
    if content.is_empty() {
        return Err(StorageError::FileContentEmpty.into());
    }

    // Check total_size < 65535 bytes (limitation du header sur 2 bytes)
    if total_size > u16::MAX as usize {
        return Err(StorageError::FileTooLarge { max_size: u16::MAX as usize, actual_size: total_size }.into());
    }

    if available_space() >= total_size {
        Ok(())
    } else {
        Err(StorageError::StorageOverflow { available: available_space(), needed: total_size }.into())
    }
}

#[cfg(not(target_os = "none"))]
pub fn can_store(_content: &[u8], _filename: &str) -> Result<(), GlobalError> {
    Err(SoftwareError::SimulatorNotSupported)
}


/// Écrit un nouveau fichier dans le stockage
/// 
/// ## Warning
/// Le contenu écrit doit être en bytes bruts. Pour écrire du texte, utilisez `write_file_string` qui gère l'encodage UTF-8 et le null terminator. 
/// 
/// Format: \[2 bytes taille\] \[nom\0\] \[contenu\]
#[cfg(target_os = "none")]
pub fn file_write_raw(filename: &str, content: &[u8]) -> Result<(), GlobalError> {
    
    is_valid_storage()?;
    can_store(content, filename)?;

    // Vérifier que le nom du fichier est valide
    if !is_valid_cstring(filename) {
        return Err(StorageError::StorageInvalidName { length: filename.len(), string: filename.to_string() }.into());
    }
    
    let write_pos = next_free() as *mut u8; // adr du nouveau fichier (début)

    let size = (2 + filename.len() + 1 + content.len()) as u16; // Total size (header + nom + term + contenu)

    unsafe {
        let dest_header_ptr = write_pos as *mut u16;
        let dest_name_slice = slice::from_raw_parts_mut((dest_header_ptr as *mut u8).add(2), filename.len());
        let dist_term_ptr = dest_name_slice.as_mut_ptr().add(filename.len());
        let dest_content_slice = slice::from_raw_parts_mut(dist_term_ptr.add(1), content.len());


        // Écrire le header
        ptr::write_unaligned(dest_header_ptr, size);

        // écrire le nom du fichier (sans null terminator)
        dest_name_slice.copy_from_slice(filename.as_bytes());

        // Écrire le null terminator
        *dist_term_ptr = 0;
        
        // Écrire le contenu du fichier
        dest_content_slice.copy_from_slice(content);
    }

    Ok(())
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_write_raw(_filename: &str, _content: &[u8]) -> Result<(), GlobalError> {
    Ok(())
}


/// Lit un fichier et retourne un pointeur vers son contenu
#[cfg(target_os = "none")]
pub unsafe fn file_read_raw(filename: &str) -> Result<&[u8], GlobalError> {

    is_valid_storage()?;
    
    // Pas besouin de check le nom, car au pire, on ne trouve pas le fichier, grace a la comparaison de bytes

    let filename_slice = filename.as_bytes(); // Obtenir les octets du nom du fichier (sans null terminator)
    let filename_len = filename_slice.len();

    let storage = epsilon::filesystem();
    let storage_start = storage.usable_start_addr;
    let storage_end = storage.usable_end_addr;

    let mut offset = storage_start;

    unsafe {
        while offset < storage_end {
            
            let size = ptr::read_unaligned(offset as *const u16) as usize;
            if size == 0 { break; } // Fin des enregistrements

            let name_ptr = offset.add(2);
            let name_candidate = slice::from_raw_parts(name_ptr, filename_len);
            let name_null_terminator = *name_ptr.add(filename_len); // Octet juste après le nom candidate, doit être le null terminator

            if name_candidate == filename_slice && name_null_terminator == 0 { // Fichier trouvé !
                let content_ptr = name_ptr.add(filename_len + 1); // +1 pour sauter le null terminator
                let content_size = size - 2 - (filename_len + 1); // Taille totale - header - nom  

                return Ok(slice::from_raw_parts(content_ptr, content_size)); // Retourner une slice vers le contenu du fichier
            }

            offset = offset.add(size); // Passer à l'enregistrement suivant
        }
    }

    Err(StorageError::FileNotFound.into())

    // let filename_cstr = to_cstring(filename)?;
    // let filename_ptr = filename_cstr.as_ptr();

    // unsafe {
    //     let storage_addr = address();
    //     let mut offset = (storage_addr as *mut u8).add(4); // Skip magic number
    //     let end_addr = (storage_addr + size()) as *mut u8;
        
    //     // Vérifier que le stockage est valide avec info sur le magic number
    //     let magic_expected = 0xBADD0BEEu32.swap_bytes();
    //     let magic_found = ptr::read_unaligned(storage_addr as *const u32);
    //     if magic_found != magic_expected {
    //         return Err(StorageError::InvalidMagicNumber { 
    //             expected: magic_expected, 
    //             found: magic_found 
    //         });
    //     }
        
    //     // Parcourir tous les enregistrements
    //     while offset < end_addr {
    //         let size = ptr::read_unaligned(offset as *const u16);
    //         if size == 0 { break; } // Fin des enregistrements
            
    //         let name = offset.add(2);
    //         if strcmp(name, filename_ptr) { // Fichier trouvé
    //             let name_size = strlen(name) + 1;
    //             let content_size = size as usize - 2 - name_size;
    //             return Ok((offset.add(2 + name_size), content_size));
    //         }
            
    //         offset = offset.add(size as usize);
    //     }
        
    //     Err(StorageError::FileNotFound)
    // }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_read_raw(_filename: &str) -> Result<&[u8], GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Vérifie si un fichier existe dans le stockage
#[cfg(target_os = "none")]
pub fn file_exists(filename: &str) -> Result<bool, GlobalError> {
    match unsafe { file_read_raw(filename) } {
        Ok(_) => Ok(true),
        Err(GlobalError::Storage(StorageError::FileNotFound)) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub fn file_exists(_filename: &str) -> Result<bool, GlobalError> {
    Ok(false)
}


// ! Tout ce qui est en dessous n'est pas encore refactoriser


/// Supprime un fichier du stockage
#[cfg(target_os = "none")]
pub unsafe fn file_erase(filename: &str) -> Result<()> {

    let filename_cstr = to_cstring(filename)?;
    let filename_ptr = filename_cstr.as_ptr();

    unsafe {
        let storage_addr = address();
        let mut offset = (storage_addr as *mut u8).add(4);
        let end_addr = (storage_addr + size()) as *mut u8;
        
        // Vérifier que le stockage est valide
        let magic_expected = 0xBADD0BEEu32.swap_bytes();
        let magic_found = ptr::read_unaligned(storage_addr as *const u32);
        if magic_found != magic_expected {
            return Err(StorageError::InvalidMagicNumber { 
                expected: magic_expected, 
                found: magic_found 
            });
        }
        
        // Chercher le fichier
        while offset < end_addr {
            let size = ptr::read_unaligned(offset as *const u16);
            if size == 0 { break; }
            
            let name = offset.add(2);
            if strcmp(name, filename_ptr) { // Fichier trouvé
                // Déplacer tous les enregistrements suivants pour combler le trou
                let next_free_pos = next_free() as *mut u8;
                let move_size = next_free_pos.offset_from(offset) as usize;
                memmove(offset, offset.add(size as usize), move_size);
                
                // Nettoyer l'espace libéré
                memset(next_free_pos.sub(size as usize), 0, size as usize);
                return Ok(());
            }
            
            offset = offset.add(size as usize);
        }
        
        Err(StorageError::FileNotFound)
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_erase(_filename: &str) -> Result<()> {
    Ok(())
}

/// Écrit une string dans le stockage (avec encodage UTF-8 et null terminator)
#[cfg(target_os = "none")]
pub unsafe fn file_write_string(filename: &str, content: &str) -> Result<()> {
    let content_cstr = to_cstring(content)?;
    let content_bytes = content_cstr.as_slice(); // Obtenir les octets, y compris le null terminator

    unsafe { file_write_raw(filename, content_bytes) }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_write_string(_filename: &str, _content: &str) -> Result<()> {
    Ok(())
}

#[cfg(target_os = "none")]
pub unsafe fn file_read_string(filename: &str) -> Result<&'static str> {
    // Obtenir les bytes bruts du fichier
    let (content_ptr, content_len) = unsafe { file_read_raw(filename)? };

    // Convertir les bytes en slice
    let content_slice = unsafe { core::slice::from_raw_parts(content_ptr, content_len) };

    // Vérifier la présence du null terminator à la fin et que ce ne soit pas vide.
    if content_slice.is_empty() || content_slice.last() != Some(&0) {
        return Err(StorageError::StorageInvalidName);
    }

    // Convertir la slice en C string
    let cstr_ptr = content_slice.as_ptr();

    // Convertir la C string en string Rust
    return cstring_to_str(cstr_ptr)
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_read_string(_filename: &str) -> Result<&'static str> {
    Ok("Dummy content")
}

