#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
extern crate eadkp;

use heapless::Vec;
use eadkp::storage;
use alloc::string::{String, ToString};

eadk_setup!(name = "Example");

const FILE_NAME: &str = "test.py";
const DEFAULT_CONTENT: &str = "testing";

#[unsafe(no_mangle)]
fn main() -> isize {
    _eadk_init_heap();

    let mut prev = eadkp::input::KeyboardState::scan();
    let mut log_list: Vec<String, 12> = Vec::new();
    
    let mut log = |message: String| {
        if log_list.len() == log_list.capacity() {
            log_list.remove(0);
        }
        let _ = log_list.push(message);
    };

    log("Storage Init...".to_string());

    // Vérifier si le fichier existe
    let is_existing = storage::file_exists(FILE_NAME);

    match is_existing {
        Ok(true) => {
            log(format!("'{}' Found !", FILE_NAME));
            // Lire le contenu
            match storage::file_read_string(FILE_NAME) {
                Ok(content) => log(format!("Contenu: {}", content)),
                Err(e) => log(format!("Erreur lecture: {:?}", e)),
            }
        },
        Ok(false) => {
            log(format!("'{}' not found. Creating...", FILE_NAME));
            // Créer le fichier avec le contenu par défaut
            match storage::file_write_string(FILE_NAME, DEFAULT_CONTENT) {
                Ok(_) => log("File was created !".to_string()),
                Err(e) => log(format!("Creation Error: {:?}", e)),
            }
        },
        Err(e) => log(format!("Storage Error: {:?}", e)),
    }

    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE);

    // Lire et afficher en hex le u32 a footer_addr
    let footer_value = unsafe { core::ptr::read_unaligned(eadkp::epsilon::storage().footer_addr) };
    log(format!("Footer Value: 0x{:X}", footer_value));

    loop {
        let now = eadkp::input::KeyboardState::scan();
        let just = now.get_just_pressed(prev);
        if just.key_down(eadkp::input::Key::Home) { break 0; };

        for (i, msg) in log_list.iter().enumerate() {
            let y_pos = 10 + (i * (eadkp::SMALL_FONT.height as usize + 2)) as u16;
            eadkp::display::draw_string(
                msg.as_str(),
                eadkp::Point { y: y_pos, x: 5 },
                false,
                eadkp::COLOR_BLACK,
                eadkp::COLOR_WHITE
            );
        }

        prev = now;
    }
}