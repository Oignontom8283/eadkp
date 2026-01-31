
unsafe extern "C" {
    fn eadk_random() -> u32;
}

/// Cache de state pour générateur ultra-rapide
static mut FAST_RNG_STATE: u32 = 0x12345678;

/// Returns a random `u32` value - version ultra-optimisée
/// Utilise un Xorshift32 rapide avec reseeding périodique
#[inline(always)]
pub fn random_c() -> u32 {
    unsafe {
        // Reseed depuis le hardware tous les 256 appels
        if FAST_RNG_STATE & 0xFF == 0 {
            FAST_RNG_STATE = eadk_random();
            if FAST_RNG_STATE == 0 { FAST_RNG_STATE = 1; }
        }
        
        // Xorshift32 ultra-rapide (3 cycles ARM)
        FAST_RNG_STATE ^= FAST_RNG_STATE << 13;
        FAST_RNG_STATE ^= FAST_RNG_STATE >> 17;
        FAST_RNG_STATE ^= FAST_RNG_STATE << 5;
        FAST_RNG_STATE
    }
}

/// Force hardware random - bypasse le cache rapide
#[inline(always)]
pub fn random_hardware() -> u32 {
    unsafe { eadk_random() }
}

/// Generates a random u64 number between min and max (inclusive).
/// Ultra-optimized for ARM Cortex-M7: avoids expensive modulo, uses bit manipulation
#[inline(always)]
pub fn randint(min: u64, max: u64) -> u64 {
    debug_assert!(min <= max, "randint: min cannot be greater than max");
    
    if min == max {
        return min;
    }
    
    let range = max - min + 1;
    
    // Ultra-fast path pour puissances de 2 (bit masking)
    if range & (range.wrapping_sub(1)) == 0 {
        let mask = range - 1;
        if range <= 0x1_0000_0000 { // <= 2^32
            return min + ((unsafe { eadk_random() } as u64) & mask);
        } else {
            // Génération 64-bit optimisée sans division
            let rand_u64 = unsafe {
                (eadk_random() as u64) | ((eadk_random() as u64) << 32)
            };
            return min + (rand_u64 & mask);
        }
    }
    
    // Fast rejection sampling pour éviter le biais
    if range <= 0x1_0000_0000 {
        let threshold = (0x1_0000_0000_u64 / range) * range;
        loop {
            let rand_val = unsafe { eadk_random() } as u64;
            if rand_val < threshold {
                return min + (rand_val % range);
            }
        }
    }
    
    // 64-bit path optimisé
    let threshold = (u64::MAX / range) * range;
    loop {
        let rand_u64 = unsafe {
            (eadk_random() as u64) | ((eadk_random() as u64) << 32)
        };
        if rand_u64 < threshold {
            return min + (rand_u64 % range);
        }
    }
}

/// Generates a random f64 between 0.0 and 1.0 (exclusive of 1.0)
/// Ultra-optimized: uses bit manipulation to avoid expensive float conversion
#[inline(always)]
pub fn random_f64() -> f64 {
    // Manipulation directe des bits IEEE 754 pour éviter la conversion coûteuse
    let rand_bits = unsafe { eadk_random() };
    // Utilise les 32 bits dans la mantisse, exponent = 1023 (bias pour [1.0, 2.0))
    let float_bits = 0x3FF0_0000_0000_0000_u64 | ((rand_bits as u64) << 20);
    f64::from_bits(float_bits) - 1.0
}

/// Generates a random f64 between min and max
/// Ultra-optimized: uses FMA instruction and inlined random generation
#[inline(always)]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    debug_assert!(min < max, "random_f64_range: min must be less than max");
    
    // Génération inline pour éviter l'overhead d'appel
    let rand_bits = unsafe { eadk_random() };
    let float_bits = 0x3FF0_0000_0000_0000_u64 | ((rand_bits as u64) << 20);
    let rnd = f64::from_bits(float_bits) - 1.0;
    
    // FMA optimisé manuellement (évite la dépendance libm)
    min + (max - min) * rnd
}

/// Generates a random boolean with 50% probability
#[inline]
pub fn random_bool() -> bool {
    unsafe { (eadk_random() & 1) != 0 }
}

/// Generates a random boolean with given probability
/// Ultra-optimized: branchless execution with bit manipulation
#[inline(always)]
pub fn random_bool_with_probability(probability: f64) -> bool {
    debug_assert!((0.0..=1.0).contains(&probability), "probability must be between 0.0 and 1.0");
    
    // Conversion optimisée vers seuil 32-bit avec saturation
    let threshold = (probability * 4294967296.0) as u64; // 2^32 pour meilleure précision
    let threshold = threshold.min(0xFFFFFFFF) as u32; // Saturation au lieu de branches
    
    unsafe { eadk_random() <= threshold }
}