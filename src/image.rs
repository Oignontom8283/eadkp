#[cfg(target_os = "none")]
use alloc::boxed::Box;

#[cfg(not(target_os = "none"))]
use std::boxed::Box;

use super::*;

/// # Représente une image chargée en mémoire
/// 
/// ## Champs
/// - `magic_number`: Numéro magique identifiant le format de l'image
/// - `width`: Largeur de l'image en pixels d'écran (axe Y)
/// - `height`: Hauteur de l'image en pixels d'écran (axe X)
/// - `pixels`: Données des pixels de l'image au format Color (RGB565)
/// - `binary`: Pointeur vers les données binaires brutes de l'image dans la flash (Immuable/Statique)
/// 
/// # Remarques
/// 
/// Utilisation de Box au lieu de Vec pour une allocation fixe. Économie de mémoire !
///
/// Explication :
/// ```rust
/// // Pour l'object Vec<T> :
/// ptr: *mut T, // 8 octets
/// len: usize,  // 8 octets
/// cap: usize,  // 8 octets
///              // Total: 24 octets
/// 
/// // Pour l'object Box<[T]> :
/// ptr: *mut T, // 8 octets
/// len: usize,  // 8 octets
///              // Total: 16 octets
/// ```
/// 
/// Économie de 8 octets (64 bits) par image chargée en mémoire. C'est considérable sur plusieurs images !
///
pub struct Image {
    pub magic_number: u32,
    pub width: u16,
    pub height: u16,
    pub pixels: Box<[Color]>,
    pub binary: &'static [u8],
}

impl Image {
    pub fn from_raw(binary_raw: &'static [u8]) -> Option<Self> {
        
        // Lire le magic number 
        // let magic_number: u32 = unsafe { *(binary_raw.as_ptr() as *const u32) };
        let magic_number: u32 = u32::from_le_bytes(
            binary_raw[..4].try_into().expect("Slice trop courte pour magic number")
        );

        // EIF1 format
        if magic_number == utils::EIF1_MAGIC_NUMBER {
            return Some(Image::from_raw_eif1(binary_raw));
        };
        
        // Format inconnu
        None
    }
    
    pub fn from_raw_eif1(binary_raw: &'static [u8]) -> Self {
        // Lire le magic number (déréférencer pour obtenir la valeur)
        let magic_number: u32 = unsafe { *(binary_raw.as_ptr() as *const u32) };

        // Lire width et height (déréférencer pour obtenir les valeurs)
        let width: u16 = unsafe { *(binary_raw.as_ptr().add(4) as *const u16) };
        let height: u16 = unsafe { *(binary_raw.as_ptr().add(6) as *const u16) };
        let surface_size = (width as usize) * (height as usize);

        // Obtenir un pointeur vers les données binaires de l'image
        let binary: &'static [u8] = &binary_raw[8..];

        // Créer directement le Box<[Color]> avec un iterator
        let pixels: Box<[Color]> = (0..surface_size)
            .map(|i| {
                let idx = i * 2;
                let rgb565 = u16::from_le_bytes([binary[idx], binary[idx + 1]]);
                Color { rgb565 }
            })
            .collect();

        Image {
            magic_number,
            width,
            height,
            pixels,
            binary,
        }
    }

    pub fn get_pixels(&self) -> &[Color] {
        &self.pixels
    }

    pub fn for_coordinates(&self, x: u16, y: u16) -> Rect {
        Rect {
            x,
            y,
            width: self.width,
            height: self.height,
        }
    }
}