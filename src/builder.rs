use image::{self, GenericImageView, ImageReader};
use regex::Regex;
use std::{fs, process::Command};
use cc;

mod utils {
    include!("utils.rs");
}

const ASSETS_DIR: &str = "assets/";
const POSSIBLE_C_DIRS: [&str; 4] = ["src/libs", "src/lib", "src/c", "src/cpp"];
const KEYBOARD_MAPPING_FILE: &str = "epsilon_simulator/ion/src/simulator/shared/keyboard.cpp";

const ICON_WIDTH: u32 = 55;
const ICON_HEIGHT: u32 = 56;

macro_rules! cargo_warn {
    ($fmt:expr, $($arg:tt)*) => {
        println!("cargo:warning={}", format!($fmt, $($arg)*))
    };
    ($fmt:expr) => {
        println!("cargo:warning={}", $fmt)
    };
}

macro_rules! cargo_changed {
    ($path:expr) => {
        println!("cargo:rerun-if-changed={}", $path);
    };
}

fn convert_image(file_path: &std::path::Path) {
    let img = ImageReader::open(file_path)
        .unwrap()
        .decode()
        .unwrap();

    let mut converted_pixels: Vec<u8> = Vec::new();

    // Ajouter l'en-tête EIF1 au DÉBUT
    converted_pixels.extend(utils::EIF1_MAGIC_NUMBER.to_le_bytes()); // Magic number (4 bytes)
    converted_pixels.extend((img.width() as u16).to_le_bytes());         // Width (2 bytes)
    converted_pixels.extend((img.height() as u16).to_le_bytes());        // Height (2 bytes)

    // Ensuite ajouter les pixels RGB565
    for pix in img.pixels() { // Convert to RGB565

        // TODO: Utiliser une méthode configuration pour indiquer la couleur de transparence
        // Si le pixel est complètement transparent (ex: background PNG)
        if pix.2.0[3] == 0 {
            // Pixel transparent, le convertir en blanc
            converted_pixels.extend(0xFFFFu16.to_le_bytes()); // Blanc en RGB565
            continue;
        }

        let rgb565 = ((pix.2.0[0] as u16 & 0b11111000) << 8) // Mettre les bits rouges a 15-11
                        | ((pix.2.0[1] as u16 & 0b11111100) << 3) // Mettre les bits verts a 10-5
                        | (pix.2.0[2] as u16 >> 3);               // Mettre les bits bleus a 4-0
        converted_pixels.extend(rgb565.to_le_bytes());  // 2 octets
    }

    // Nom du fichier sans le chemin
    let out_name = file_path.file_name().unwrap().to_str().unwrap();
    
    // Définir le dossier de sortie des assets convertis
    let out_dir = format!("{}/assets", std::env::var("OUT_DIR").unwrap());

    // Créer le dossier OUT_DIR/assets/ s'il n'existe pas
    fs::create_dir_all(&out_dir)
        .expect("Failed to create output assets/ directory");

    // Écrire le fichier converti au format .bin
    fs::write(format!("{}/{}.eif", out_dir, out_name), converted_pixels.as_slice())
        .expect("Failed to write converted image file");
}

pub fn setup() {
    setup_with_options(None, None, None);
}

