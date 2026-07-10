
use crate::{ common::{Rect, Point, Color, Image, ImageFormat} };
use alloc::{ vec::Vec, ffi::{CString} };
use core::ffi::{c_char};

use alloc::vec::Vec;
use alloc::ffi::CString;
use core::ffi::c_char;

/// Pousser un rectangle de pixels (buffer) à l'écran.
pub fn push_rect(rect: Rect, pixels: &[Color]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

/// Récupérer un rectangle de pixels (buffer) depuis l'écran.
pub fn pull_rect(rect: Rect) -> Vec<Color> {
    let size = rect.width as usize * rect.height as usize;
    let mut vec: Vec<Color> = Vec::with_capacity(size);
    for _ in 0..size {
        use crate::COLOR_BLACK;

        vec.push(COLOR_BLACK);
    }

    unsafe {
        eadk_display_pull_rect(rect, vec.as_mut_slice().as_mut_ptr());
    }

    vec
}

/// Pousser un rectangle de pixels de couleur uniforme à l'écran.
pub fn push_rect_uniform(rect: Rect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

/// Attendre le prochain VBlank (Vertical Blank) pour synchroniser les mises à jour de l'écran et éviter les artefacts visuels.
pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}

/// Dessiner une chaîne de caractères à l'écran. Couleurs du texte et du fond ansi que la taille de la police doivent être spécifiées.
pub fn draw_string(
    text: &str,
    point: Point,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let c_string = CString::new(text).expect("Failed to convert string to C string");
    unsafe {
        eadk_display_draw_string(
            c_string.as_ptr(),
            point,
            large_font,
            text_color,
            background_color,
        )
    }
}
// TODO: Faire une version unsafe de draw_string qui prend un *const c_char

/// Pousser une image à l'écran à partir de ses pixels et de sa position.
pub fn push_image(image: &Image, point: Point) {
    image.draw(point);
}


unsafe extern "C" {
    fn eadk_display_push_rect_uniform(rect: Rect, color: Color);
    fn eadk_display_push_rect(rect: Rect, color: *const Color);
    fn eadk_display_wait_for_vblank();
    fn eadk_display_pull_rect(rect: Rect, color: *mut Color);
    fn eadk_display_draw_string(
        text: *const c_char,
        point: Point,
        large_font: bool,
        text_color: Color,
        background_color: Color,
    );
}