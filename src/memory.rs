
use super::*;
use core::ptr;


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
