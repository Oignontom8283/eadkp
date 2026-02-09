
use super::*;
use core::ptr;

/// Magic number for EIF1 format. Magic number in hex `0x31464945`
pub const EIF1_MAGIC_NUMBER: u32 = u32::from_le_bytes(*b"EIF1"); 

/// Rectangle structure
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Rect {
    pub x: u16,
    pub y: u16,

    /// Width of the rectangle in pixels (axe X) (fr: Largeur)
    pub width: u16,

    /// Height of the rectangle in pixels (axe Y) (fr: Hauteur)
    pub height: u16,
}

/// Full screen rectangle (320x240)
#[allow(dead_code)]
pub const SCREEN_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};


/// Character size for a font
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct FontSize {
    /// Width (axe X) of one character in pixels (fr: Largeur)
    pub width: u16,
    /// Height (axe Y) of one character in pixels (fr: Hauteur)
    pub height: u16,
}

/// Size of SMALL font character
#[allow(dead_code)]
pub const SMALL_FONT: FontSize = FontSize {
    width: 7,
    height: 14,
};


/// Size of LARGE font character
#[allow(dead_code)]
pub const LARGE_FONT: FontSize = FontSize {
    width: 10,
    height: 18,
};

/// 2D Point structure
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}


// ============================================================================
// MEMORY UTILITIES
// ============================================================================

/// Compare deux C strings
pub unsafe fn strcmp(s1: *const u8, s2: *const u8) -> bool {
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
/// **Erreur en cas de chevauchement des zones mémoire** pour éviter les comportements indéfinis/corrompuptions.
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> Result<(), SoftwareError> {
    
    if n == 0 {
        return Ok(()); // Rien à copier, pas de risque de chevauchement
    }

    let src_start = src as usize;
    let src_end = src_start.checked_add(n).ok_or(SoftwareError::PointerOverflow)?;
    let dest_start = dest as usize;
    let dest_end = dest_start.checked_add(n).ok_or(SoftwareError::PointerOverflow)?;

    // Vérifier que les zones ne chevauchent pas
    if dest_start < src_end && src_start < dest_end {
        // Zones chevauchantes détectées
        return Err(SoftwareError::OverlappingRegions {
            src_start: src,
            src_end: src.add(n),
            dest_start: dest,
            dest_end: dest.add(n)
        });
    }
    
    // Effectuer la copie
    ptr::copy_nonoverlapping(src, dest, n);
    Ok(())
}

/// Copie `n` bytes de `src:*` vers `dest:*` (zones peuvent chevaucher)
/// 
/// **Comportement défini même en cas de chevauchement:** La copie se fait par une mémoire tampon.
pub unsafe fn memmove(dest: *mut u8, src: *const u8, n: usize) {
    unsafe { ptr::copy(src, dest, n) }
}

/// Remplit n bytes avec la valeur c
pub unsafe fn memset(s: *mut u8, c: u8, n: usize) {
    for i in 0..n {
        unsafe { *s.add(i) = c };
    }
}
