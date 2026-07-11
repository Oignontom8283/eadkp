
/*!
# Module de Gestion du Stockage pour les Applications Externes Epsilon sur Calculatrices NumWorks.

Fournit des fonctions pour lire, écrire et gérer les fichiers dans le système de fichiers.

Ce module est à l'origine un portage en Rust de `storage.c` (Licence MIT) provenant du 
projet **NumWorks Extapp Storage**. Cependant, de lourdes modifications on été apportées.
La forme et la logique actuelle diffèrent donc significativement de l'origial.

## Crédits

Auteur original : **[Yaya Cout](https://github.com/Yaya-Cout)**

Fichier source original : [numworks-extapp-storage/src/storage.c](https://framagit.org/Yaya.Cout/numworks-extapp-storage/-/blob/master/src/storage.c)

Un merci tout particulier à Yaya Cout pour son travail d'ingénierie remarquable sur la 
manipulation du stockage, sans lequel ce module n'aurait probablement jamais vu le jour.
*/

use crate::{
    SoftwareError, GlobalError, StorageError,
    epsilon::{storage},
    constant
};
use core::{ ptr, slice, str };
use alloc::{ vec::Vec, string::{ToString} };


// ============================================================================
// STORAGE OPERATIONS  
// ============================================================================


/// Vérifie que le stockage semble valide. **Ne vérifie pas l'integrité des fichiers !**
#[cfg(target_os = "none")]
pub fn is_valid_storage() -> Result<(), GlobalError> {
    let storage = storage();
    storage.is_valid().map_err(|_| SoftwareError::InvalidStorage.into())
}

/// Vérifie que la string est un c string valide (pas de null byte à l'intérieur) et pas vide (un nom de fichier vide n'est pas autorisé)
pub(crate) fn is_valid_cstring(s: &str) -> bool {
    !s.as_bytes().contains(&0) && !s.is_empty()
}

/// Trouve le pointeur vers le null terminator d'une string commençant à `start`,
/// sans dépasser `max` dans un maxium de `epsilon::STORAGE_FILE_MAX_NAME_LEN`.
pub(crate) fn strnend(start:*const u8, max:*const u8) -> Result<*const u8, StorageError> {
    //TODO: Separer la fonction en deux, une général pour trouver le nt et celle ici avec limite STORAGE_FILE_MAX_NAME_LEN
    unsafe  {
        // limite d'Epsilon et l'imite donnée
        let len = (max.offset_from(start) as usize).min(constant::STORAGE_FILE_MAX_NAME_LEN);
        
        for offset in 0..len {
            if *start.add(offset) == 0 {
                return Ok(start.add(offset));
            }
        }
        Err(StorageError::NullTerminatorNotFound { start })
    }
}

/// Trouve la prochaine position libre dans le stockage
/// 
/// Retourne un pointeur vers le début de la fin de l'espace utilisé (le prochain enregistrement vide).
/// Si le stockage est plein, retourne l'adresse de fin du stockage utilisable
/// 
/// @unchecked
#[cfg(target_os = "none")]
#[doc(hidden)]
pub fn next_free() -> *const u8 {

    let storage = storage();
    let usable_end_addr = storage.usable_end_addr;
    let mut offset = storage.usable_start_addr;

    while offset < usable_end_addr {
        let size = unsafe { ptr::read_unaligned(offset as *const u16) };
        if size == 0 {
            return offset;
        }
        offset = unsafe { offset.add(size as usize) };
    }

    usable_end_addr
}


/// Structure pour représenter un fichier.
#[doc(hidden)]
pub struct FileEntry {
    size: usize,
    ptr: *const u8,
    #[allow(unused)]
    name: *const u8,
    content: *const u8,
    content_size: usize,
}

