
unsafe extern "C" {
    fn eadkp_random() -> u32;
}

/// Génére un nombre u32 aléatoire en appelant l'ABi Epsilon sans optimisation
pub fn random_c() -> u32 {
    unsafe { eadkp_random()}
}


