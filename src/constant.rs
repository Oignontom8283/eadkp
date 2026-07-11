
use crate::{ common::{Rect, FontSize, Color} };

#[path = "shared.rs"]
mod shared;
pub use shared::*;


// SVC number 
pub const SVC_AUTHENTICATION_CLEARANCE_LEVEL: u32 = 0;
pub const SVC_BACKLIGHT_BRIGHTNESS: u32           = 1;
pub const SVC_BATTERY_IS_CHARGING: u32            = 3;
pub const SVC_BATTERY_LEVEL: u32                  = 4;
pub const SVC_BATTERY_VOLTAGE: u32                = 5;
pub const SVC_FCC_ID: u32                         = 29;
pub const SVC_PCB_VERSION: u32                    = 39;
pub const SVC_RESET_LAST_RESET_TYPE: u32          = 59;
pub const SVC_SERIAL_NUMBER_COPY: u32             = 47;
pub const SVC_TIMING_MILLIS: u32                  = 48;
pub const SVC_COMPILATION_FLAGS: u32              = 56;


// Magic numbers for various data structures in the firmware
pub const SLOTINFO_MAGIC: u32 = 0xEFEEDBBA;
pub const USERLAND_HEADER_MAGIC: u32 = 0xDEC0EDFE;
pub const KERNEL_HEADER_MAGIC: u32 = 0xDEC00DF0;
pub const FILESYSTEM_MAGIC: u32 = 0xBADD0BEEu32.swap_bytes();
pub const EXTERNAL_APPS_MAGIC: u32 = 0xDEC0EDFE;

// Memory layout constants for different hardware versions
pub const RAM_BASE_N0110_OR_N0115: u32 = 0x20000000;
pub const RAM_BASE_N0120: u32 = 0x24000000;

// Memory slot addresses for different hardware versions
pub const SLOTS_N0110_OR_N0115: [*const u32; 2] = [0x90010000 as *const u32, 0x90410000 as *const u32];
pub const SLOTS_N0120: [*const u32; 2] = [0x90020000 as *const u32, 0x90420000 as *const u32];

// Maximum length
pub const STORAGE_FILE_MAX_NAME_LEN: usize = u16::MAX as usize;


/// Représente le rectangle de l'écran entier. (preset pour éviter de devoir le recréer à chaque fois)
#[allow(dead_code)]
pub const SCREEN_RECT: Rect = Rect {
    x: 0,
    y: 0,
    width: 320,
    height: 240,
};

/// Taille d'un SMALL font character
#[allow(dead_code)]
pub const SMALL_FONT: FontSize = FontSize {
    width: 7,
    height: 14,
};


/// Taille d'un LARGE font character
#[allow(dead_code)]
pub const LARGE_FONT: FontSize = FontSize {
    width: 10,
    height: 18,
};


pub const COLOR_BLACK: Color = Color::from_888(0, 0, 0);
pub const COLOR_WHITE: Color = Color::from_888(255, 255, 255);
pub const COLOR_RED: Color = Color::from_888(255, 0, 0);
pub const COLOR_GREEN: Color = Color::from_888(0, 255, 0);
pub const COLOR_BLUE: Color = Color::from_888(0, 0, 255);
pub const COLOR_YELLOW: Color = Color::from_888(255, 255, 0);
pub const COLOR_CYAN: Color = Color::from_888(0, 255, 255);
pub const COLOR_MAGENTA: Color = Color::from_888(255, 0, 255);
pub const COLOR_GRAY: Color = Color::from_888(128, 128, 128);