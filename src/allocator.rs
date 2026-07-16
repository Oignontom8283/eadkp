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


/// Adresse de fin du tas, fournie par le script de linkage.
#[cfg(target_os = "none")]
pub fn end() -> Result<*const u8, GlobalError> {
    Ok(core::ptr::addr_of!(_heap_end))
}

#[cfg(not(target_os = "none"))]
pub fn end() -> Result<*const u8, GlobalError> { Err(SoftwareError::SimulatorNotSupported.into()) }


/// Taille totale du tas en bytes (fixe, définie par le linker script).
pub fn total_size() -> Result<usize, GlobalError> {
    Ok(unsafe { end()?.offset_from(start()?) as usize } )
}


/// Initialise l'allocateur global.
/// - Retourne [`SoftwareError::AlreadyInitialized`] si appelé plus d'une fois.
/// 
/// ## Warning
/// - **À appeler une seule fois au démarrage !**
#[cfg(target_os = "none")]
pub fn init() -> Result<(), GlobalError> {
    
    // Vérifie si l'allocateur a déjà été initialisé
    if INITIALIZED.swap(true, Ordering::SeqCst) {
        return Err(SoftwareError::AlreadyInitialized { details: "allocator already initialized" }.into());
    }

    // Initialise l'allocateur avec les adresses de début et de fin de heap
    unsafe {
        HEAP.init(start()? as usize, total_size()?)
    }
    
    Ok(())
}

#[cfg(not(target_os = "none"))]
pub fn init() -> Result<(), GlobalError> {
    // En OS ne rien faire, l'allocateur est celui du std
    Ok(())
}
