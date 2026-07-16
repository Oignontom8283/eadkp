#![cfg_attr(target_os = "none", no_std)]

/*!
# eadkp
A Rust library to facilitate the development of external applications for NumWorks Epsilon (features and abstractions). 

## More information

For more information, visit the [eadkp repository on GitHub](https://github.com/Oignontom8283/eadkp).
*/

extern crate alloc;

#[macro_use]
pub mod r#macro;

pub mod backlight;
pub mod common;
pub mod utils;
pub mod display;
pub mod timing;
pub mod random;
pub mod battery;
pub mod input;
pub mod storage;
pub mod epsilon;
pub mod constant;
pub mod sys;
pub mod allocator;
mod errors;

// Module builder uniquement disponible pour les build scripts (OS hôte, pas embarqué)
#[cfg(all(not(target_os = "none"), feature = "build-tools"))]
pub mod builder;

// Réexportations du contenu des modules
pub use common::*;
pub use constant::*;
pub use errors::*;
// Note: Le panic handler et l'allocateur global sont définis par la macro eadkp_setup!
// L'application n'a pas besoin de les définir manuellement




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
