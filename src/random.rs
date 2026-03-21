
unsafe extern "C" {
    fn eadkp_random() -> u32;
}

/// Génére un nombre u32 aléatoire en appelant l'ABi Epsilon sans optimisation
pub fn random_c() -> u32 {
    unsafe { eadkp_random()}
}


/// état du générateur Xorshift32 (ne doit jamais être 0)
static mut FAST_RNG_STATE: u32 = 0x12345678;
/// Compteur pour le reseed matériel périodique
static mut SEED_COUNTER: u8 = 0;


/// Génére un nombre u32 pseudo-aléatoire ultra-rapidement
/// - Utilise l'algorithme Xorshift32 pour une génération rapide de nombres pseudo-aléatoires
/// - Reseed de l'entropie matérielle tout les 256 appels
/// - Non cryptographiquement sécurisé
/// - Rapide
#[inline(always)]
pub fn random() -> u32 {
    unsafe {
        // Reseed matériel tout les 256 appels
        if SEED_COUNTER == 0 {
            
            let hardware_seed = random_c();

            // Mélanger le seed matériel dans l'état du RNG pour améliorer l'entropie
            FAST_RNG_STATE ^= hardware_seed;
            if FAST_RNG_STATE == 0 {
                FAST_RNG_STATE = 0x12345678; // Assurer que l'état ne soit jamais 0
            }
        }

        // Incrémenter le compteur de seed (revient à 0 après 256 appels, déclenchant un nouveau reseed)
        SEED_COUNTER = SEED_COUNTER.wrapping_add(1);

        // Xorshift32 algorithm
        let mut x = FAST_RNG_STATE;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        FAST_RNG_STATE = x;
        x
    }
}