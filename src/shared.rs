
/**
Ce fichier contient les constantes et les types partagés entre la lib et le script de compilation.
- Ce fichier est reexporté par le module `constant` de la lib.
*/


/// Magic number for EIF1 format. Magic number in hex `0x31464945`
pub const EIF1_MAGIC_NUMBER: u32 = u32::from_le_bytes(*b"EIF1"); 