/// Trouve un fichier par son nom et retourne une structure contenant des pointeurs vers son header, son nom et son contenu, ainsi que sa taille.
#[doc(hidden)]
#[cfg(target_os = "none")]
pub fn _find_file(filename: &str) -> Result<FileEntry, StorageError> {
    let filename_slice = filename.as_bytes();
    let filename_len = filename_slice.len();

    let storage = storage();
    let storage_start = storage.usable_start_addr;
    let storage_end = storage.usable_end_addr;

    let mut offset = storage_start;

    unsafe {
        while offset < storage_end {
            let size = ptr::read_unaligned(offset as *const u16) as usize;
            if size == 0 { break; }

            let name_ptr = offset.add(2);
            let name_candidate = slice::from_raw_parts(name_ptr, filename_len);
            let name_null_terminator = name_ptr.add(filename_len);

            if name_candidate == filename_slice && *name_null_terminator == 0 {
                let content_ptr = name_ptr.add(filename_len + 1);
                let content_size = size - 2 - (filename_len + 1);
                return Ok(FileEntry { size: size, ptr: offset, name: name_ptr, content: content_ptr, content_size });
            }

            offset = offset.add(size);
        }
    }

    Err(StorageError::FileNotFound)
}


/// Trouve tous les fichiers dont le nom se termine par un suffix donné et retourne une liste de leurs noms
/// 
/// ## Exemple
/// ```
/// fn display(file: &str) { ... } // Fonction d'affichage fictive
/// 
/// let txt_files = find_files_with_suffix(".txt")?;
/// for file in txt_files {
///     display(file); // Affiche tous les fichiers se terminant par .txt
/// }
/// ```
#[cfg(target_os = "none")]
pub fn find_files_with_suffix(suffix: &str) -> Result<Vec<&str>, GlobalError> {

    // Vérifier que le suffix est un c string valide (pas de null byte à l'intérieur) et pas vide (un suffix vide correspondrait à tous les fichiers)
    if !is_valid_cstring(suffix) {
        return Err(SoftwareError::InvalidParameter { param_name: "suffix".to_string(), details: "suffix is empty or contains null bytes".to_string() }.into());
    }

    let suffix_slice = suffix.as_bytes();
    let suffix_len = suffix_slice.len();

    let storage = storage();
    let storage_start = storage.usable_start_addr;
    let storage_end = storage.usable_end_addr;

    let mut offset = storage_start;
    let mut matching_files = Vec::new();

    unsafe {
        while offset < storage_end {
            let size = ptr::read_unaligned(offset as *const u16) as usize;
            if size == 0 { break; }

            let name_ptr = offset.add(2);
            
            // let mut nt_ptr = name_ptr; // Trouver l'adr du nt
            // while *nt_ptr != 0 {
            //     nt_ptr = nt_ptr.add(1);
            // }
            let nt_ptr = strnend(name_ptr, offset.add(size))?;

            let name_len = nt_ptr.offset_from(name_ptr) as usize;
            
            // Vérifier que le nom est assez long pour contenir le suffix
            if name_len >= suffix_len {
                let suffix_candidate_ptr = nt_ptr.sub(suffix_len);
                let suffix_candidate = slice::from_raw_parts(suffix_candidate_ptr, suffix_len);
                
                if suffix_candidate == suffix_slice {
                    // Si trouvé une correspondance, extraire le nom complet du fichier pour le push
                    let name_slice = slice::from_raw_parts(name_ptr, name_len);
                    let name_str = str::from_utf8_unchecked(name_slice);  
                    
                    matching_files.push(name_str);
                }
            }

            offset = offset.add(size);
        }
    }

    Ok(matching_files)
}

#[cfg(not(target_os = "none"))]
pub fn find_files_with_suffix(_suffix: &str) -> Result<Vec<&str>, GlobalError> {
    Ok(Vec::new())
}


