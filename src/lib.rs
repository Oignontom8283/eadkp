#![cfg_attr(target_os = "none", no_std)]

/*!
# eadkp
A Rust library to facilitate the development of external applications for NumWorks Epsilon (features and abstractions). 

## More information

For more information, visit the [eadkp repository on GitHub](https://github.com/Oignontom8283/eadkp).
*/

#[cfg(target_os = "none")]
extern crate alloc;

// Réexportation d'embedded_alloc pour qu'elle soit accessible via la macro eadk_setup!
#[cfg(target_os = "none")]
pub use embedded_alloc;

#[macro_use]
pub mod r#macro;

mod color;
mod image;
pub mod backlight;
pub mod utils;
pub mod display;
pub mod timing;
pub mod random;
pub mod battery;
pub mod input;
pub mod storage;

// Module builder uniquement disponible pour les build scripts (OS hôte, pas embarqué)
#[cfg(all(not(target_os = "none"), feature = "build-tools"))]
pub mod builder;

// Réexportations du contenu des modules
pub use utils::*;
pub use color::*;
pub use image::*;

// Note: Le panic handler et l'allocateur global sont définis par la macro eadkp_setup!
// L'application n'a pas besoin de les définir manuellement



// ~~   Déclarations des symboles de début et de fin du heap, définis dans le script de linkage   ~~
// En gros, le début des trucs compliqués, mais obligatoire sinon, bah ça marche pas.

unsafe extern "C" {
    pub static mut _heap_start: u8;
    pub static mut _heap_end: u8;
}

pub static mut HEAP_START: *mut u8 = core::ptr::addr_of_mut!(_heap_start);
pub static mut HEAP_END: *mut u8 = core::ptr::addr_of_mut!(_heap_end);

pub fn heap_size() -> usize {
    (unsafe { HEAP_END.offset_from(HEAP_START) }) as usize
}

// Stub requis par l'ARM EABI pour le unwinding de pile (inutilisé en no_std)
#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __aeabi_unwind_cpp_pr0() {}

// Désactive les interruptions et retourne l'état précédent (section critique)
#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _critical_section_1_0_acquire() -> u8 {
    let primask: u32;
    
    unsafe {
        core::arch::asm!(
            "mrs {}, PRIMASK",  // Lire l'état actuel des interruptions (0=activées, 1=désactivées)
            "cpsid i",          // Désactiver les interruptions
            out(reg) primask,
            options(nomem, nostack, preserves_flags)
        );
    }
    
    primask as u8  // Retourner l'état pour restauration ultérieure
}

// Restaure l'état des interruptions à partir du token (fin de section critique)
#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn _critical_section_1_0_release(token: u8) {
    if token & 1 == 0 {  // Si les interruptions étaient activées avant
        unsafe {
            core::arch::asm!(
                "cpsie i",  // Réactiver les interruptions
                options(nomem, nostack, preserves_flags)
            );
        }
    }
    // Sinon ne rien faire (les interruptions restent désactivées)
}
