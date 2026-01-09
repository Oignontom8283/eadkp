// Macro to set up EADK application boilerplate
// This macro generates all the necessary embedded configuration
#[macro_export]
macro_rules! eadk_setup {
    (name = $app_name:expr) => {
        eadk_setup!(name = $app_name, icon = "../target/icon.nwi", api_level = 0);
    };
    (name = $app_name:expr, icon = $icon_path:expr, api_level = $api_level:expr) => {
        // Import necessary crates for embedded targets
        #[cfg(target_os = "none")]
        use embedded_alloc::LlffHeap as Heap;

        // Set up global allocator
        #[global_allocator]
        #[cfg(target_os = "none")]
        static HEAP: Heap = Heap::empty();

        // Import alloc for heap allocations
        #[cfg(target_os = "none")]
        extern crate alloc;

        // Common imports for embedded development
        #[cfg(target_os = "none")]
        use alloc::format;
        
        #[cfg(target_os = "none")]
        use alloc::string::String;

        #[cfg(target_os = "none")]
        use core::panic::PanicInfo;

        // Panic handler préconfiguré
        #[cfg(target_os = "none")]
        #[panic_handler]
        fn panic(panic: &PanicInfo<'_>) -> ! {
            fn write_wrapped(text: &str, limit: usize) {
                use alloc::string::String;
                let mut line_count = 0;
                let mut line = String::new();
                
                for i in 0..text.len() {
                    line.push(text.as_bytes()[i] as char);
                    if line.len() >= limit || text.as_bytes()[i] as char == '\n' || i >= text.len() - 1 {
                        eadkp::display::draw_string(
                            line.as_str(),
                            eadkp::Point { x: 10, y: (10 + 20 * line_count) as u16 },
                            false,
                            eadkp::Color { rgb565: 65503 },
                            eadkp::Color { rgb565: 63488 },
                        );
                        line.clear();
                        line_count += 1;
                    }
                }
            }

            eadkp::display::push_rect_uniform(
                eadkp::Rect { x: 0, y: 0, width: 320, height: 240 },
                eadkp::Color { rgb565: 63488 },
            );
            
            write_wrapped(format!("{}", panic).as_str(), 42);
            
            loop {
                eadkp::timing::msleep(50);
            }
        }

        // Generate null-terminated app name
        const _APP_NAME_STR: &str = $app_name;
        const _APP_NAME_LEN: usize = _APP_NAME_STR.len();
        
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_name")]
        pub static EADK_APP_NAME: [u8; _APP_NAME_LEN + 1] = {
            let mut arr = [0u8; _APP_NAME_LEN + 1];
            let bytes = _APP_NAME_STR.as_bytes();
            let mut i = 0;
            while i < _APP_NAME_LEN {
                arr[i] = bytes[i];
                i += 1;
            }
            arr
        };

        // EADK API level
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_api_level")]
        pub static EADK_APP_API_LEVEL: u32 = $api_level;

        // EADK app icon
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_icon")]
        pub static EADK_APP_ICON: [u8; {
            const ICON_DATA: &[u8] = include_bytes!($icon_path);
            const ICON_SIZE: usize = ICON_DATA.len();
            
            const _: () = assert!(ICON_SIZE > 0, "Icon file is empty");
            
            ICON_SIZE
        }] = *include_bytes!($icon_path);

        // Helper function to initialize the heap
        #[cfg(target_os = "none")]
        #[inline]
        fn _eadk_init_heap() {
            use eadkp::heap_size;
            let heap_size_val: usize = heap_size();
            unsafe { HEAP.init(eadkp::HEAP_START as usize, heap_size_val) }
        }

        // Dummy function for non-embedded targets
        #[cfg(not(target_os = "none"))]
        #[inline]
        fn _eadk_init_heap() {}
    };
}

/// Macro to include asset files from the assets directory.
/// This macro simplifies the inclusion of asset files by automatically.
/// 
/// Including the bytes from the target assets directory with a .bin extension.
/// 
/// ## Example
/// Usage:
/// ```
/// static IMG_DATA: &[u8] = include_asset!("images/image1.png");
/// ```
/// Output:
/// ```
/// static IMG_DATA: &[u8] = include_bytes!("<project_root>/target/<cible>/assets/images/image1.png.bin");
/// ```
#[macro_export]
macro_rules! include_image {
    ($path:literal) => {
        include_bytes!(concat!(
            env!("OUT_DIR"),
            "/assets/",
            $path,
            ".eif",
        ))
    };
}