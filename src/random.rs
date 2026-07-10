
/*!
Sous-module fournissant divers fonctions de génération de nombres pseudo-aléatoires.
*/

use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};


unsafe extern "C" {
    fn eadk_random() -> u32;
}

/// Génère un nombre u32 aléatoire en appelant l'ABI Epsilon sans optimisation
pub fn random_c() -> u32 {
    unsafe { eadk_random() }
}


/// État du générateur protégé via Atomics (Zero-cost avec Ordering::Relaxed)
static FAST_RNG_STATE: AtomicU32 = AtomicU32::new(0x12345678);
static SEED_COUNTER: AtomicU8 = AtomicU8::new(0);


/// Génére un nombre u32 pseudo-aléatoire ultra-rapidement
/// - Rapide
/// - Utilise l'algorithme Xorshift32 pour une génération rapide de nombres pseudo-aléatoires
/// - Reseed de l'entropie matérielle tout les 256 appels
/// - Non cryptographiquement sécurisé
#[inline(always)]
pub fn random() -> u32 {
    
    let mut counter = SEED_COUNTER.load(Ordering::Relaxed); // Charger le compteur actuel (0-255)
    let mut state = FAST_RNG_STATE.load(Ordering::Relaxed); // Charger l'état actuel du RNG rapide

    // Reseed de l'entropie matérielle tous les 256 appels pour éviter la stagnation statistique
    if counter == 0 {
        let hardware_seed = random_c();
        state ^= hardware_seed;
        
        // Ne doit jamais être égal à 0 pour éviter de rester bloqué sur 0
        if state == 0 {
            state = 0x12345678;
        }
    }

    // Incrémentation et sauvegarde du compteur
    counter = counter.wrapping_add(1);
    SEED_COUNTER.store(counter, Ordering::Relaxed);

    // Xorshift32 pur (3 cycles)
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 5;
    
    FAST_RNG_STATE.store(state, Ordering::Relaxed);
    
    state
}

/// Génére un nombre f32 pseudo-aléatoire dans l'intervale 0.0 et 1.0 (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_f32() -> f32 {
    let rand_bits = random();
    let float_bits = 0x3F80_0000_u32 | (rand_bits >> 9);
    f32::from_bits(float_bits) - 1.0
}

/// Génére un nombre f32 pseudo-aléatoire dans un intervale donné : `min` et `max` (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_f32_range(min: f32, max: f32) -> f32 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    min + (max - min) * random_f32()
}

/// Génére un nombre u32 pseudo-aléatoire dans une plage donnée : `min` et `max` (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
/// - Utilise l'algorithme de Lemire (version **fast path**)
/// - **Très rapide**
/// - Peut introduire un léger biais statistique pour les plages qui ne sont pas des puissances de 2. Une chance sur `2^32/(max-min+1)`
/// - *Probabilité de biais : 0.0000000002328% pour une plage de 1000*
#[inline(always)]
pub fn randint(min: u32, max: u32) -> u32 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    
    if min == 0 && max == u32::MAX {
        return random();
    }
    
    let range = max - min;
    
    // Fast path de Lemire : 1 multiplication matérielle + 1 décalage
    let m = (random() as u64) * (range as u64);
    
    min + (m >> 32) as u32
}

/// Génére un nombre u32 pseudo-aléatoire dans une plage donnée : `min` et `max` (exclus)
/// - Voir `random()` pour les détails de l'algorithme de génération
/// - Utilise l'algorithme de Lemire (version **complète**)
/// - **Plus lent que `randint()`**. Déconseillé dans la plupart des cas, sauf si une uniformité parfaite est requise (ex: simulations scientifiques, jeux de hasard, etc.)
/// - Garantit une uniformité parfaite sans biais statistique, même pour les plages qui ne sont pas des puissances de 2.
#[inline(always)]
pub fn randint_unbiased(min: u32, max: u32) -> u32 {
    debug_assert!(min <= max, "min must be less than or equal to max");
    
    if min == 0 && max == u32::MAX {
        return random();
    }
    
    let range = max - min;
    
    // Algorithme complet de Lemire
    let mut m = (random() as u64) * (range as u64);
    let mut l = m as u32; // Partie fractionnaire (les 32 bits de poids faible)
    
    // Rejection sampling pour garantir une uniformité mathématiquement parfaite
    // Probabilité : range / 2^32
    if l < range {
        let t = range.wrapping_neg() % range; // Modulo matériel ARM (calcul du seuil de rejet)
        while l < t {
            m = (random() as u64) * (range as u64);
            l = m as u32;
        }
    }
    
    // Retourne les 32 bits de poids fort (offset sans biais)
    min + (m >> 32) as u32
}

/// Génére un booléen peudo-aléatoire
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_bool() -> bool {
    (random() & 1) != 0 
}

/// Génére un booléen pseudo-aléatoire avec une probabilité donnée d'être `true` (entre 0.0 et 1.0)
/// - Voir `random()` pour les détails de l'algorithme de génération
#[inline(always)]
pub fn random_bool_prob(probability: f32) -> bool {
    debug_assert!((0.0..=1.0).contains(&probability), "probability must be between 0.0 and 1.0");

    random_f32() < probability
}
