
// Macro pour configurer le boilerplate d'application EADK
// Cette macro génère toute la configuration embarquée nécessaire
#[macro_export]
macro_rules! eadk_setup {
    (name = $app_name:expr) => {
        eadk_setup!(name = $app_name, icon = concat!(env!("OUT_DIR"), "/icon.nwi"), api_level = 0);
    };
    (name = $app_name:expr, icon = $icon_path:expr, api_level = $api_level:expr) => {
        // Importer les crates nécessaires pour les cibles embarquées
        #[cfg(target_os = "none")]
        use embedded_alloc::LlffHeap as Heap;

        // Configurer l'allocateur global
        #[global_allocator]
        #[cfg(target_os = "none")]
        static HEAP: Heap = Heap::empty();

        // Importer alloc pour les allocations sur le tas
        extern crate alloc;

        // Importations communes pour le développement embarqué
        use alloc::format;

        // Importer les traits du gestionnaire de panique
        use core::panic::PanicInfo;

        // Gestionnaire de panique préconfiguré
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

        // Générer le nom d'application terminé par un null terminator
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

        // Niveau d'API EADK (Pas le choix du niveau pour app externe)
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_api_level")]
        pub static EADK_APP_API_LEVEL: u32 = $api_level;

        // Icône de l'application EADK
        #[used]
        #[cfg(target_os = "none")]
        #[unsafe(link_section = ".rodata.eadk_app_icon")]
        pub static EADK_APP_ICON: [u8; {
            const ICON_DATA: &[u8] = include_bytes!($icon_path);
            const ICON_SIZE: usize = ICON_DATA.len();
            
            const _: () = assert!(ICON_SIZE > 0, "Le fichier d'icône est vide");
            
            ICON_SIZE
        }] = *include_bytes!($icon_path);

        // Fonction d'initialisation de l'application, appelée par le système au démarrage de l'application
        #[cfg(target_os = "none")]
        #[inline]
        fn _eadk_init_heap() {
            use eadkp::heap_size;
            let heap_size_val: usize = heap_size();
            unsafe { HEAP.init(eadkp::HEAP_START as usize, heap_size_val) }
        }

        // Fonction vide pour les cibles non-embarquées
        #[cfg(not(target_os = "none"))]
        #[inline]
        fn _eadk_init_heap() {}
    };
}

/// Macro pour inclure les fichiers d'assets depuis le répertoire assets.
/// Cette macro simplifie l'inclusion des fichiers d'assets en automatisant
/// l'inclusion des octets du répertoire cible des assets avec l'extension `.eif`.
/// 
/// ## Exemple
/// Utilisation :
/// ```
/// static IMG_DATA: &[u8] = include_asset!("images/image1.png");
/// ```
/// Résultat :
/// ```
/// static IMG_DATA: &[u8] = include_bytes!("<racine_du_projet>/target/<cible>/assets/images/image1.png.eif");
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
