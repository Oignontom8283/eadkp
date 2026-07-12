#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
extern crate eadkp;

use heapless::Vec;
use eadkp::storage;
use alloc::string::{String, ToString};
use alloc::format;
mod serial_lib;

eadk_setup!(name = "Specs");

const FILE_NAME: &str = "test.py";
const DEFAULT_CONTENT: &str = "testing";

#[unsafe(no_mangle)]
fn main() -> isize {
    _eadk_init_heap();
    let mut log_list: Vec<String, 20> = Vec::new();
    let mut total_log_bytes: usize = 0;

    let mut log = |message: String| {
        let mut msg = message;
        let max_total = 32 * 1024;

        if msg.len() > max_total {
            let mut trimmed = String::new();
            let keep = max_total.saturating_sub(3);
            for (i, ch) in msg.chars().enumerate() {
                if i >= keep { break; }
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

    // ── Système ──────────────────────────────────────────────────────────────
    log(format!("Version:       {}", eadkp::sys::version()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("Commit:        {}", eadkp::sys::hash_commit()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("Expected ver:  {}", eadkp::sys::expected_version()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("Reset type:    {}", eadkp::sys::last_reset_type()
        .map_or_else(|e| format!("Err: {:?}", e), |v| format!("{:?}", v))));

    log(format!("Clearance:     {}", eadkp::sys::clearance_level()
        .map_or_else(|e| format!("Err: {:?}", e), |v| format!("{:?}", v))));

    log(format!("Slot A:        {}", eadkp::sys::is_slot_a()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    // ── Hardware ─────────────────────────────────────────────────────────────
    log(format!("Device:        {}", eadkp::sys::device_name()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("Serial:        {}", eadkp::sys::serial_number()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v)));

    log(format!("FCC ID:        {}", eadkp::sys::fcc_id()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("PCB version:   {}", eadkp::sys::pcb_version()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    // ── Mémoire ──────────────────────────────────────────────────────────────
    log(format!("FS size:       {} B", eadkp::sys::filesystem_size()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("App RAM:       {} B", eadkp::sys::ext_app_ram_size()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    log(format!("App Flash:     {} B", eadkp::sys::ext_app_flash_size()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    // ── Compilation flags ────────────────────────────────────────────────────
    match eadkp::sys::compilation_flags() {
        Ok(flags) => {
            log(format!("API level:     {}", flags.api_level()));
            log(format!("Security lvl:  {}", flags.security_level()));
            log(format!("3rd party:     {}", flags.third_party_allowed()));
        }
        Err(e) => log(format!("Flags:         Err: {:?}", e)),
    }

    // ── Exam mode ────────────────────────────────────────────────────────────
    log(format!("Exam mode:     {}", eadkp::sys::exam_mode()
        .map_or_else(|e| format!("Err: {:?}", e), |m| format!("{:?} (active={})", m.ruleset, m.active))));

    log("Press Home to exit.".to_string());

    serial_lib::run(&log_list)
}