/// Calcule l'espace libre restant dans le stockage
/// 
/// Retourne la différence entre l'adresse de fin du stockage utilisable et l'adresse de la position libre actuelle.
/// Si 0, le stockage est plein.
/// 
/// @unchecked
#[cfg(target_os = "none")]
pub fn available_space() -> usize {

    let free_addr = next_free() as usize; // Adresse de la prochaine position libre
    let usable_end = storage().usable_end_addr as usize; // Adresse fin stockage utilisable (adresse du footer)

    // Retourner l'espace libre restant, en soustrayant l'adresse de la prochaine position libre de l'adresse de fin du stockage utilisable
    usable_end - free_addr
}


/// Sous-fonction de `can_store()`. Utilise directement la longueur du contenu sans pointeur. Se réfère à `can_store()` pour la documentation complète.
#[cfg(target_os = "none")]
pub fn can_store_len(content_len: usize, filename: &str) -> Result<(), GlobalError> {
    let filename_size = filename.len() + 1; // +1 pour le null terminator
    let total_size = 2 + filename_size + content_len; // 2 bytes pour la taille du header

    // Check que le nom peut être une c string valide
    if !is_valid_cstring(filename) {
        return Err(StorageError::StorageInvalidName { length: filename_size, string: filename.to_string() }.into());
    }

    // Check nom < 255 bytes (limitation Epsilon)
    if filename_size > u8::MAX as usize {
        return Err(StorageError::StorageInvalidName { length: filename_size, string: filename.to_string() }.into());
    }

    // Check que le content n'est pas vide
    if content_len == 0 {
        return Err(StorageError::FileContentEmpty.into());
    }

    // Check total_size < 65535 bytes (limitation du header sur 2 bytes)
    if total_size > u16::MAX as usize {
        return Err(StorageError::FileTooLarge { max_size: u16::MAX as usize, actual_size: total_size }.into());
    }

    if available_space() >= total_size {
        Ok(())
    } else {
        Err(StorageError::StorageOverflow { available: available_space(), needed: total_size }.into())
    }
}

#[cfg(not(target_os = "none"))]
pub fn can_store_len(_content_len: usize, _filename: &str) -> Result<(), GlobalError> {
    Ok(())
}


/// Vérifie si un fichier peut être stocké en respectant les contraintes suivantes :
/// - Espace disponible suffisant
/// - Nom du fichier ≤ 255 bytes (limite Epsilon)
/// - Nom du fichier valide (pas de null byte à l'intérieur)
/// - Taille totale (header + nom + contenu) ≤ 65535 bytes (u16 max)
/// Retourne `true` si possible, sinon `false`.
/// 
/// ## Exemple
/// ```
/// let content = b"Hello, world!";
/// let filename1 = "greeting.txt";
/// assert!(can_store(content1, filename1).is_ok()); // Vérifie que le fichier peut être stocké
/// 
/// let filename2 = "invalid\0name.txt";
/// let content2 = b"Some content";
/// assert!(can_store(content2, filename2).is_err()); // Vérifie que le nom de fichier invalide est rejeté
/// ```
/// 
/// @unchecked
#[cfg(target_os = "none")]
pub fn can_store(content: &[u8], filename: &str) -> Result<(), GlobalError> {
    can_store_len(content.len(), filename)
}


