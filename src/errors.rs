use alloc::string::String;

#[derive(Debug)]
pub enum StorageError {
    /// Fichier non trouvé dans le stockage
    FileNotFound,
    /// Pas assez d'espace disponible pour l'écriture
    InsufficientSpace,
    /// Nom de fichier invalide ou trop long (max 256 bytes)
    StorageInvalidName {length: usize, string: String},
    /// Taille du fichier dépasse la limite donnée ou maximale (u16::MAX)
    FileTooLarge { max_size: usize, actual_size: usize },
    /// Contenu du fichier plus court que la taille minimale attendue
    FileContentTooShort { expected: usize, actual: usize },
    /// Fichier vide, pas de données à stocker
    FileContentEmpty,
    /// Stockage plein, position libre null
    StorageFull,
    /// Dépassement de la taille du stockage
    StorageOverflow { available: usize, needed: usize },
    /// Format de fichier invalide ou corrompu
    InvalidFileFormat { ptr: *const u8, details: String },
    /// Le null terminator n'a pas été trouvé
    NullTerminatorNotFound { start: *const u8 },
}

#[derive(Debug)]
pub enum SoftwareError {
    /// Simulateur non supporté
    SimulatorNotSupported,
    /// SlotInfo invalide
    InvalidSlotInfo,
    /// UserlandHeader invalide
    InvalidUserlandHeader,
    /// Stockage invalide ou corrompu (magic number incorrect)
    InvalidStorage,
    /// Magic number incorrect dans une structure (ex: UserlandHeader, SlotInfo)
    InvalidMagicNumber { expected: u32, found: u32 },
    /// Zones de mémoire ce chevauchantes
    OverlappingRegions { src_start: *const u8, src_end: *const u8, dest_start: *const u8, dest_end: *const u8 },
    /// Overflow lors du calcul de pointeur
    PointerOverflow,
    /// Paramètre invalide fourni à une fonction
    InvalidParameter { param_name: String, details: String },
    /// Aucun \0 trouvé dans le buffer
    NoNullTerminator,
    // Pointeur null rencontré là où un pointeur valide était attendu
    NullPointer,
    /// Plage de pointeurs invalide (start > end)
    InvalidPointerRange { start: *const u8, end: *const u8 },
}

#[derive(Debug)]
pub enum ImageError {
    /// Format de fichier d'image non supporté
    UnsupportedFormat { magic_number: u32 },
    /// Données d'image corrompues ou incomplètes
    CorruptedData { details: String },
}

/// Erreur globale englobant tout les types d"erreurs
#[derive(Debug)]
pub enum GlobalError {
    Storage(StorageError),
    Software(SoftwareError),
    Image(ImageError),
}

impl From<StorageError> for GlobalError { // Convertire un StorageError en GlobalError automatiquement
    fn from(err: StorageError) -> Self {
        GlobalError::Storage(err)
    }
}

impl From<SoftwareError> for GlobalError { // De même pour SoftwareError
    fn from(err: SoftwareError) -> Self {
        GlobalError::Software(err)
    }
}

impl From<ImageError> for GlobalError { // De même pour ImageError
    fn from(err: ImageError) -> Self {
        GlobalError::Image(err)
    }
}