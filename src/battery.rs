
use crate::{ constant, svc_r0, svc_s0 };


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


/// Obtenir le niveau de charge de la batterie.
/// - Renvoie un enum `BatteryCharge` représentant le niveau de charge.
#[cfg(target_os = "none")]
pub fn level() -> BatteryCharge {

    // Obtenir le niveau de charge de la batterie via le SVC
    let result = svc_r0!(constant::SVC_BATTERY_LEVEL, u8);

    // Convertir le résultat en enum BatteryCharge
    BatteryCharge::from(result)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn level() -> BatteryCharge {
    BatteryCharge::High // Valeur Dummy pour les cibles non-embarquées
}


/// Obtenir la tension actuelle de la batterie en volts.
#[cfg(target_os = "none")]
pub fn voltage() -> f32 {
    // Obtenir la tension de la batterie via le SVC
    svc_s0!(constant::SVC_BATTERY_VOLTAGE)
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn voltage() -> f32 {
    4.2 // Valeur Dummy pour les cibles non-embarquées
}


/// Savoir si la batterie est en cours de charge.
#[cfg(target_os = "none")]
pub fn is_charging() -> bool {
    // Obtenir l'état de charge de la batterie via le SVC
    svc_r0!(constant::SVC_BATTERY_IS_CHARGING, u8) != 0
}

#[cfg(not(target_os = "none"))] // Version dummy
pub fn is_charging() -> bool {
    false
}
