/*!
Sous-module `allocator` pour l'implémentation et gestion de l'allocateur global de heap sur hardware.
*/

use core::sync::atomic::{AtomicBool, Ordering};
use crate::{GlobalError, SoftwareError};

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;
