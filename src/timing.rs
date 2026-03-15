
/// Sleep pour un nombre de microsecondes donné. (Arrête le thread pendant ce temps)
pub fn usleep(us: u32) {
    unsafe {
        eadk_timing_usleep(us);
    }
}

/// Sleep pour un nombre de millisecondes donné. (Arrête le thread pendant ce temps)
pub fn msleep(ms: u32) {
    unsafe {
        eadk_timing_msleep(ms);
    }
}

/// Obtient le nombre de millisecondes depuis le démarrage du système. ()
/// 
/// - Ne prend pas en compte les modes de veille.
/// - Depuis le démarrage/redémarrage/crash du système
pub fn millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

unsafe extern "C" {
    fn eadk_timing_usleep(us: u32);
    fn eadk_timing_msleep(ms: u32);
    fn eadk_timing_millis() -> u64;
}