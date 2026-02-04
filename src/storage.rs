
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

use core::ptr;
use heapless;


// ============================================================================
// STORAGE MISC UTILITIES
// ============================================================================

const SLOTINFO_MAGIC: u32 = 0xEFEEDBBA;
const USERLAND_HEADER_MAGIC: u32 = 0xDEC0EDFE;
const KERNEL_HEADER_MAGIC: u32 = 0xDEC00DF0;
const FILESYSTEM_MAGIC: u32 = 0xBADD0BEEu32.swap_bytes();
const EXTERNAL_APPS_MAGIC: u32 = 0xDEC0EDFE;

const RAM_BASE_N0110_OR_N0115: u32 = 0x20000000;
const RAM_BASE_N0120: u32 = 0x24000000;

const SLOTS_N0110_OR_N0115: [*const u32; 2] = [0x90010000 as *const u32, 0x90410000 as *const u32];
const SLOTS_N0120: [*const u32; 2] = [0x90020000 as *const u32, 0x90420000 as *const u32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Simulateur non supporté
    SimulatorNotSupported,
    /// SlotInfo invalide
    InvalidSlotInfo,
    /// UserlandHeader invalide
    InvalidUserlandHeader,
    /// FileSystem invalide
    InvalidFileSystem,
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


// ============================================================================
// HARDWARE METADATA
// ============================================================================

/// SlotInfo - 16 bytes au dèbut de la RAM
/// 
/// Le SlotInfo est une structure de métadonnées essentielle, située au dèbut de la RAM (SRAM) de la calculatrice
/// (L'adresse du dèbut de la RAM varie en fonction du modèle, voir `CalculatorModel`).
/// Il contient des pointeurs vers les headers du kernel et du userland, permettant ainsi de connaître leur emplacement en mémoire.
/// Ces informations sont cruciales, car elles permettent d'accéder notamment au UserlandHeader, qui lui-même contient par exemple l'adresse et la taille du système de fichiers.
/// 
/// Contient les pointeurs vers les headers du kernel et du userland :
/// - kernel_header_address : Pointeur static vers le KernelHeader.
/// - userland_header_address : Pointeur static vers le UserlandHeader
/// 
/// Note: 
/// - Le SlotInfo est protégé par des "magic numbers" au dèbut et à la fin (0xEFEEDBBA) pour vérifier son intégrité et ça présence.
/// - Les pointeurs sont de type static car il s’agit de données primaires de l'os lui-même, donc impérativement présentes, sinon pas d'os, donc pas d'application non plus.
#[repr(C)]
#[derive(Debug)]
pub struct SlotInfo {
    pub header: u32,                                      // +0x00: 0xEFEEDBBA en little-endian
    pub kernel_header_address: &'static KernelHeader,     // +0x04: Pointeur static vers KernelHeader
    pub userland_header_address: &'static UserlandHeader, // +0x08: Pointeur static vers UserlandHeader
    pub footer: u32,                                      // +0x0C: 0xEFEEDBBA en little-endian
}                                                         // = Total: 16 bytes

impl SlotInfo {
    /// Vérifie que le SlotInfo est valide en vérifiant les magic numbers.
    /// 
    /// Peut probable de retourner false en pratique, car si le SlotInfo est corrompu ou absent, l'os ne devrait pas être encore en cours d'exécution.
    pub fn is_valid(&self) -> bool {
        self.header == SLOTINFO_MAGIC && self.footer == SLOTINFO_MAGIC
    }
}


/// UserlandHeader - 48 bytes au début du userland
/// 
/// Contient les métadonnées essentielles du userland,
/// notamment l'emplacement du système de fichiers.
#[repr(C)]
#[derive(Debug)]
pub struct UserlandHeader {
    pub header: u32,                          // +0x00: 0xDEC0EDFE
    pub expected_epsilon_version: [u8; 8],    // +0x04: Version attendue
    pub storage_address_ram: *const u8,       // +0x0C: Adresse du FileSystem en RAM
    pub storage_size_ram: u32,                // +0x10: Taille du storage (42 Ko)
    pub external_apps_flash_start: *const u8, // +0x14: Début apps externes
    pub external_apps_flash_end: *const u8,   // +0x18: Fin apps externes
    pub external_apps_ram_start: *const u8,   // +0x1C: RAM apps externes
    pub external_apps_ram_end: *const u8,     // +0x20: Fin RAM apps
    pub device_name_flash_start: *const u8,   // +0x24: Nom device
    pub device_name_flash_end: *const u8,     // +0x28: Fin nom device
    pub footer: u32,                          // +0x2C: 0xDEC0EDFE
}


/// KernelHeader - 24 bytes au début de la zone kernel
/// 
/// Contient les informations de version du noyau Epsilon.
/// 
/// Emplacement :
/// - Juste après la signature (8 bytes) au début du slot actif
/// - N0110/N0115 Slot A : 0x90000008
/// - N0110/N0115 Slot B : 0x90400008
/// - N0120 Slot A       : 0x90000008
/// - N0120 Slot B       : 0x90400008
#[repr(C)]
#[derive(Debug)]
pub struct KernelHeader {
    pub header: u32,                      // +0x00: 0xDEC00DF0
    pub epsilon_version: [u8; 8],         // +0x04: Version Epsilon (ex: "23.2.1")
    pub patch_level: [u8; 8],             // +0x0C: Niveau de patch (ex: "official")
    pub footer: u32,                      // +0x14: 0xDEC00DF0
}

impl KernelHeader {
    /// Vérifie que le KernelHeader est valide
    pub fn is_valid(&self) -> bool {
        self.header == KERNEL_HEADER_MAGIC && self.footer == KERNEL_HEADER_MAGIC
    }
}


/// Modèle de la calculatrice
/// 
/// Utilisé pour déterminer les adresses de dèbut de la RAM et l'emplacement du slotinfo.
/// 
/// - `N0110` ou `N0115` partagent la même adresse de RAM de base et les mêmes emplacements de slots.
/// - `N0120` utilise une adresse de RAM différente et des emplacements de slots distincts.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculatorModel {
    N0110_N0115,
    N0120,
}

