use crate::{SoftwareError, GlobalError};


/// Convertit un buffer d'octets brut en chain de caractères Rust (`&str`)
/// - S'arrête au premier octet nul (`\0`) rencontré.
/// - Renvoie une erreur si aucun octet nul n'est trouvé dans le buffer.
/// ```
/// //                          S'arrête ici
/// //                          ↓
/// let buffer = b"Hello, world!\0Extra data";
/// let result = str_from_fixed_buffer(buffer);
/// 
/// assert_eq!(result.unwrap(), "Hello, world!");
/// ```
pub fn str_from_fixed_buffer(buffer: &[u8]) -> Result<&str, GlobalError> {
    
    // Obtenir la position du premier octet nul dans le buffer
    let len = buffer
        .iter()
        .position(|&b| b == 0)
        .ok_or(SoftwareError::NoNullTerminator)?;

    let slice = &buffer[..len];

    // Convertir le slice en &str sans vérifier la validité UTF-8 (unsafe)
    Ok(unsafe { str::from_utf8_unchecked(slice) })
}


/// Calculer la taille d'une plage de mémoire définie par deux pointeurs.
/// - Renvoie 0 si les pointeurs sont invalides ou dans le mauvais ordre.
/// 
/// ## Exemple
/// ```
/// let start_ptr = 0x1000 as *const u8; // 4096
/// let end_ptr = 0x2000 as *const u8; // 8192
/// 
/// let size = ptr_range_size(start_ptr, end_ptr);
/// 
/// assert_eq!(size, 0x1000); // 4096
/// ```
pub unsafe fn ptr_range_size(start: *const u8, end: *const u8) -> usize {

    // Vérifier que les pointeurs sont valides et dans le bon ordre
    if start.is_null() || end.is_null() || end < start {
        return 0;
    }
    
    // Calculer la taille de la plage en utilisant offset_from
    end.offset_from(start) as usize
}


/// Faire un appel SVC et récupérer la valeur de retour dans `r0`.
/// - (`u32` / `bool` / `enum` `#[repr(u32)]`)
/// - Le type de retour doit être spécifié explicitement lors de l'appel de la macro.
/// - Miroir de SVC_RETURNING_R0 du C++
#[macro_export]
#[cfg(target_os = "none")]
macro_rules! svc_r0 {
    ($num:expr, $ty:ty) => {{
        let result: u32;
        unsafe {
            core::arch::asm!(
                "svc {num}",
                "mov {out}, r0",
                num = const $num,
                out = out(reg) result,
                // r0-r3 sont clobberés par le SVC (convention AAPCS)
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
        result as $ty
    }};
}

/// Faire un appel SVC et récupérer la valeur de retour dans `s0`.
/// - (`f32`)
/// - Miroir de SVC_RETURNING_S0 du C++
#[macro_export]
#[cfg(target_os = "none")]
macro_rules! svc_s0 {
    ($num:expr) => {{
        let bits: u32;
        unsafe {
            core::arch::asm!(
                "svc {num}",
                // move s0 into a general-purpose register as u32 bits
                "vmov {bits}, s0",
                num  = const $num,
                bits = out(reg) bits,
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
        f32::from_bits(bits)
    }};
}

/// Faire un appel SVC et récupérer la valeur de retour 64 bits dans `r0:r1`.
/// - (`u64` / `enum` `#[repr(u64)]`)
/// - Miroir de SVC_RETURNING_R0R1 du C++
#[macro_export]
#[cfg(target_os = "none")]
macro_rules! svc_r0r1 {
    ($num:expr) => {{
        let lo: u32;
        let hi: u32;
        unsafe {
            core::arch::asm!(
                "svc {num}",
                "mov {lo}, r0",
                "mov {hi}, r1",
                num = const $num,
                lo  = out(reg) lo,
                hi  = out(reg) hi,
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
        ((hi as u64) << 32) | (lo as u64)
    }};
}

/// Faire un appel SVC et stocker l'adresse de destination dans `r0` avant le SVC.
/// - (`*mut T` / `*const T`)
/// - Miroir de SVC_RETURNING_STASH_ADDRESS_IN_R0 du C++
#[macro_export]
#[cfg(target_os = "none")]
macro_rules! svc_addr_in_r0 {
    ($num:expr, $ptr:expr) => {{
        unsafe {
            core::arch::asm!(
                "mov r0, {ptr}",
                "svc {num}",
                ptr = in(reg) $ptr,
                num = const $num,
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
    }};
}

/// Faire un appel SVC avec un buffer alloué par l'appelant (caller-allocated buffer).
/// - Le kernel reçoit une adresse dans `r0` et y écrit les données de retour (Notre buffer).
/// - Miroir de SVC_RETURNING_STASH_ADDRESS_IN_R0 du C++ **mais avec gestion du buffer intégrée.**
/// - Retourne `[u8; $len]` rempli par le kernel.
/// - Il s'agit d'une version automatisée de `svc_addr_in_r0!` qui gére le buffer pour vous.
/// 
/// ## Exemple
/// ```rust
/// // Appel SVC pour récupérer le numéro de série
/// let serial = svc_buf!(SVC_SERIAL_NUMBER_COPY, 16); // 16 octets — taille du buffer voulu
/// 
/// // Le buffer est rempli par le kernel
/// assert!(serial.iter().all(|&b| b != 0)); 
/// ```
#[macro_export]
#[cfg(target_os = "none")]
macro_rules! svc_buf {
    ($num:expr, $len:expr) => {{
        let mut buf = [0u8; $len];
        unsafe {
            core::arch::asm!(
                "mov r0, {ptr}",
                "svc {num}",
                ptr = in(reg) buf.as_mut_ptr(),
                num = const $num,
                lateout("r0") _,
                lateout("r1") _,
                lateout("r2") _,
                lateout("r3") _,
                options(nostack),
            );
        }
        buf
    }};
}