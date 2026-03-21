
/// Représente le niveau de charge de la batterie, avec des variantes allant de "Empty" (vide) à "Full" (plein).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BatteryCharge {
    Empty = 0,      // 0%   - Batterie vide. Pas super utile pour un programme ça di donc, si y'a plus de batterie.
    Critical = 1,   // ~20% - Batterie critique
    Low = 2,        // ~40% - Batterie faible
    Medium = 3,     // ~60% - Batterie moyenne
    High = 4,       // ~80% - Batterie élevée
    Full = 5,       // 100% - Batterie pleine
}

impl From<u8> for BatteryCharge {
    fn from(val: u8) -> Self {
        match val {
            0 => BatteryCharge::Empty,
            1 => BatteryCharge::Critical,
            2 => BatteryCharge::Low,
            3 => BatteryCharge::Medium,
            4 => BatteryCharge::High,
            _ => BatteryCharge::Full,
        }
    }
}

impl BatteryCharge {
    pub fn to_str(&self) -> &'static str {
        match self {
            BatteryCharge::Empty => "Empty",
            BatteryCharge::Critical => "Critical",
            BatteryCharge::Low => "Low",
            BatteryCharge::Medium => "Medium",
            BatteryCharge::High => "High",
            BatteryCharge::Full => "Full",
        }
    }
}

#[cfg(target_os = "none")]
pub fn level() -> BatteryCharge {
    let result: u8;
    unsafe {
        core::arch::asm!( // Obtenir le niveau de charge de la batterie via un appel système (svc)
            "svc {svc_num}",
            "mov {out}, r0",
            svc_num = const 4,
            out = out(reg) result,
            options(nostack, nomem)
        );
    }
    BatteryCharge::from(result)
}

#[cfg(not(target_os = "none"))]
pub fn level() -> BatteryCharge {
    BatteryCharge::High // Valeur Dummy pour les cibles non-embarquées
}

#[cfg(target_os = "none")]
pub fn voltage() -> f32 {
    let result: f32;
    unsafe {
        core::arch::asm!(
            "svc {svc_num}",
            "vmov {out}, s0",
            svc_num = const 5,
            out = out(reg) result,
            options(nostack, nomem)
        );
    }
    result
}

#[cfg(not(target_os = "none"))]
pub fn voltage() -> f32 {
    4.2 // Valeur Dummy pour les cibles non-embarquées
}

#[cfg(target_os = "none")]
pub fn is_charging() -> bool {
    let result: u8;
    unsafe {
        core::arch::asm!(
            "svc {svc_num}",
            "mov {out}, r0",
            svc_num = const 3,
            out = out(reg) result,
            options(nostack, nomem)
        );
    }
    result != 0
}

#[cfg(not(target_os = "none"))]
pub fn is_charging() -> bool {
    false // Valeur Dummy pour les cibles non-embarquées
}