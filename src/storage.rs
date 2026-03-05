
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

/// Trouve le pointeur vers le null terminator d'une string commençant à `start`,
/// sans dépasser `max` dans un maxium de `epsilon::STORAGE_FILE_MAX_NAME_LEN`.
fn strnend(start:*const u8, max:*const u8) -> Result<*const u8, StorageError> {
    unsafe  {
        // limite d'Epsilon et l'imite donnée
        let len = (max.offset_from(start) as usize).min(epsilon::STORAGE_FILE_MAX_NAME_LEN);
        
        for offset in 0..=len {
            if *start.add(offset) == 0 {
                return Ok(start.add(offset));
            }
        }
        Err(StorageError::NullTerminatorNotFound { start })
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


/// Structure pour représenter un fichier.
struct FileEntry {
    size: usize,
    ptr: *const u8,
    name: *const u8,
    content: *const u8,
    content_size: usize,
}

/// Trouve un fichier par son nom et retourne une structure contenant des pointeurs vers son header, son nom et son contenu, ainsi que sa taille.
fn _find_file(filename: &str) -> Result<FileEntry, StorageError> {
    let filename_slice = filename.as_bytes();
    let filename_len = filename_slice.len();

    let storage = epsilon::filesystem();
    let storage_start = storage.usable_start_addr;
    let storage_end = storage.usable_end_addr;

    let mut offset = storage_start;

    unsafe {
        while offset < storage_end {
            let size = ptr::read_unaligned(offset as *const u16) as usize;
            if size == 0 { break; }

            let name_ptr = offset.add(2);
            let name_candidate = slice::from_raw_parts(name_ptr, filename_len);
            let name_null_terminator = *name_ptr.add(filename_len);

            if name_candidate == filename_slice && name_null_terminator == 0 {
                let content_ptr = name_ptr.add(filename_len + 1);
                let content_size = size - 2 - (filename_len + 1);
                return Ok(FileEntry { size: size, ptr: offset, name: name_ptr, content: content_ptr, content_size });
            }

            offset = offset.add(size);
        }
    }

    Err(StorageError::FileNotFound)
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
            
            // let mut nt_ptr = name_ptr; // Trouver l'adr du nt
            // while *nt_ptr != 0 {
            //     nt_ptr = nt_ptr.add(1);
            // }
            let nt_ptr = strnend(name_ptr, offset.add(size))?;

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

    // Vérifier que le storage est valide
    is_valid_storage()?;
    
    // Localiser le fichier via une recherche directe
    let file_view = _find_file(filename)?;

    // Retourner une slice pointant vers le contenu du fichier
    return Ok(slice::from_raw_parts(file_view.content, file_view.content_size));
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


/// Efface un fichier du stockage (**Irréversible!**)
#[cfg(target_os = "none")]
pub unsafe fn file_erase(filename: &str) -> Result<(), GlobalError> {

    is_valid_storage()?;

    let file_to_erase = _find_file(filename)?;

    let free_space_before_deletion = next_free();
    let next_file_pos = file_to_erase.ptr.add(file_to_erase.size);

    // Déplacer tout depuis la fin du fichier supprimé jusqu'à la fin de l'espace utilisé vers le début du fichier supprimé pour combler le trou
    // Normalement ptr::copy devrait automatiquement détecter le chvauchement et faire la copie de manière sûre.
    ptr::copy(
        next_file_pos,
        file_to_erase.ptr as *mut u8,
        free_space_before_deletion.offset_from(next_file_pos) as usize
    );

    // Nettoyer l'espace libéré à la fin du stockage (obligatoire)
    ptr::write_bytes(
        free_space_before_deletion.sub(file_to_erase.size) as *mut u8,
        0,
        file_to_erase.size
    );

    Ok(())
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

