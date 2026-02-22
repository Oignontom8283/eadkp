/*!
Ce module fournit des structures de données et fonctions utilitaires pour interagir
avec les composants essentiels d'Epsilon en mêmoire.
*/

use core::ptr;
use core::ffi::{CStr, c_char};

use crate::SoftwareError;


pub const SLOTINFO_MAGIC: u32 = 0xEFEEDBBA;
pub const USERLAND_HEADER_MAGIC: u32 = 0xDEC0EDFE;
pub const KERNEL_HEADER_MAGIC: u32 = 0xDEC00DF0;
pub const FILESYSTEM_MAGIC: u32 = 0xBADD0BEEu32.swap_bytes();
pub const EXTERNAL_APPS_MAGIC: u32 = 0xDEC0EDFE;

pub const RAM_BASE_N0110_OR_N0115: u32 = 0x20000000;
pub const RAM_BASE_N0120: u32 = 0x24000000;

pub const SLOTS_N0110_OR_N0115: [*const u32; 2] = [0x90010000 as *const u32, 0x90410000 as *const u32];
pub const SLOTS_N0120: [*const u32; 2] = [0x90020000 as *const u32, 0x90420000 as *const u32];


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
    pub fn is_valid(&self) -> Result<(), SoftwareError> {

        if self.header != KERNEL_HEADER_MAGIC {
            return Err(SoftwareError::InvalidMagicNumber { expected: KERNEL_HEADER_MAGIC, found: self.header });
        }
        if self.footer != KERNEL_HEADER_MAGIC {
            return Err(SoftwareError::InvalidMagicNumber { expected: KERNEL_HEADER_MAGIC, found: self.footer });
        }
        Ok(())
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
    pub fn is_valid(&self) -> Result<(), SoftwareError> {
        unsafe {
            let header = ptr::read_unaligned(self.header_addr);
            let footer = ptr::read_unaligned(self.footer_addr);

            if header != FILESYSTEM_MAGIC {
                return Err(SoftwareError::InvalidMagicNumber { expected: FILESYSTEM_MAGIC, found: header });
            }
            if footer != FILESYSTEM_MAGIC {
                return Err(SoftwareError::InvalidMagicNumber { expected: FILESYSTEM_MAGIC, found: footer });
            }
        }
        Ok(())
    }
}


// ============================================================================
// HARDWARE INTERFACE / TOOLS
// ============================================================================

/// Retourne le modèle de la calculatrice
#[cfg(target_os = "none")]
pub fn model() -> CalculatorModel {
    CalculatorModel::detect()
}

/// Retourne l'adresse de début de la RAM
#[cfg(target_os = "none")]
pub fn address() -> *const u8 {
    CalculatorModel::detect().ram_base()
}

/// Retourne l'adresse du Kernel Header
#[cfg(target_os = "none")]
pub fn kernel_header() -> &'static KernelHeader {
    CalculatorModel::detect().slotinfo_address().kernel_header_address
}

/// Retourne l'adresse du Userland Header
#[cfg(target_os = "none")]
pub fn userland_header() -> &'static UserlandHeader {
    CalculatorModel::detect().slotinfo_address().userland_header_address
}

/// Retourne l'utilitaire d'adresse du filesystem
#[cfg(target_os = "none")]
pub fn filesystem() -> Filesystem {
    Filesystem::new()
}