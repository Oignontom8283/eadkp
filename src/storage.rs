
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
use core::ffi::{CStr, c_char};
use core::ptr;
use heapless;
use ::alloc::*;
use ::alloc::ffi::CString;


// ============================================================================
// STORAGE OPERATIONS  
// ============================================================================


/// Trouve la prochaine position libre dans le stockage
/// 
/// Retourne un pointeur vers le début de la fin de l'espace utilisé (le prochain enregistrement vide).
/// Si le stockage est plein, retourne l'adresse de fin du stockage utilisable
/// 
/// Faire : `usable_end_addr - next_free()` pour obtenir l'espace libre restant, si 0, stockage plein.
#[cfg(target_os = "none")]
fn next_free() -> *const u8 {
    // unsafe {
    //     let storage_addr = address();
    //     let mut offset = (storage_addr as *mut u8).add(4);
    //     let end_addr = (storage_addr + size()) as *mut u8;
        
    //     // Vérifier validité mais ignorer l'erreur (retourne null si invalide)
    //     if is_valid(storage_addr as *const u32).is_err() { return ptr::null(); }
        
    //     // Parcourir jusqu'à trouver un enregistrement vide (size=0)
    //     while offset < end_addr {
    //         let size = ptr::read_unaligned(offset as *const u16);
    //         if size == 0 { return offset as *const u32; }
    //         offset = offset.add(size as usize);
    //     }
        
    //     end_addr as *const u32
    // }

    let storage = epsilon::filesystem();
    let mut offset = storage.usable_start_addr;

    if !storage.is_valid() {
        panic!("Invalid filesystem detected at address {:p}", storage.storage_start_addr);
    }

    while offset < storage.usable_end_addr {
        let size = unsafe { ptr::read_unaligned(offset as *const u16) };
        if size == 0 {
            return offset;
        }
        offset = unsafe { offset.add(size as usize) };
    }

    storage.usable_end_addr
}

/// Calcule l'espace libre restant dans le stockage
/// 
/// Retourne la différence entre l'adresse de fin du stockage utilisable et l'adresse de la position libre actuelle.
/// Si 0, le stockage est plein.
#[cfg(target_os = "none")]
pub fn available_space() -> usize {

    let free_addr = next_free() as usize; // Adresse de la prochaine position libre
    let usable_end = filesystem().usable_end_addr as usize; // Adresse de fin du stockage utilisable (adresse du footer)

    // Retourner l'espace libre restant, en soustrayant l'adresse de la prochaine position libre de l'adresse de fin du stockage utilisable
    usable_end - free_addr
}

/// Vérifie que :
/// - Qu'il y a assez d'espace disponible pour stocker le fichier
/// - Que la taille du nom du fichier ne dépasse pas 255 bytes (limitation imposée par Epsilon)
/// - Que la taille totale du fichier (header + nom + contenu) ne dépasse pas la taille maximum de `2^16 - 1` (=65535 bytes)
/// 
/// Retourne `true` si le fichier peut être stocké, `false` sinon.
/// 
/// La taille maximum du fichier est limité par la taille du header qui est sur 2 octects (u16),
/// et le nom du fichier est limité à 255 bytes pour des raisons de compatibilité avec Epsilon.
#[cfg(target_os = "none")]
pub fn can_store(content_size: usize, filename_size: usize) -> bool {
    // Calculer la taille totale nécessaire pour stocker le fichier (header + nom + contenu)
    let total_size = 2 + filename_size + content_size; // 2 bytes pour la taille du header

    // Vérifier que la taille du nom du fichier ne dépasse pas 255, limitation imposée par Epsilon.
    // Par soucis de compatibilité avec Epsilon, on limite a 255 bytes pour le nom du fichier.
    if filename_size > u8::MAX as usize {
        return false;
    }

    // Vérifier que la taille totale ne dépasse pas la capacité maximale du header (2 bytes, soit 65535)
    if total_size > u16::MAX as usize {
        return false; // La taille totale dépasse la capacité maximale du header (2 bytes), donc impossible à stocker;
    }

    // Vérifier si l'espace disponible est suffisant
    available_space() >= total_size
}

