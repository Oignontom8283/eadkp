
pub fn usleep(us: u32) {
    unsafe {
        eadk_timing_usleep(us);
    }
}

pub fn msleep(ms: u32) {
    unsafe {
        eadk_timing_msleep(ms);
    }
}

pub fn millis() -> u64 {
    unsafe { eadk_timing_millis() }
}

unsafe extern "C" {
    fn eadk_timing_usleep(us: u32);
    fn eadk_timing_msleep(us: u32);
    fn eadk_timing_millis() -> u64;
}