/// Configure et exécute le processus de build complet avec des options personnalisées
pub fn setup_with_options(
    asset_dir: Option<&str>,
    c_dirs: Option<Vec<&str>>,
    keyboard_mapping_file: Option<&str>
) {


    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let asset_dir = std::path::Path::new(&manifest_dir).join(asset_dir.unwrap_or(ASSETS_DIR));
    let c_dirs: Vec<String> = c_dirs
        .unwrap_or(POSSIBLE_C_DIRS.iter().map(|s| *s).collect())
        .into_iter()
        .map(|d| std::path::Path::new(&manifest_dir).join(d).to_string_lossy().to_string())
        .collect();
    let keyboard_mapping_file = std::path::Path::new(&manifest_dir)
        .join(keyboard_mapping_file.unwrap_or(KEYBOARD_MAPPING_FILE))
        .to_string_lossy()
        .to_string();

    // Créer le dossier assets/ s'il n'existe pas

    fs::create_dir_all(&asset_dir)
        .expect("Failed to create assets/ directory");

    let entries = fs::read_dir(&asset_dir)
        .expect("Failed to read assets/ directory");

    for entry_result in entries {

        // Stoper le build si une erreur survient lors de la lecture d'un fichier
        let entry = entry_result.unwrap_or_else(|err| {
            panic!("Failed to read entry in assets/: {}", err);
        });

        let path = entry.path();

        // Ignorer le fichier s'il n'a pas d'extension
        // Definir 'ext'
        let ext = match path.extension() {
            Some(e) => e,
            None => {
                cargo_warn!("Ignoring file without extension: {}", path.display());
                continue;
            },
        };

        // ignorer le fichier s'il n'a pas de nom
        // Definir 'name'
        let name = match path.file_stem() {
            Some(e) => e,
            None => {
                cargo_warn!("No file stem (name) for file: {}", path.display());
                continue;
            }
        };

        // Ignorer le fichier s'il n'est pas un .png
        if ext != "png" {
            cargo_warn!("Ignoring non-png file: {}", path.display());
            continue;
        };

        // Si le fichier est l'icône, le convertir en .nwi
        // Sinon, le convertir en .bin
        if name == "icon" {

            // Vérifier que l'image est de la bonne taille (55×56 pixels)
            
            // Charger l'image
            let img = ImageReader::open(&path);

            // Stoper le build si une erreur survient lors de l'ouverture de l'image
            let img = img.unwrap_or_else(|err| {
                panic!("Failed to open icon image {}: {}", path.display(), err);
            });

            // Stoper le build si une erreur survient lors du décodage de l'image
            let img = img.decode().unwrap_or_else(|err| {
                panic!("Failed to decode icon image {}: {}", path.display(), err);
            });

            let (width, height) = img.dimensions();
            assert_eq!(width, ICON_WIDTH, "Icon width must be {} pixels", ICON_WIDTH); // Stoper le build si la largeur est incorrecte
            assert_eq!(height, ICON_HEIGHT, "Icon height must be {} pixels", ICON_HEIGHT); // Stoper le build si la hauteur est incorrecte
            
            // Utiliser nwlink pour convertir l'image en .nwi
            // Définir le chemin de sortie dans OUT_DIR
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let icon_out_path = format!("{}/icon.nwi", out_dir);
            let output = Command::new("sh")
                .arg("-c")
                .arg(format!("npx --yes -- nwlink@0.0.19 png-nwi {} {}", path.display(), icon_out_path))
                .output()
                .expect("Failed to run nwlink for icon.nwi");
            // Stoper le build si nwlink retourne une erreur
            assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        } else {
            // Convertir l'image en binaire RGB565
            convert_image(&path);
        }

    };
        


    // Compilation et linkage des fichiers C/C++ présent dans src/libs/, src/lib/, src/c ou src/cpp
    
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let is_embedded = target_os == "none";
    
    // Configuration du compilateur C pour ARM (architecture de la NumWorks) uniquement pour l'embarqué
    if is_embedded {
        unsafe { std::env::set_var("CC", "arm-none-eabi-gcc") };
    }

    // Récupération des flags de compilation spécifiques à EADK via nwlink (uniquement pour l'embarqué)
    let nwlink_flags = if is_embedded {
        let program = "npx";
        String::from_utf8(
            Command::new(program)
                .args(["--yes", "--", "nwlink@0.0.19", "eadk-cflags"])
                .output()
                .expect("Failed to get nwlink eadk-cflags")
                .stdout,
        )
        .expect("Invalid UTF-8 in nwlink flags")
    } else {
        String::new()
    };
    
    let mut build = cc::Build::new();
    let mut has_files = false;
        
    // Fonction récursive pour ajouter les fichiers C/C++ d'un répertoire
    fn add_c_files_recursive(dir: &std::path::Path, build: &mut cc::Build) -> Result<bool, std::io::Error> {
        let mut found = false;

        // Obtenir la liste du contenu du répertoire
        let entries = match fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(e) => return Err(e),
        };
        
        // Parcourir chaque entrée dans le répertoire
        for entry in entries.flatten() {
            let path = entry.path();

            // Si l'entrée est un répertoire, descendre récursivement
            if path.is_dir() {
                
                // Descendre récursivement dans les sous-répertoires
                match add_c_files_recursive(&path, build) {
                    Ok(sub_found) => found |= sub_found, // True si 'found' est déja true alors que 'sub_found' est false. Sinon, prend la valeur de 'sub_found'.
                    Err(e) => return Err(e), // Propager l'erreur vers l'appelant
                };
            }
            // Sinon, vérifier l'extension du fichier
            else if let Some(ext) = path.extension() {

                // Vérifier si le fichier est un .c ou .cpp
                if ext == "c" || ext == "cpp" {
                    // Indiquer à Cargo de relancer le build si ce fichier change
                    cargo_changed!(path.display());
                    // Ajouter le fichier à la liste de compilation
                    build.file(path);
                    found = true;
                };
            };
        };

        // You may want to implement the recursive logic here, or leave as is.
        Ok(found)
    }
    
    // Parcourir chaque répertoire racine pour trouver les fichiers C/C++
    for dir_path in c_dirs {
        let libs_dir = std::path::Path::new(&dir_path);
        if libs_dir.exists() {
            // Ajouter les fichiers trouvés dans ce répertoire
            has_files |= match add_c_files_recursive(libs_dir, &mut build) {
                Ok(found) => found, // True si des fichiers ont été trouvés
                Err(e) => panic!("Failed to read C/C++ files from {}: {}", dir_path, e), // Panique en cas d'erreur
            };   
        };
    };
    
    // Configuration des flags de compilation C
    build.flag("-std=c99");      // Standard C99
    build.flag("-Wall");         // Tous les avertissements
    build.flag("-ggdb");         // Informations de debug pour GDB
    build.warnings(false);       // Ne pas traiter les warnings comme des erreurs
    
    // Flags spécifiques à l'embarqué (optimisation taille et sections séparées)
    if is_embedded {
        build.flag("-Os");           // Optimisation pour la taille (important pour l'embarqué)
        build.flag("-ffunction-sections");  // Chaque fonction dans sa propre section
        build.flag("-fdata-sections");      // Chaque variable dans sa propre section
        
        // Ajouter les flags spécifiques à EADK (chemins d'include, macros, etc.)
        for flag in nwlink_flags.split_whitespace() {
            build.flag(flag);
        }
    }
    
    // Si aucun fichier C/C++ n'a été trouvé, créer un fichier vide
    // pour éviter les erreurs du linker (qui attend une bibliothèque)
    if !has_files {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let empty_c = format!("{}/empty.c", out_dir);
        fs::write(&empty_c, "// Empty C file to satisfy linker when no C/C++ files are present\n").unwrap();
        build.file(&empty_c);
    }
    
    // Compiler et créer la bibliothèque statique libnative_libs.a
    // Ce nom doit correspondre au flag -lnative_libs dans .cargo/config.toml
    build.compile("native_libs");
    

    // Remapper les touches du simulateur NumWorks en modifiant keyboard.cpp
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "none" {
        if let Some(keyboard_file) = Some(&keyboard_mapping_file) {
        cargo_changed!(keyboard_file);
        
        // Nouveau mapping de touches personnalisé pour le simulateur
        let remapped = "constexpr static KeySDLKeyPair sKeyPairs[] = {\
  KeySDLKeyPair(Key::OK,        SDL_SCANCODE_RETURN),\
  KeySDLKeyPair(Key::Back,      SDL_SCANCODE_BACKSPACE),\
  KeySDLKeyPair(Key::EXE,       SDL_SCANCODE_ESCAPE),\
\
  KeySDLKeyPair(Key::Var,       SDL_SCANCODE_I),\
\
  KeySDLKeyPair(Key::Toolbox,   SDL_SCANCODE_W),\
  KeySDLKeyPair(Key::Imaginary, SDL_SCANCODE_A),\
  KeySDLKeyPair(Key::Power,     SDL_SCANCODE_D),\
  KeySDLKeyPair(Key::Comma,     SDL_SCANCODE_S),\
  KeySDLKeyPair(Key::Shift,     SDL_SCANCODE_SPACE),\
  KeySDLKeyPair(Key::Exp,       SDL_SCANCODE_LSHIFT),\
\
  KeySDLKeyPair(Key::Down,      SDL_SCANCODE_DOWN),\
  KeySDLKeyPair(Key::Up,        SDL_SCANCODE_UP),\
  KeySDLKeyPair(Key::Left,      SDL_SCANCODE_LEFT),\
  KeySDLKeyPair(Key::Right,     SDL_SCANCODE_RIGHT),\
};";

        // Lire le fichier de configuration dukeyboard_filer
        let file_content = fs::read_to_string(keyboard_file)
        .expect("Cannot open keyboard.cpp file from emulator. Please check if the simulator is clonned properly.");

        // Vérifier si le mapping n'est pas déjà appliqué pour éviter les réécritures inutiles
        if !file_content.contains(remapped) {
            // Utiliser une regex pour trouver et remplacer l'ancien tableau de mapping
            // Pattern: "constexpr static KeySDLKeyPair sKeyPairs[] = { ... };"
            let re = Regex::new(r"constexpr static KeySDLKeyPair sKeyPairs\[] ?= ?\{[\S\s]*?};")
                .unwrap();
            let _result = re.replace(&file_content, remapped);

            // Écrire le nouveau contenu avec le mapping personnalisé
            // fs::write(
            //     KEYBOARD_MAPPING_FILE,
            //     result.as_bytes(),
            // )
            // .unwrap();
            // Désactivation du mapping custom de touches.
        }
        }
    }
}
