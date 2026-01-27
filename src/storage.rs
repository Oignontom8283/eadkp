

/*!
# Module de gestion du stockage pour applications Epsilon sur calculatrice Numworks.

Fournit des fonctions pour lire, écrire et gérer des fichiers dans le stockage embarqué.

Ce module est à l’origine un portage en Rust de `storage.c` (Sous licence MIT) du projet
**NumWorks Extapp Storage**. Toutefois, plusieurs modifications et ajouts de
fonctionnalités ont été apportés au fil du développement. En conséquence,
l’implémentation actuelle peut différer du code source original, tant sur le
plan structurel que comportemental.

## Remarques importantes

- Ce module est conçu pour fonctionner dans un environnement `no_std` sur
  calculatrice NumWorks.

- Les modèles de calculatrices supportés sont :
  - **N0115** (testé et fonctionnel)
  - **N0120** (non testé, mais théoriquement supporté)
  - **N0110** (non testé, mais théoriquement supporté)

## Crédits

Auteur original: **[Yaya Cout](https://framagit.org/Yaya.Cout)**

Fichier source original: [numworks-extapp-storage/src/storage.c](https://framagit.org/Yaya.Cout/numworks-extapp-storage/-/blob/master/src/storage.c)

---

Portage en Rust Original, adaptations et modifications original: **[Oignontom8283](https://github.com/Oignontom8283)**

## Remerciements

Tout mes remerciements à Yaya Cout pour son travail d'ingénierie remarquable sur la manipulation du stockage,
sans lequel ce module n'aurait probablement jamais vu le jour.
*/

use core::ptr;
use heapless;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Stockage invalide ou corrompu (magic number incorrect)
    InvalidStorage,
    /// Fichier non trouvé dans le stockage
    FileNotFound,
    /// Pas assez d'espace disponible pour l'écriture
    InsufficientSpace,
    /// Nom de fichier invalide ou trop long (max 256 bytes)
    InvalidInput,
    /// Magic number invalide à l'adresse de stockage
    InvalidMagicNumber { expected: u32, found: u32 },
    /// Stockage plein, position libre null
    StorageFull,
    /// Dépassement de la taille du stockage
    StorageOverflow { available: usize, needed: usize },
}

pub type Result<T> = core::result::Result<T, StorageError>;

/// Convertit une string Rust en C string (avec null terminator)
pub fn to_cstring(s: &str) -> Result<heapless::Vec<u8, 256>> {
    let mut cstr = heapless::Vec::new();
    cstr.extend_from_slice(s.as_bytes()).map_err(|_| StorageError::InvalidInput)?;
    cstr.push(0).map_err(|_| StorageError::InvalidInput)?; // Ajouter \0
    Ok(cstr)
}

/// Convertit une C string en string Rust
fn cstring_to_str(s: *const u8) -> Result<&'static str> {
    unsafe {
        let len = strlen(s);
        let slice = core::slice::from_raw_parts(s, len);
        match core::str::from_utf8(slice) {
            Ok(str_ref) => Ok(str_ref),
            Err(_) => Err(StorageError::InvalidInput),
        }
    }
}

/// Calcule la longueur d'une C string (sans le \0)
unsafe fn strlen(s: *const u8) -> usize {
    let mut len = 0;
    let mut p = s;
    while unsafe { *p != 0 } { // Chercher le null terminator
        len += 1;
        p = unsafe { p.add(1) };
    }
    len
}

/// Compare deux C strings
unsafe fn strcmp(s1: *const u8, s2: *const u8) -> bool {
    let mut p1 = s1;
    let mut p2 = s2;

    while unsafe { *p1 != 0 && *p1 == *p2 } { // Comparer jusqu'au null terminator ou différence
        // Avancer les pointeurs
        p1 = unsafe { p1.add(1) };
        p2 = unsafe { p2.add(1) };
    }
    unsafe { ((*p1 as i32) - (*p2 as i32)) == 0 } // Différence ASCII, si 0, ce sont les mêmes caractères, donc on est arrivé à la fin des deux chaînes en même temps

}
/// Copie n bytes de src vers dest (zones non chevauchantes)
/// 
/// **Comportement INDÉFINI en cas de CHEVAUCHEMENT des zones !** NON SÉCURISÉ .
#[cfg(target_os = "none")]
unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    unsafe { ptr::copy_nonoverlapping(src, dest, n) }
}

/// Copie `n` bytes de `src:*` vers `dest:*` (zones peuvent chevaucher)
/// 
/// **Comportement défini même en cas de chevauchement:** La copie se fait par une mémoire tampon.
#[cfg(target_os = "none")]
unsafe fn memmove(dest: *mut u8, src: *const u8, n: usize) {
    unsafe { ptr::copy(src, dest, n) }
}

/// Remplit n bytes avec la valeur c
#[cfg(target_os = "none")]
unsafe fn memset(s: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        unsafe { *s.add(i) = c };
    }
}

// ============================================================================
// STORAGE OPERATIONS  
// ============================================================================

