
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

/// Génére un nombre f32 pseudo-aléatoire dans l'intervale 0.0 et 1.0 (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_f32() -> f32 {
    let rand_bites = random();

    // Construire un float dans l'intervale [1.0, 2.0)
    // ox3f80_0000 est l'exposant 127. On y ajoute 23 bits aléatoire de mentisse
    let float_bits = 0x3f80_0000_u32 | (rand_bites >> 9);

    // Soustraire 1.0 donne un résultat dans l'intervale [0.0, 1.0)
    f32::from_bits(float_bits) - 1.0
}

/// Génére un nombre f32 pseudo-aléatoire dans l'intervale min et max (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
    debug_assert!(min < max, "min must be less than max");
    min + (max - min) * random_f32()
}

/// Génére un nombre u32 pseudo-aléatoire dans une plage donnée : `min` et `max` (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
/// - Utilise l'algorithme de Lemire (version **fast path**) pour un échantillonnage rapide et sans biais statistique significatif
#[inline(always)]
pub fn randint(min: u32, max: u32) -> u32 {
    debug_assert!(min <= max, "min doit être inférieur ou égal à max");
    
    // En cas de plage complètes (0 - u32::MAX) on returne directement un u32 aléatoire
    // évite l'overflow du `+ 1` à la ligne suivante.
    if min == 0 && max == u32::MAX {
        return random();
    }
    
    // max est inclus donc le nombre de valeurs possibles est (max - min) + 1.
    // Garanti sans overflow grace à la condition du dessus
    let range = max - min + 1;
    
    // Fast path de Lemire*
    let m = (random() as u64) * (range as u64);
    let offset = (m >> 32) as u32;
    
    min + offset
}