#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#![allow(non_snake_case)]

// Import the library
#[macro_use]
extern crate eadkp;

// Additional imports needed for the application logic
use postcard::to_slice;
use serde::{Serialize, Deserialize};
use heapless::Vec;
use eadkp::storage;

// Configure EADK application with metadata
// This macro will generate: HEAP, alloc imports, format!, String, panic handler, and EADK metadata
eadk_setup!(name = "Eadkp example");

use alloc::string::String;
use alloc::string::ToString;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct FormatDurationOptions {
    pub show_days: bool,
    pub show_hours: bool,
    pub show_minutes: bool,
    pub show_seconds: bool,
    pub show_millis: bool,
    /// If true, always show the field even if zero (e.g. "0h")
    pub always_show: bool,
}

impl Default for FormatDurationOptions {
    fn default() -> Self {
        Self {
            show_days: true,
            show_hours: true,
            show_minutes: true,
            show_seconds: true,
            show_millis: true,
            always_show: false,
        }
    }
}



fn format_number_with_separator(number: i128, separator: &str) -> String {
    // Buffer for digits and separators, max i128 is 39 digits + separators + sign
    let mut buf = [0u8; 48];
    let mut idx = buf.len();

    let negative = number < 0;
    let mut n = if negative { number.wrapping_neg() as u128 } else { number as u128 };

    if n == 0 {
        idx -= 1;
        buf[idx] = b'0';
    } else {
        let sep_bytes = separator.as_bytes();
        let mut digit_count = 0;
        while n > 0 {
            if digit_count > 0 && digit_count % 3 == 0 && !sep_bytes.is_empty() {
                for &b in sep_bytes.iter().rev() {
                    idx -= 1;
                    buf[idx] = b;
                }
            }
            idx -= 1;
            buf[idx] = b'0' + (n % 10) as u8;
            n /= 10;
            digit_count += 1;
        }
    }

    if negative {
        idx -= 1;
        buf[idx] = b'-';
    }

    // SAFETY: only ASCII written
    unsafe { String::from_utf8_unchecked(buf[idx..].to_vec()) }
}

fn format_number(number: i128) -> String {
    format_number_with_separator(number, ",")
}

#[allow(dead_code)]
fn format_duration(ms: u64) -> String {
    format_duration_with_options(ms, &FormatDurationOptions::default())
}

fn format_duration_with_options(ms: u64, opts: &FormatDurationOptions) -> String {
    // Avoid heap allocations, use a fixed-size buffer and manual formatting
    let millis = ms % 1000;
    let total_seconds = ms / 1000;
    let seconds = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let total_hours = total_minutes / 60;
    let hours = total_hours % 24;
    let days = total_hours / 24;

    // Use a stack buffer for the result (max 32 chars is enough)
    let mut buf = [0u8; 32];
    let mut len = 0;

    macro_rules! push_num_unit {
        ($val:expr, $unit:expr) => {
            {
                // Only show if always_show or value > 0 or previous part exists
                let show = $val > 0 || opts.always_show || len > 0;
                if show {
                    // Write number
                    let mut n = $val;
                    let mut digits = [0u8; 20];
                    let mut dlen = 0;
                    if n == 0 {
                        digits[0] = b'0';
                        dlen = 1;
                    } else {
                        while n > 0 {
                            digits[dlen] = b'0' + (n % 10) as u8;
                            n /= 10;
                            dlen += 1;
                        }
                    }
                    // Reverse digits
                    for i in 0..dlen {
                        buf[len] = digits[dlen - 1 - i];
                        len += 1;
                    }
                    // Write unit
                    let unit = $unit.as_bytes();
                    for &c in unit {
                        buf[len] = c;
                        len += 1;
                    }
                    // Add separator if more fields will follow
                    buf[len] = b':';
                    len += 1;
                }
            }
        };
    }

    if opts.show_days && (opts.always_show || days > 0) {
        push_num_unit!(days, "j");
    }
    if opts.show_hours && (opts.always_show || hours > 0) {
        push_num_unit!(hours, "h");
    }
    if opts.show_minutes && (opts.always_show || minutes > 0) {
        push_num_unit!(minutes, "m");
    }
    if opts.show_seconds && (opts.always_show || seconds > 0) {
        push_num_unit!(seconds, "s");
    }
    if opts.show_millis && (opts.always_show || millis > 0) {
        push_num_unit!(millis, "ms");
    }

    // Remove trailing ':' if present
    if len > 0 && buf[len - 1] == b':' {
        len -= 1;
    }

    // SAFETY: buf is always valid UTF-8 as we only write ASCII
    unsafe { String::from_utf8_unchecked(buf[..len].to_vec()) }
}