impl CalculatorModel {
    /// Adresse de début de la RAM selon le modèle
    pub fn ram_base(&self) -> *const u8 {
        match self {
            Self::N0110_N0115 => RAM_BASE_N0110_OR_N0115 as *const u8,
            Self::N0120 => RAM_BASE_N0120 as *const u8,
        }
    }
    
    /// Retourne un pointeur vers le SlotInfo de la calculatrice
    pub fn slotinfo_address(&self) -> &'static SlotInfo {

        // Obtenir l'adresse de début de la RAM
        let ram = self.ram_base() as *const u32;
        
        // Lire le SlotInfo depuis la RAM
        // Le SlotInfo est situé au début de la RAM, a la première adresse
        let slot_info = ram as *const SlotInfo;
        
        // Vérifier que le pointeur n'est pas null
        let slot_info_ref = unsafe { slot_info.as_ref().expect("SlotInfo pointer is null") };

        if !slot_info_ref.is_valid() {
            panic!("Invalid SlotInfo detected at address {:p}", slot_info);
        }
        
        return slot_info_ref;
    }

    /// Modèle détecté à partir des slots magic
    pub fn detect() -> Self {
        unsafe {
            // Compter les slots valides pour chaque modèle
            let count_n0110_n0115 = SLOTS_N0110_OR_N0115.iter().filter(|&&slot| ptr::read_unaligned(slot) == EXTERNAL_APPS_MAGIC).count();
            let count_n0120 = SLOTS_N0120.iter().filter(|&&slot| ptr::read_unaligned(slot) == EXTERNAL_APPS_MAGIC).count();
            
            // Déterminer le modèle avec le plus de slots valides
            if count_n0110_n0115 > count_n0120 {
                Self::N0110_N0115
            } else {
                Self::N0120
            }
        }
    }
}

/// Filesystem addresses and metadata
/// 
/// Repertorie les adresses et tailles du système de fichiers embarqué dans la RAM.
/// Calcule les zones utilisables en fonction des headers et footers magiques.
/// 
/// ## Layout du Filesystem :
/// ```
/// Adresse de base : storage_address_ram
/// Taille totale   : storage_size_ram (43016 bytes)
///
///  Offset 0            Offset 4                              Offset 43012     Offset 43016  
///  ↓                   ↓                                     ↓                ↓         
/// ┌───────────────────┬─────────────────────────────────────┬────────────────┐
/// │  Magic Header     │     Buffer utilisable (42 Ko)       │  Magic Footer  │
/// │   (4 bytes)       │           (43008 bytes)             │   (4 bytes)    │
/// └───────────────────┴─────────────────────────────────────┴────────────────┘
///  ↑                   ↑                                     ↑                ↑
///  storage_start_addr  usable_start_addr                     usable_end_addr  storage_end_addr
///  header_addr                                               footer_addr
/// ```
#[derive(Debug)]
pub struct Filesystem {
    /// Taille totale du stockage, y compris header/footer
    pub storage_size: u32,
    
