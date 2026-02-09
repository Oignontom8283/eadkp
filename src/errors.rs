#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// Fichier non trouvé dans le stockage
    FileNotFound,
    /// Pas assez d'espace disponible pour l'écriture
    InsufficientSpace,
    /// Nom de fichier invalide ou trop long (max 256 bytes)
    StorageInvalidName,
    /// Magic number invalide à l'adresse de stockage
    InvalidMagicNumber { expected: u32, found: u32 },
    /// Stockage plein, position libre null
    StorageFull,
    /// Dépassement de la taille du stockage
    StorageOverflow { available: usize, needed: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareError {
    /// Simulateur non supporté
    SimulatorNotSupported,
    /// SlotInfo invalide
    InvalidSlotInfo,
    /// UserlandHeader invalide
    InvalidUserlandHeader,
    /// Stockage invalide ou corrompu (magic number incorrect)
    InvalidStorage,
    /// Zones de mémoire ce chevauchantes
    OverlappingRegions { src_start: *const u8, src_end: *const u8, dest_start: *const u8, dest_end: *const u8 },
    /// Overflow lors du calcul de pointeur
    PointerOverflow,
}