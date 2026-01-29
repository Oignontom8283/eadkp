
use super::{Color, Point, Rect, Image};

#[cfg(target_os = "none")]
use alloc::vec::Vec;

#[cfg(target_os = "none")]
use alloc::ffi::CString;

#[cfg(not(target_os = "none"))]
use std::ffi::CString;

use core::ffi::c_char;

pub fn push_rect(rect: Rect, pixels: &[Color]) {
    unsafe {
        eadk_display_push_rect(rect, pixels.as_ptr());
    }
}

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

pub fn push_rect_uniform(rect: Rect, color: Color) {
    unsafe {
        eadk_display_push_rect_uniform(rect, color);
    }
}

pub fn wait_for_vblank() {
    unsafe {
        eadk_display_wait_for_vblank();
    }
}

pub fn draw_string(
    text: &str,
    point: Point,
    large_font: bool,
    text_color: Color,
    background_color: Color,
) {
    let c_string = CString::new(text).expect("Can't convert str to C_String. Maybe invalid caracter.");
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

pub fn push_image(image: &Image, point: Point) {
    push_rect(
        image.for_coordinates(point.x, point.y),
        &image.get_pixels()
    );
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