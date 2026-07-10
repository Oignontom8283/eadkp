
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