
/// Définit la luminosité de l'écran.
pub fn set_brightness(brightness: u8) {
    unsafe {
        eadk_backlight_set_brightness(brightness);
    }
}

/// Récupère la luminosité actuelle de l'écran.
pub fn brightness() -> u8 {
    unsafe { eadk_backlight_brightness() }
}

unsafe extern "C" {
    fn eadk_backlight_set_brightness(brightness: u8);
    fn eadk_backlight_brightness() -> u8;
}