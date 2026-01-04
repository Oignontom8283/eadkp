
/// Represents a color using the RGB565 format.
/// 
/// The `rgb565` field stores the color as a 16-bit unsigned integer,
/// where the bits are allocated as follows:
/// - 5 bits for red
/// - 6 bits for green
/// - 5 bits for blue
///
/// This format is commonly used in embedded systems and graphics applications
/// for efficient color representation.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub rgb565: u16,
}

pub const COLOR_BLACK: Color = Color::from_888(0, 0, 0);
pub const COLOR_WHITE: Color = Color::from_888(255, 255, 255);
pub const COLOR_RED: Color = Color::from_888(255, 0, 0);
pub const COLOR_GREEN: Color = Color::from_888(0, 255, 0);
pub const COLOR_BLUE: Color = Color::from_888(0, 0, 255);
pub const COLOR_YELLOW: Color = Color::from_888(255, 255, 0);
pub const COLOR_CYAN: Color = Color::from_888(0, 255, 255);
pub const COLOR_MAGENTA: Color = Color::from_888(255, 0, 255);
pub const COLOR_GRAY: Color = Color::from_888(128, 128, 128);

impl Color {
    #[inline]
    /// Creates a Color from RGB565 **(5-bit red, 6-bit green, and 5-bit blue)** components.
    pub const fn from_components(r: u8, g: u8, b: u8) -> Self {
        Color {
            rgb565: ((r as u16) << 11) | ((g as u16) << 5) | (b as u16),
        }
    }
    /// Creates a Color from 8-bit per channel RGB values. (Normal RGB)
    pub const fn from_888(r: u8, g: u8, b: u8) -> Self {
        Color::from_components(r >> 3, g >> 2, b >> 3)
    }

    pub fn apply_light(&self, light_level: u8) -> Self {
        let light_level = light_level as u16;
        let (r, g, b) = self.get_components();
        Color::from_components(
            ((r as u16 * light_level / 255).min(31)) as u8,
            ((g as u16 * light_level / 255).min(63)) as u8,
            ((b as u16 * light_level / 255).min(31)) as u8,
        )
    }

    pub const fn get_components(&self) -> (u8, u8, u8) {
        let r = self.rgb565 >> 11;
        let g = (self.rgb565 & 0b0000011111100000) >> 5;
        let b = self.rgb565 & 0b0000000000011111;

        (r as u8, g as u8, b as u8)
    }

    pub const fn get_888(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.get_components();

        return (
            ((r << 3) | (r >> 2)) as u8,
            ((g << 2) | (g >> 4)) as u8,
            ((b << 3) | (b >> 2)) as u8,
        );
    }
}
