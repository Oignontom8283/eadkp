#![cfg_attr(target_os = "none", no_std)]
#![no_main]
#[macro_use]
extern crate eadkp;

use heapless::Vec;
use alloc::{format, string::{String, ToString}};
mod serial_lib;

eadk_setup!(name = "Specs");

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

    let fs_size = eadkp::sys::filesystem_size();
    let app_ram = eadkp::sys::ext_app_ram_size();
    let apps_flash = eadkp::sys::ext_app_flash_size();
    let heap_size = eadkp::allocator::total_size();
    let heap_used = eadkp::allocator::used();
    let heap_used_percent = eadkp::allocator::usage_percent();

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

    // ── Hardware ─────────────────────────────────────────────────────────────
    log(format!("Serial:        {}", eadkp::sys::serial_number()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v)));

    log(format!("FCC ID:        {}", match eadkp::sys::fcc_id() {
        Ok(id) => id.to_string(),
        Err(eadkp::GlobalError::Software(eadkp::SoftwareError::NotAvailable { .. })) => "NA".to_string(),
		Err(e) => format!("Err: {:?}", e),
    }));

    log(format!("PCB version:   {}", eadkp::sys::pcb_version()
        .map_or_else(|e| format!("Err: {:?}", e), |v| v.to_string())));

    // ── Mémoire ──────────────────────────────────────────────────────────────
    log(format!("FS size:       {} ({} B)",
        fs_size.as_ref().map_or_else(|e| format!("Err: {:?}", e), |v| eadkp::utils::format_size(*v, true, 2)),
        fs_size.as_ref().map_or_else(|_| format!("None"), |v| eadkp::utils::format_number(*v as f64, ',', 0))
    ));

    log(format!("App RAM:       {} ({} B)",
        app_ram.as_ref().map_or_else(|e| format!("Err: {:?}", e), |v| eadkp::utils::format_size(*v, true, 2)),
        app_ram.as_ref().map_or_else(|_| format!("None"), |v| eadkp::utils::format_number(*v as f64, ',', 0))
    ));

    log(format!("App Flash:     {} ({} B)",
        apps_flash.as_ref().map_or_else(|e| format!("Err: {:?}", e), |v| eadkp::utils::format_size(*v, false, 2)),
        apps_flash.as_ref().map_or_else(|_| format!("None"), |v| eadkp::utils::format_number(*v as f64, ',', 0))
    ));

    // ── mode ────────────────────────────────────────────────────────────
    log(format!("Exam mode:     {}", eadkp::sys::exam_mode()
        .map_or_else(|e| format!("Err: {:?}", e), |m| format!("{:?} (active={})", m.ruleset, m.active))));

	log(format!("Kernel debug:  {}", eadkp::sys::kernel_flags()
		.map_or_else(|e| format!("Err: {:?}", e), |v| (!v.is_production_build()).to_string())));

    log("Press Home to exit.".to_string());

    serial_lib::run(&log_list)
}