/// Écrit un fichier dans le stockage en utilisant un ou plusieur(s) segments de contenu (Pas de copie, directement écrit a l'emplacement)
/// 
/// ## Specifications
/// - Les données sont écrites dans l'ordre de la liste fournie.
/// - Aucune copie intermédiaire n'est effectuée
/// 
/// ## Exemple avec le format de fichier python d'Epsilon:
/// ```
/// let filename = "greeting.py";
/// 
/// let segment1 = [0x1];                     // Metadata         Écrit en premier
/// let segment2 = "Hello world!".as_bytes(); // contenu          Écrit en second
/// let segment3 = [0x0];                     // Null terminator  Écrit en dernier
/// 
/// let segments = [&segment1, &segment2, &segment3]; // Assemblage en liste
/// file_write_segments(filename, &segments)?; // Écrit le fichier
/// ``` 
#[cfg(target_os = "none")]
pub fn file_write_segments(filename: &str, segments: &[&[u8]]) -> Result<(), GlobalError> {
    let total_content_size: usize = segments.iter().map(|s| s.len()).sum();

    is_valid_storage()?;
    can_store_len(total_content_size, filename)?;

    // can_store_lken vérifie déja que le nom sois valide

    let write_pos = next_free() as *mut u8; // adr du nouveau fichier (début)

    let size = (2 + filename.len() + 1 + total_content_size) as u16; // Total size (header + nom + term + contenu)

    unsafe {
        // = Écrire le header et le nom du fichier (avec null terminator) =
        let filename_len = filename.len();
        
        let dest_header_ptr = write_pos as *mut u16;
        let dest_name_slice = slice::from_raw_parts_mut((dest_header_ptr as *mut u8).add(2), filename_len);
        let dist_nt_ptr = dest_name_slice.as_mut_ptr().add(filename_len);
        
        // Écrire le header
        ptr::write_unaligned(dest_header_ptr, size);

        // écrire le nom du fichier (sans null terminator)
        dest_name_slice.copy_from_slice(filename.as_bytes());

        // Écrire le null terminator
        *dist_nt_ptr = 0;

        // = Écrire les segments de contenu =
        let mut content_write_ptr = dist_nt_ptr.add(1); // ptr du contenue (juste après le null terminator)

        for segment in segments {
            let segment_len = segment.len();
            let dest_content_slice = slice::from_raw_parts_mut(content_write_ptr, segment_len);

            // Écrire le segment de contenu
            dest_content_slice.copy_from_slice(segment);

            // Avancer le pointeur d'écriture du contenu
            content_write_ptr = content_write_ptr.add(segment_len); 
        }

        // = Écrire 2 bytes vide a la fin du fichier par sécurité =

        // Même si le fichie va jusqu'a la tout fin du stockage, la limite de zone définie ici laisse la marche de 2 bytes avent le magic footer
        ptr::write_unaligned(content_write_ptr as *mut u16, 0);
    }

    Ok(())
}

#[cfg(not(target_os = "none"))]
pub fn file_write_segments(_filename: &str, _segments: &[&[u8]]) -> Result<(), GlobalError> {
    Ok(())
}


/// Écrit un nouveau fichier dans le stockage
/// 
/// ## Warning
/// Cette fonction wrtie des bytes bruts, si vous voulais stocker du texte utilisez plutôt `file_write_string()`
/// qui gère le format de fichier texte de Epsilon (notament utiliser pour les fichier python)
/// 
/// Format: \[2 bytes taille\] \[nom\0\] \[contenu\]
#[cfg(target_os = "none")]
pub fn file_write_raw(filename: &str, content: &[u8]) -> Result<(), GlobalError> {
    file_write_segments(filename, &[content])?;
    Ok(())
}