#[unsafe(no_mangle)]
fn main() -> isize {
    // Initialize the heap
    _eadk_init_heap();

    // ~~~ Définition des fonctions utilitaires


    #[derive(Serialize, Deserialize, Debug)]
    struct GameData {
        bounces: u64,
        total_time: u64,
        max_time: u64
    }

    fn random_color() -> eadkp::Color {
        eadkp::Color::from_888(
            eadkp::random::randint(1u64, 255) as u8,
            eadkp::random::randint(1u64, 255) as u8,
            eadkp::random::randint(1u64, 255) as u8
        )
    }

    // Convertir des bytes en string hexadécimale
    #[allow(dead_code)]
    fn bytes_to_hex_string(bytes: &[u8]) -> String {
        let mut hex_string = String::new();
        for byte in bytes {
            hex_string.push_str(&format!("{:02x}", byte));
        }
        hex_string
    }

    // Convertir string hexadécimale en bytes
    #[allow(dead_code)]
    fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8, 256>, String> {
        if hex_str.len() % 2 != 0 {
            return Err("Longueur impaire".to_string());
        }

        let mut bytes: Vec<u8, 256> = Vec::new();

        for chunk in hex_str.as_bytes().chunks(2) {
            let hex_byte_str = core::str::from_utf8(chunk).map_err(|_| "Échec de conversion UTF-8".to_string())?;
            let byte_val = u8::from_str_radix(hex_byte_str, 16).map_err(|_| "Caractère hexadécimal invalide".to_string())?;
            bytes.push(byte_val).map_err(|_| "Buffer plein".to_string())?;
        }

        Ok(bytes)
    }

    // Struct → Bytes
    #[allow(dead_code)]
    fn struct_to_bytes(stats: &GameData) -> Option<Vec<u8, 256>> {
        let mut buffer = [0u8; 256];
        if let Ok(used) = to_slice(stats, &mut buffer) {
            let mut result: Vec<u8, 256> = Vec::new();
            for byte in used {
                if result.push(*byte).is_err() {
                    return None;
                }
            }
            Some(result)
        } else {
            None
        }
    }

    // Bytes → Struct
    #[allow(dead_code)]
    fn bytes_to_struct(bytes: &[u8]) -> Option<GameData> {
        if let Ok(stats) = postcard::from_bytes::<GameData>(bytes) {
            Some(stats)
        } else {
            None
        }
    }

    #[cfg(target_os = "none")]
    fn save_data(data: &GameData) -> bool {

        // Convertir data en bytes et sauvegarder
        if let Some(bytes) = struct_to_bytes(&data) {

            // Convertir les bytes en hexadécimal au format UTF-8
            let hex_string = bytes_to_hex_string(&bytes);

            let filename = "bounce_data.py";

            // Si le fichier existe déja, le supprimer
            unsafe {
                if storage::file_exists(filename) {
                    let _ = storage::file_erase(filename);
                }

                // Écrire dans le fichier de sauvegarde                            Le premier caractère sera effacé pour une raison inconnue, il faut donc ajouter un caractère inutile au début, comme un espace.
                let content = format!(" # This file contains the save data for the Bounce app.\n# Do not edit this file manually, as it may corrupt your save data.\n\n{}", hex_string);
                return storage::file_write_string(filename, content.as_str()).is_ok();
            }
        }
        false
    }

    #[cfg(not(target_os = "none"))]
    fn save_data(_data: &GameData) -> bool {
        false
    }

    #[cfg(target_os = "none")]
    fn load_data() -> Option<GameData> {
        use alloc::string::ToString;
        
        let filename = "bounce_data.py";

        // Si le fichier n'existe pas, retourner des données par défaut
        if storage::file_exists(filename) == false {
            return Some(GameData {
                bounces: 15,
                total_time: 15,
                max_time: 15
            });
        }

        // Charger les données depuis le fichier
        let file_content = unsafe { storage::file_read_string(filename).ok()};
        
        // Convertir le pointer en String
        let clean_data = file_content.unwrap_or("").to_string();
        
        // 3. Extraire toutes les lignes qui ne sont pas des commentaires
        let hex_string: String = clean_data.lines()
            .map(|line| line.trim()) // Supprimer espaces et retours chariot
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .collect::<Vec<&str, 256>>()
            .join("");
        
        // 4. Nettoyer la chaîne hex des caractères non-hex
        let clean_hex: String = hex_string.chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect::<String>();
        
        // Diagnostic: vérifier si on a trouvé des données hex
        if clean_hex.is_empty() {
            panic!("Aucune donnée hex trouvée dans le fichier. Contenu nettoyé: '{}', Contenu brut: '{}'", 
                    &hex_string[..core::cmp::min(100, hex_string.len())],
                    &clean_data[..core::cmp::min(100, clean_data.len())]);
        }
        
        // Convertir la chaîne hexadécimale en bytes
        let bytes = match hex_string_to_bytes(&clean_hex) {
            Ok(b) => b,
            Err(e) => panic!("Erreur de conversion hexadecimale: \n{}\n{}", clean_hex, e),
        };
            
        // Convertir les bytes en struct
        let loaded_data = match bytes_to_struct(&bytes) {
            Some(data) => data,
            None => panic!("Erreur de désérialisation postcard. Taille bytes: {}, Premiers bytes: {:?}", 
                            bytes.len(), 
                            &bytes[..core::cmp::min(16, bytes.len())])
        };
                
        // Retourner les données chargées
        Some(loaded_data)
    }

    #[cfg(not(target_os = "none"))]
    fn load_data() -> Option<GameData> {
        None
    }



    // ~~~ Commencement du programme principal



    const RECT_WIDTH: u16 = 60;
    const RECT_HEIGHT: u16 = 50;

    #[allow(unused_mut)]
    let mut _SAVE_EVERY: u64 = 30_000; // Sauvegarder toutes les 30 secondes

    let mut color: eadkp::Color = random_color();
    let BACKGROUND_COLOR: eadkp::Color = eadkp::COLOR_WHITE;
    let TEXT_COLOR: eadkp::Color = eadkp::COLOR_BLACK;

    let mut x: u16 = eadkp::random::randint(1u64, (eadkp::SCREEN_RECT.width - RECT_WIDTH - 1) as u64) as u16;
    let mut y: u16 = eadkp::random::randint(1u64, (eadkp::SCREEN_RECT.height - RECT_HEIGHT - 1) as u64) as u16;

    let mut x_speed: i16 = 5;
    let mut y_speed: i16 = 5;

    let mut bounces: u64 = 0;
    let mut start_total_time: u64 = 0;
    let mut max_time: u64 = 0;

    let startTime = eadkp::timing::millis();

    eadkp::display::push_rect_uniform(eadkp::Rect { x: 0, y: 0, width: eadkp::SCREEN_RECT.width, height: eadkp::SCREEN_RECT.height }, BACKGROUND_COLOR);

    let mut prev = eadkp::input::KeyboardState::scan();

    let mut lastSecond = eadkp::timing::millis() / 1000;
    let mut fps: u32 = 0;
    let mut fps_display: u32 = 0;

    let mut last_save_time = eadkp::timing::millis();

    // Charger les données sauvegardées
    if let Some(loaded_data) = load_data() {
        bounces = loaded_data.bounces;
        start_total_time = loaded_data.total_time;
        max_time = loaded_data.max_time;
    }

    let mut info = String::new();
    let mut info_time = 0;
    let mut on_the_last_frame_drawn = false;

    // Préparer l'image bread une seule fois
    let bread_image = eadkp::Image::from_raw(
        include_image!("bread.png")
    ).expect("Failed to load bread image");

    fn popup(info: &mut String, info_time: &mut u64, text: String, time: u64) {
        *info = text;
        *info_time = eadkp::timing::millis() + time;
    }

    loop {

        let now = eadkp::input::KeyboardState::scan();
        let just = now.get_just_pressed(prev);
        if just.key_down(eadkp::input::Key::Home) { break 0; };

        // Calcul du FPS
        let currentSecond = eadkp::timing::millis() / 1000;
        fps += 1;
        if currentSecond != lastSecond {
            lastSecond = currentSecond;
            fps_display = fps;
            fps = 0;
        }
        

        
        // Sauvegarder l'ancienne position
        let old_x = x;
        let old_y = y;
        
        // Calculer la nouvelle position potentielle
        let new_x = (x as i16 + x_speed).max(0) as u16;
        let new_y = (y as i16 + y_speed).max(0) as u16;
        
        // Vérifier les collisions et ajuster la vitesse
        if new_x + RECT_WIDTH >= eadkp::SCREEN_RECT.width || new_x == 0 {
            x_speed = -x_speed;
            bounces += 1;
            color = random_color();
        }
        if new_y + RECT_HEIGHT >= eadkp::SCREEN_RECT.height || new_y == 0 {
            y_speed = -y_speed;
            bounces += 1;
            color = random_color();
        }
        
        // Appliquer la nouvelle position avec la vitesse corrigée
        x = (x as i16 + x_speed).max(0).min((eadkp::SCREEN_RECT.width - RECT_WIDTH) as i16) as u16;
        y = (y as i16 + y_speed).max(0).min((eadkp::SCREEN_RECT.height - RECT_HEIGHT) as i16) as u16;

        // Attendre le VBlank
        eadkp::display::wait_for_vblank();

        // 1. Effacer l'ancienne position (dessiner en blanc)
        eadkp::display::push_rect_uniform(eadkp::Rect { x: old_x, y: old_y, width: RECT_WIDTH, height: RECT_HEIGHT }, BACKGROUND_COLOR);
        
        // 2. Dessiner à la nouvelle position
        eadkp::display::push_rect_uniform(eadkp::Rect { x: x, y: y, width: RECT_WIDTH, height: RECT_HEIGHT }, color);


        let actual_time = eadkp::timing::millis() - startTime;
        let total_time = start_total_time + actual_time;
        if actual_time > max_time {
            max_time = actual_time;
        }

        let time_format_options = FormatDurationOptions {
            show_days: false,
            show_hours: true,
            show_minutes: true,
            show_seconds: true,
            show_millis: false,
            always_show: false,
        };

        const HEIGHT: u16 = eadkp::SMALL_FONT.height;
        const WIDTH: u16 = eadkp::SMALL_FONT.width;
        eadkp::display::draw_string(&format!("Bounces: {}", format_number(bounces as i128)), eadkp::Point { x: 5, y: 5 + HEIGHT * 0}, false, TEXT_COLOR, BACKGROUND_COLOR);
        eadkp::display::draw_string(&format!("FPS: {}", fps_display), eadkp::Point { x: 5, y: 5 + HEIGHT * 1 }, false, TEXT_COLOR, BACKGROUND_COLOR);
        eadkp::display::draw_string(&format!("Time: {}", format_duration_with_options(actual_time, &time_format_options)), eadkp::Point { x: 5, y: 5 + HEIGHT * 2 }, false, TEXT_COLOR, BACKGROUND_COLOR);

        let total_time_str = format!("Total: {}", format_duration_with_options(total_time, &time_format_options));
        eadkp::display::draw_string(&total_time_str, eadkp::Point { x: eadkp::SCREEN_RECT.width - WIDTH * total_time_str.len() as u16 - 5, y: 5 + HEIGHT * 0 }, false, TEXT_COLOR, BACKGROUND_COLOR);

        let max_time_str = format!("Best: {}", format_duration_with_options(max_time, &time_format_options));
        eadkp::display::draw_string(&max_time_str, eadkp::Point { x: eadkp::SCREEN_RECT.width - WIDTH * max_time_str.len() as u16 - 5, y: 5 + HEIGHT * 1 }, false, TEXT_COLOR, BACKGROUND_COLOR);

        let battery_str = format!("Battery: {}", eadkp::battery::level().to_str().to_uppercase());
        eadkp::display::draw_string(&battery_str, eadkp::Point { x: eadkp::SCREEN_RECT.width - WIDTH * battery_str.len() as u16 - 5, y: 5 + HEIGHT * 2 }, false, TEXT_COLOR, BACKGROUND_COLOR);


        // Gérer l'affichage du popup
        let popup_displayed = !info.is_empty() && eadkp::timing::millis() < info_time;

        const POPUP_DISPLAY_AXE_X:u16 = eadkp::SCREEN_RECT.width / 2;
        const POPUP_DISPLAY_AXE_Y:u16 = eadkp::SCREEN_RECT.height - eadkp::LARGE_FONT.height - 5;

        // Afficher le message popup s'il est encore valide
        if popup_displayed {

            const MAX_CHARACTERS: usize = (eadkp::SCREEN_RECT.width / eadkp::SMALL_FONT.width) as usize; // Nombre maximum de caractères affichables en une ligne

            let truncated_info = info.clone(); // Cloner la chaîne pour éviter de modifier l'original
            let must_be_cuted = truncated_info.len() > MAX_CHARACTERS; // Vérifier si la chaîne doit être coupée

            let cuted_info = &truncated_info[..if must_be_cuted { MAX_CHARACTERS - 3 } else { truncated_info.len() }]; // Couper la chaîne si elle est trop longue
            let display_info = if must_be_cuted { format!("{}...", cuted_info) } else { String::from(cuted_info) };// Chaîne de texte finale à afficher

            eadkp::display::draw_string(
                &display_info,
                eadkp::Point { x: POPUP_DISPLAY_AXE_X - ((display_info.len() as u16 * WIDTH) / 2), y: POPUP_DISPLAY_AXE_Y },
                true,
                TEXT_COLOR,
                BACKGROUND_COLOR
            );
            
            on_the_last_frame_drawn = true;
        }
        // Effacer la zone du popup si on vient de le fermer
        if on_the_last_frame_drawn && !popup_displayed {
            eadkp::display::push_rect_uniform(
                eadkp::Rect { x: 0, y: POPUP_DISPLAY_AXE_Y, width: eadkp::SCREEN_RECT.width, height: eadkp::LARGE_FONT.height },
                BACKGROUND_COLOR
            );
            on_the_last_frame_drawn = false;
        }

        // Afficher l'image bread.png a l'écran
        // println!("Drawing bread image at position ({}, {})", eadkp::SCREEN_RECT.width - bread_image.width, eadkp::SCREEN_RECT.height - bread_image.height);
        // eadkp::display::push_rect_uniform(eadkp::Rect { x: eadkp::SCREEN_RECT.width - bread_image.width, y: eadkp::SCREEN_RECT.height - bread_image.height, width: bread_image.width, height: bread_image.height }, eadkp::Color::from_888(0, 0, 0));
        eadkp::display::push_rect(
            bread_image.for_coordinates(eadkp::SCREEN_RECT.width - bread_image.width, eadkp::SCREEN_RECT.height - bread_image.height),
            bread_image.get_pixels(),
        );

        if just.key_down(eadkp::input::Key::Back) {

            // save
            let _ = save_data(&GameData {
                bounces: bounces,
                total_time: total_time,
                max_time: max_time
            });

            break 0;
        }
        
        // Sauvegarder toutes les 30 secondes
        if eadkp::timing::millis() - last_save_time >= _SAVE_EVERY {
            last_save_time = eadkp::timing::millis();

            popup(&mut info, &mut info_time, String::from("Saving..."), 2000);

            let _ = save_data(&GameData {
                bounces: bounces,
                total_time: total_time,
                max_time: max_time
            });
            
        }

        prev = now;
    }

}
