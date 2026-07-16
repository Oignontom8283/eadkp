/*!
Sous-module `allocator` pour l'implémentation et gestion de l'allocateur global de heap sur hardware.
*/

use core::sync::atomic::{AtomicBool, Ordering};
use crate::{GlobalError, SoftwareError};

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;


// Déclarations des symboles externes
unsafe extern "C" {
    /// Adresse de début du tas, fournie par le script de linkage.
    static _heap_start: u8;
    /// Adresse de fin du tas, fournie par le script de linkage.
    static _heap_end: u8;
}


/// Allocateur global pour la heap.
#[cfg(target_os = "none")]
#[global_allocator]
static HEAP: Heap = Heap::empty();

/// Indique si l'allocateur global a été initialisé.
static INITIALIZED: AtomicBool = AtomicBool::new(false);


/// Adresse de début du tas, fournie par le script de linkage.
#[cfg(target_os = "none")]
pub fn start() -> Result<*const u8, GlobalError> {
    Ok(core::ptr::addr_of!(_heap_start))
}

#[cfg(not(target_os = "none"))]
pub fn start() -> Result<*const u8, GlobalError> { Err(SoftwareError::SimulatorNotSupported.into()) }