/// Écrit un nouveau fichier dans le stockage
/// 
/// ## Warning
/// Le contenu écrit doit être en bytes bruts. Pour écrire du texte, utilisez `write_file_string` qui gère l'encodage UTF-8 et le null terminator. 
/// 
/// Format: \[2 bytes taille\] \[nom\0\] \[contenu\]
#[cfg(target_os = "none")]
pub unsafe fn file_write_raw(filename: &str, content: &[u8]) -> Result<()> {

    // Convertir le nom du fichier en C string (UTF-8 avec null terminator)
    let filename_cstring = CString::new(filename)
        .map_err(|_| StorageError::StorageInvalidName)?; // Erreur si le nom contient un null byte ou est trop long
    
    let content_ptr = content.as_ptr(); // Pointeur vers le contenu à écrire
    let content_len = content.len();

    let free_pos = next_free(); // Trouver la position libre dans le stockage
    let free_space = available_space();

    // Vérifier que le fichier peut être stocker (taille max du nom, taille max, espace disponible)
    if !can_store(content_len, filename_cstring.as_bytes_with_nul().len()) {
        return Err(StorageError::StorageOverflow { 
            available: free_space,
            needed: 2 + filename_cstring.as_bytes_with_nul().len() + content_len,
        });
    }

    let write_pos = free_pos as *mut u8; // Adresse où écrire le nouvel enregistrement
    let total_size = 2 + filename_cstring.as_bytes_with_nul().len() + content_len; // Taille totale de l'enregistrement (header + nom + contenu)
    
    let size_addr = write_pos as *mut u16; // Adresse où écrire la taille du header (2 bytes)
    let name_addr = unsafe { write_pos.add(2) }; // Adresse où écrire le nom du fichier (juste après la taille)
    let content_addr = unsafe { name_addr.add(filename_cstring.as_bytes_with_nul().len()) }; // Adresse où écrire le contenu (juste après le nom)

    unsafe {
        // Écrire le header (taille totale sur 2 bytes)
        ptr::write_unaligned(size_addr, total_size as u16);
        // Écrire le nom du fichier (avec null terminator)
        memcpy(name_addr, filename_cstring.as_ptr(), filename_cstring.as_bytes_with_nul().len())?;
        // Écrire le contenu
        memcpy(content_addr, content_ptr, content_len)?;
    }

    Ok(())

    // let filename_cstr = to_cstring(filename)?;
    // let filename_ptr = filename_cstr.as_ptr();
    // let filename_len = filename_cstr.len(); // Avec le null terminator !

    // let content_ptr = content.as_ptr();
    // let content_len = content.len();

    // unsafe {
    //     // Trouver la position libre dans le stockage
    //     let free_pos = next_free();
    //     if free_pos.is_null() { 
    //         return Err(StorageError::StorageFull); 
    //     }
        
    //     // Calculer la taille totale nécessaire
    //     let total_size = 2 + filename_len + content_len; // taille_header + nom (avec null terminator) + contenu
    //     let storage_end = (address() + size()) as usize;
    //     let free_pos_usize = free_pos as usize;
    //     let needed_end = free_pos_usize + total_size;
        
    //     // Vérifier qu'on a assez d'espace avec info détaillée
    //     if needed_end > storage_end { 
    //         return Err(StorageError::StorageOverflow { 
    //             available: storage_end.saturating_sub(free_pos_usize),
    //             needed: total_size,
    //         }); 
    //     }
        
    //     // Écrire le header (taille totale sur 2 bytes)
    //     let write_pos = free_pos as *mut u8;
    //     ptr::write_unaligned(write_pos as *mut u16, total_size as u16);
        
    //     // Écrire le nom du fichier (avec null terminator)
    //     let name_pos = write_pos.add(2);
    //     memcpy(name_pos, filename_ptr, filename_len);
        
    //     // Écrire le contenu
    //     let content_pos = name_pos.add(filename_len);
    //     memcpy(content_pos, content_ptr, content_len);
        
    //     // Nettoyer le reste (marquer la fin des enregistrements)
    //     let cleanup_pos = content_pos.add(content_len);
    //     let cleanup_size = ((address() + size()) as *mut u8).offset_from(cleanup_pos) as usize;
    //     memset(cleanup_pos, 0, cleanup_size);
        
    //     Ok(())
    // }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_write_raw(_filename: &str, _content: &[u8]) -> Result<()> {
    Ok(())
}


/// Lit un fichier et retourne un pointeur vers son contenu
#[cfg(target_os = "none")]
pub unsafe fn file_read_raw(filename: &str) -> Result<(*const u8, usize)> {

    let filename_cstr = to_cstring(filename)?;
    let filename_ptr = filename_cstr.as_ptr();

    unsafe {
        let storage_addr = address();
        let mut offset = (storage_addr as *mut u8).add(4); // Skip magic number
        let end_addr = (storage_addr + size()) as *mut u8;
        
        // Vérifier que le stockage est valide avec info sur le magic number
        let magic_expected = 0xBADD0BEEu32.swap_bytes();
        let magic_found = ptr::read_unaligned(storage_addr as *const u32);
        if magic_found != magic_expected {
            return Err(StorageError::InvalidMagicNumber { 
                expected: magic_expected, 
                found: magic_found 
            });
        }
        
        // Parcourir tous les enregistrements
        while offset < end_addr {
            let size = ptr::read_unaligned(offset as *const u16);
            if size == 0 { break; } // Fin des enregistrements
            
            let name = offset.add(2);
            if strcmp(name, filename_ptr) { // Fichier trouvé
                let name_size = strlen(name) + 1;
                let content_size = size as usize - 2 - name_size;
                return Ok((offset.add(2 + name_size), content_size));
            }
            
            offset = offset.add(size as usize);
        }
        
        Err(StorageError::FileNotFound)
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_read_raw(_filename: &str) -> Result<(*const u8, usize)> {
    Err(StorageError::InvalidStorage)
}


/// Vérifie si un fichier existe dans le stockage
#[cfg(target_os = "none")]
pub fn file_exists(filename: &str) -> bool {
    match unsafe { file_read_raw(filename) } {
        Ok(_) => true,
        Err(StorageError::FileNotFound) => false,
        Err(_) => false,
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_exists(_filename: &str) -> bool {
    false
}


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