    /// Adresse de début du stockage
    pub storage_start_addr: *const u8,
    /// Adresse de fin du stockage, juste après le footer
    pub storage_end_addr: *const u8,
    /// Adresse du header magique
    pub header_addr: *const u32,
    /// Adresse du footer magique
    pub footer_addr: *const u32,
    
    /// Taille utilisable (total - header/footer)
    pub usable_size: u32,
    /// Adresse de début de la zone utilisable (juste après le header)
    pub usable_start_addr: *const u8,
    /// Adresse de fin de la zone utilisable (même adresse que le footer / juste après la zone utilisable, `range semi-ouvert [start, end)`)
    /// 
    /// Suit la convention des ranges Rust : [usable_start_addr, usable_end_addr)
    /// où usable_end_addr est **exclusif** (premier byte non utilisable)
    pub usable_end_addr: *const u8,
}

impl Filesystem {
    /// Initialise le Filesystem en lisant les adresses depuis le UserlandHeader et en calculant les zones utilisables.
    pub fn new() -> Self {
        let user_land = CalculatorModel::detect().slotinfo_address().userland_header_address;

        Self {
            storage_size: user_land.storage_size_ram, 

            storage_start_addr: user_land.storage_address_ram,
            storage_end_addr: unsafe { user_land.storage_address_ram.add(user_land.storage_size_ram as usize) }, // Fin du stockage = début + taille
            header_addr: user_land.storage_address_ram as *const u32, // Adresse du header (début du stockage)
            footer_addr: unsafe { user_land.storage_address_ram.add(user_land.storage_size_ram as usize - 4) as *const u32 }, // fin du stockage - 4 bytes (taille footer)

            usable_size: user_land.storage_size_ram - 8, // Taille utilisable (total - header/footer)
            usable_start_addr: unsafe { user_land.storage_address_ram.add(4)}, // Adresse juste après le header
            usable_end_addr: unsafe { user_land.storage_address_ram.add(user_land.storage_size_ram as usize - 4)} // adresse du footer = fin de la zone utilisable
        }
    }

    /// Vérifie que le Filesystem est valide en vérifiant les magic numbers au début et à la fin.
    pub fn is_valid(&self) -> bool {
        unsafe {
            ptr::read_unaligned(self.header_addr) == FILESYSTEM_MAGIC &&
            ptr::read_unaligned(self.footer_addr) == FILESYSTEM_MAGIC
        }
    }
}

// ============================================================================
// HARDWARE INTERFACE / TOOLS
// ============================================================================

/// Retourne le modèle de la calculatrice
#[cfg(target_os = "none")]
fn model() -> CalculatorModel {
    CalculatorModel::detect()
}

/// Retourne l'adresse de début de la RAM
#[cfg(target_os = "none")]
fn address() -> *const u8 {
    CalculatorModel::detect().ram_base()
}

/// Retourne l'adresse du Kernel Header
#[cfg(target_os = "none")]
fn kernel_header() -> &'static KernelHeader {
    CalculatorModel::detect().slotinfo_address().kernel_header_address
}

/// Retourne l'adresse du Userland Header
#[cfg(target_os = "none")]
fn userland_header() -> &'static UserlandHeader {
    CalculatorModel::detect().slotinfo_address().userland_header_address
}

/// Retourne l'utilitaire d'adresse du filesystem
#[cfg(target_os = "none")]
fn filesystem() -> Filesystem {
    Filesystem::new()
}


// ============================================================================
// MEMORY UTILITIES
// ============================================================================

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
/// **Panic en cas de chevauchement des zones mémoire** pour éviter les comportements indéfinis/corrompuptions.
#[cfg(target_os = "none")]
unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {

    let src_start = src as usize;
    let src_end = src_start + n;
    let dest_start = dest as usize;
    let dest_end = dest_start + n;

    // Vérifier que les zones ne chevauchent pas
    if dest_start < src_end && src_start < dest_end {
        // Zones chevauchantes détectées
        panic!("memcpy called with overlapping regions: src {:p} - {:p}, dest {:p} - {:p}", src, src.add(n), dest, dest.add(n));
    }
    
    // Effectuer la copie
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

    let storage = filesystem();
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

// TODO: can_store() pour vérifier si un fichier peut être stocké avant d'essayer de l'écrire
// TODO: available_space() pour obtenir l'espace libre restant

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