// Dummy version
#[cfg(not(target_os = "none"))]
pub fn file_write_raw(_filename: &str, _content: &[u8]) -> Result<(), GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Lit un fichier et retourne un pointeur vers son contenu
#[cfg(target_os = "none")]
pub unsafe fn file_read_raw(filename: &str) -> Result<&[u8], GlobalError> {

    // Vérifier que le storage est valide
    is_valid_storage()?;
    
    // Localiser le fichier via une recherche directe
    let file_view = _find_file(filename)?;

    // Retourner une slice pointant vers le contenu du fichier
    return Ok(slice::from_raw_parts(file_view.content, file_view.content_size));
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_read_raw(_filename: &str) -> Result<&[u8], GlobalError> {
    Err(SoftwareError::SimulatorNotSupported.into())
}


/// Vérifie si un fichier existe dans le stockage
#[cfg(target_os = "none")]
pub fn file_exists(filename: &str) -> Result<bool, GlobalError> {
    match unsafe { file_read_raw(filename) } {
        Ok(_) => Ok(true),
        Err(GlobalError::Storage(StorageError::FileNotFound)) => Ok(false),
        Err(e) => Err(e),
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub fn file_exists(_filename: &str) -> Result<bool, GlobalError> {
    Ok(false)
}


/// Efface un fichier du stockage (**Irréversible!**)
#[cfg(target_os = "none")]
pub unsafe fn file_erase(filename: &str) -> Result<(), GlobalError> {

    is_valid_storage()?;

    let file_to_erase = _find_file(filename)?;

    let free_space_before_deletion = next_free();
    let next_file_pos = file_to_erase.ptr.add(file_to_erase.size);

    // Déplacer tout depuis la fin du fichier supprimé jusqu'à la fin de l'espace utilisé vers le début du fichier supprimé pour combler le trou
    // Normalement ptr::copy devrait automatiquement détecter le chvauchement et faire la copie de manière sûre.
    ptr::copy(
        next_file_pos,
        file_to_erase.ptr as *mut u8,
        free_space_before_deletion.offset_from(next_file_pos) as usize
    );

    // Nettoyer l'espace libéré à la fin du stockage (obligatoire)
    ptr::write_bytes(
        free_space_before_deletion.sub(file_to_erase.size) as *mut u8,
        0,
        file_to_erase.size
    );

    Ok(())
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub unsafe fn file_erase(_filename: &str) -> Result<(), GlobalError> {
    Ok(())
}


/// Écrit un fichier texte dans le stockage en utilisant le format de fichier texte d'Epsilon
/// 
/// Ce format est notament utilisé pour les fichiers python d'Epsilon. **A utiliser pour type de fichier texte pour une universification !**
#[cfg(target_os = "none")]
pub fn file_write_string(filename: &str, content: &str) -> Result<(), GlobalError> {
    
    let header_slice = [0u8];
    let content_slice = content.as_bytes();
    let footer_slice = [0u8];

    // 1 bytes de metadata + contenu en utf-8 + 1 bytes de null terminator car c'est comme ça
    let segments: [&[u8]; 3] = [&header_slice, content_slice, &footer_slice];

    // Écrire le fichier par segmentation, aucune copie intermédiaire
    file_write_segments(filename, &segments)?;

    Ok(())
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub fn file_write_string(_filename: &str, _content: &str) -> Result<(), GlobalError> {
    Ok(())
}


/// Lit un fichier texte en utilisant le format de fichier texte d'Epsilon et retourne une slice de string pointant vers son contenu.
/// 
/// ## Warning
/// **Retourne une string statique pointant directement vers le contenu du fichier dans le stockage !**
/// 
/// Si vous supprimez le fichier ou le modifiez/déplacez, assurez-vous de ne plus utiliser la string obtenue via cette fonction,
/// car elle ne pointera plus vers le bon contenu. **Réutilisez la fonction pour obtenir une nouvelle string après toute modification du fichier.**
#[cfg(target_os = "none")] 
pub fn file_read_string(filename: &str) -> Result<&'static str, GlobalError> {
    unsafe {
        // Obtenir le contenu brut du fichier
        let raw_content = file_read_raw(filename)?;

        if raw_content.len() < 2 { // Il faut au moins 2 bytes (metadata et nt)
            return Err(StorageError::FileContentTooShort { expected: 2, actual: raw_content.len() }.into());
        }

        // Ignorer le premier bytes de metadata et le dernier byte de nt pour uniquement le texte
        let trimmed_content = &raw_content[1..raw_content.len() - 1];
        
        // Manipulation pour convertire en static str. 
        let static_slice = slice::from_raw_parts(trimmed_content.as_ptr(), trimmed_content.len());
        let static_str = str::from_utf8_unchecked(static_slice);

        Ok(static_str) // Cast en 'static car le contenu du fichier est supposé rester valide tant que le fichier existe (et on ne gère pas la suppression dans cette fonction)
    }
}

/// Dummy version
#[cfg(not(target_os = "none"))]
pub fn file_read_string(_filename: &str) -> Result<&'static str, GlobalError> {
    Ok("Dummy content")
}

