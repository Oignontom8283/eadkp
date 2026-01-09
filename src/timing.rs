
/// Sleep for the specified number of microseconds.
pub fn usleep(us: u32) {
    unsafe {
        eadk_timing_usleep(us);
    }
}

/// Sleep for the specified number of milliseconds.
pub fn msleep(ms: u32) {
    unsafe {
        eadk_timing_msleep(ms);
    }
}

/// Get the number of milliseconds since the system started.
/// 
/// Does not take sleep modes into account.
pub fn millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

unsafe extern "C" {
    fn eadk_timing_usleep(us: u32);
    fn eadk_timing_msleep(ms: u32);
    fn eadk_timing_millis() -> u64;
}