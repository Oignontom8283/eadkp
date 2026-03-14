#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
extern crate eadkp;

use heapless::Vec;
use eadkp::storage;
use alloc::string::{String, ToString};
mod serial;

eadk_setup!(name = "Example");

const FILE_NAME: &str = "test.py";
const DEFAULT_CONTENT: &str = "testing";

#[unsafe(no_mangle)]
fn main() -> isize {
    _eadk_init_heap();
    let mut log_list: Vec<String, 12> = Vec::new();
    let mut total_log_bytes: usize = 0;

    let mut log = |message: String| {
        let mut msg = message;
        let max_total = 32 * 1024;

        if msg.len() > max_total {
            let mut trimmed = String::new();
            let keep = max_total.saturating_sub(3);
            for (i, ch) in msg.chars().enumerate() {
                if i >= keep {
                    break;
                }
                trimmed.push(ch);
            }
            trimmed.push_str("...");
            msg = trimmed;
        }

        while total_log_bytes + msg.len() > max_total && !log_list.is_empty() {
            let removed = log_list.remove(0);
            total_log_bytes = total_log_bytes.saturating_sub(removed.len());
        }

        if log_list.len() == log_list.capacity() {
            let removed = log_list.remove(0);
            total_log_bytes = total_log_bytes.saturating_sub(removed.len());
        }

        total_log_bytes += msg.len();
        let _ = log_list.push(msg);
    };

    log("Storage Init...".to_string());

    let voltage = eadkp::battery::voltage();
    let level = eadkp::battery::level();
    let percent = eadkp::battery::percentage();
    log(format!("Battery: {:.2}V", voltage));
    log(format!("Battery level: {}", level.to_str()));
    log(format!("Battery: {}%", percent));

    let is_existing = storage::file_exists(FILE_NAME);

    match is_existing {
        Ok(true) => {
            log(format!("'{}' Found !", FILE_NAME));
            match storage::file_read_string(FILE_NAME) {
                Ok(content) => log(format!("Content: {}", content)),
                Err(e) => log(format!("Read error: {:?}", e)),
            }
        }
        Ok(false) => {
            log(format!("'{}' not found. Creating...", FILE_NAME));
            match storage::file_write_string(FILE_NAME, DEFAULT_CONTENT) {
                Ok(_) => log("File was created !".to_string()),
                Err(e) => log(format!("Creation error: {:?}", e)),
            }
        }
        Err(e) => log(format!("Storage error: {:?}", e)),
    }

    let random_file = format!("test_{}.py", eadkp::random::random_c());
    match storage::file_write_string(&random_file, DEFAULT_CONTENT) {
        Ok(_) => log("File created (forced).".to_string()),
        Err(e) => log(format!("Create error: {:?}", e)),
    }

    match storage::file_exists(&random_file) {
        Ok(true) => log("File exists after create.".to_string()),
        Ok(false) => log("File missing after create.".to_string()),
        Err(e) => log(format!("Exists check error: {:?}", e)),
    }

    match unsafe { storage::file_erase(&random_file) } {
        Ok(_) => log("File deleted.".to_string()),
        Err(e) => log(format!("Delete error: {:?}", e)),
    }

    match storage::file_exists(&random_file) {
        Ok(true) => log("File still exists after delete.".to_string()),
        Ok(false) => log("File deleted successfully.".to_string()),
        Err(e) => log(format!("Exists check error: {:?}", e)),
    }

    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE);

    #[cfg(target_os = "none")]
    let footer_value = {
        log("Reading footer value...".to_string());
        unsafe { core::ptr::read_unaligned(eadkp::epsilon::storage().footer_addr) }
    };

    #[cfg(not(target_os = "none"))]
    let footer_value = { 0x00000000 };

    log(format!("Footer Value: 0x{:X}", footer_value));

    log("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee".to_string());
    log("This is a long message that should be truncated in the log display to ensure it doesn't overflow the buffer.".to_string());
    log("lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string());
    log("lorem ipsum dolor sit amet,".to_string());
    log("consectetur adipiscing elit,".to_string());
    log("sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string());
    log("consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim venia".to_string());

    log("Press Home to exit.".to_string());

    serial::run(&log_list)
}