/// Écrit un fichier dans le stockage
/// 
/// ## attention:
/// Le contenu écrit doit être en bytes bruts. Pour écrire du texte, utilisez `write_file_string` qui gère l'encodage UTF-8 et le null terminator. 
/// 
/// Format: \[2 bytes taille\] \[nom\0\] \[contenu\]
#[cfg(target_os = "none")]
pub unsafe fn file_write_raw(filename: &str, content: &[u8]) -> Result<()> {

    let filename_cstr = to_cstring(filename)?;
    let filename_ptr = filename_cstr.as_ptr();
    let filename_len = filename_cstr.len(); // Avec le null terminator !

    let content_ptr = content.as_ptr();
    let content_len = content.len();

    unsafe {
        // Trouver la position libre dans le stockage
        let free_pos = next_free();
        if free_pos.is_null() { 
            return Err(StorageError::StorageFull); 
        }
        
        // Calculer la taille totale nécessaire
        let total_size = 2 + filename_len + content_len; // taille_header + nom (avec null terminator) + contenu
        let storage_end = (address() + size()) as usize;
        let free_pos_usize = free_pos as usize;
        let needed_end = free_pos_usize + total_size;
        
        // Vérifier qu'on a assez d'espace avec info détaillée
        if needed_end > storage_end { 
            return Err(StorageError::StorageOverflow { 
                available: storage_end.saturating_sub(free_pos_usize),
                needed: total_size,
            }); 
        }
        
        // Écrire le header (taille totale sur 2 bytes)
        let write_pos = free_pos as *mut u8;
        ptr::write_unaligned(write_pos as *mut u16, total_size as u16);
        
        // Écrire le nom du fichier (avec null terminator)
        let name_pos = write_pos.add(2);
        memcpy(name_pos, filename_ptr, filename_len);
        
        // Écrire le contenu
        let content_pos = name_pos.add(filename_len);
        memcpy(content_pos, content_ptr, content_len);
        
        // Nettoyer le reste (marquer la fin des enregistrements)
        let cleanup_pos = content_pos.add(content_len);
        let cleanup_size = ((address() + size()) as *mut u8).offset_from(cleanup_pos) as usize;
        memset(cleanup_pos, 0, cleanup_size);
        
        Ok(())
    }
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
        return Err(StorageError::InvalidInput);
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

// ============================================================================
// HARDWARE INTERFACE
// ============================================================================

/// Retourne l'adresse de base du stockage
#[cfg(target_os = "none")]
unsafe fn address() -> u32 {
    unsafe { ptr::read_unaligned((userland_address() + 0xC) as *const u32) }
}

/// Retourne la taille totale du stockage
#[cfg(target_os = "none")]
unsafe fn size() -> u32 {
    unsafe { ptr::read_unaligned((userland_address() + 0x10) as *const u32) }
}

/// Trouve la prochaine position libre dans le stockage
#[cfg(target_os = "none")]
unsafe fn next_free() -> *const u32 {
    unsafe {
        let storage_addr = address();
        let mut offset = (storage_addr as *mut u8).add(4);
        let end_addr = (storage_addr + size()) as *mut u8;
        
        // Vérifier validité mais ignorer l'erreur (retourne null si invalide)
        if is_valid(storage_addr as *const u32).is_err() { return ptr::null(); }
        
        // Parcourir jusqu'à trouver un enregistrement vide (size=0)
        while offset < end_addr {
            let size = ptr::read_unaligned(offset as *const u16);
            if size == 0 { return offset as *const u32; }
            offset = offset.add(size as usize);
        }
        
        end_addr as *const u32
    }
}

/// Vérifie si le stockage est valide (magic number)
#[cfg(target_os = "none")]
unsafe fn is_valid(addr: *const u32) -> Result<()> {
    let magic_expected = 0xBADD0BEEu32.swap_bytes();
    let magic_found = unsafe { ptr::read_unaligned(addr) };
    if magic_found == magic_expected {
        Ok(())
    } else {
        Err(StorageError::InvalidMagicNumber { expected: magic_expected, found: magic_found })
    }
}

/// Détecte le modèle de calculatrice et retourne l'adresse userland
#[cfg(target_os = "none")]
unsafe fn userland_address() -> u32 {
    unsafe {
        // Adresses des slots magic pour chaque modèle
        let slots_n0110 = [0x90010000 as *const u32, 0x90410000 as *const u32];
        let slots_n0120 = [0x90020000 as *const u32, 0x90420000 as *const u32];
        let magic = 0xfeedc0deu32.swap_bytes();
        
        // Compter les slots valides pour chaque modèle
        let count_n0110 = slots_n0110.iter().filter(|&&slot| ptr::read_unaligned(slot) == magic).count();
        let count_n0120 = slots_n0120.iter().filter(|&&slot| ptr::read_unaligned(slot) == magic).count();
        
        // Choisir l'adresse de base selon le modèle détecté
        let base_addr = if count_n0110 > count_n0120 {
            ptr::read_unaligned(0x20000004 as *const u32).wrapping_add(0x10000) // N0110
        } else {
            ptr::read_unaligned(0x24000004 as *const u32).wrapping_add(0x20000) // N0120
        };
        
        base_addr.wrapping_sub(0x8)
    }
}