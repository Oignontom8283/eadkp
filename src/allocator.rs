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

    // ⚠️ IMPORTANT : L'instruction `AtomicBool.swap()` est volontairement évitée ici !
    // Le compilateur transforme le `.swap()` en instructions assembleur LDREX/STREX.
    // La section où se trouve l'application est probablement configurée en "Shareable" par l'OS via la MPU.
    // PROBLÈME : Pour que des instructions LDREX/STREX fonctionnent sur une section "Shareable",
    //            le bus système doit disposer d'un **Moniteur d'exclusivité global**, ce qui n'est pas le cas sur STM32.
    // L'utilisation des instructions LDREX/STREX provoque donc un BusFault (Crash CPU) de la calculatrice.
    
    // Vérifie si l'allocateur a déjà été initialisé
    if INITIALIZED.load(Ordering::Relaxed) {
        return Err(SoftwareError::AlreadyInitialized { details: "allocator already initialized" }.into());
    }

    // Marque l'allocateur comme initialisé
    INITIALIZED.store(true, Ordering::Relaxed);

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


/// Vérifie si l'allocateur a déjà été initialisé.
pub fn is_initialized() -> Result<(), GlobalError> {
    if INITIALIZED.load(Ordering::SeqCst) {
        Ok(())
    } else {
        Err(SoftwareError::NotAvailable { details: "allocator not initialized" }.into())
    }
}



/// Bytes actuellement alloués sur le tas.
#[cfg(target_os = "none")]
pub fn used() -> Result<usize, GlobalError> {
    is_initialized()?;

    Ok(HEAP.used())
}

#[cfg(not(target_os = "none"))]
pub fn used() -> Result<usize, GlobalError> { Err(SoftwareError::SimulatorNotSupported.into()) }


/// Bytes encore libres sur le tas.
#[cfg(target_os = "none")]
pub fn free() -> Result<usize, GlobalError> {
    is_initialized()?;

    Ok(HEAP.free())
}

#[cfg(not(target_os = "none"))]
pub fn free() -> Result<usize, GlobalError> { Err(SoftwareError::SimulatorNotSupported.into()) }


/// Pourcentage d'utilisation du tas (0.0 à 100.0).
pub fn usage_percent() -> Result<f32, GlobalError> {
    is_initialized()?;

    let used = used()?;
    let total = total_size()?;

    if total == 0 { // Évite la division par zéro
        return Err(SoftwareError::EmptyValue.into());
    }

    Ok((used as f32 / total as f32) * 100.0)
}