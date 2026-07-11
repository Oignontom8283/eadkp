
/*!
Ce fichier contient la structure `Color` relatif a la manipulation des couleurs.
- Ce fichier est reexporté par le module `common` de la lib.
*/

/// Représente une couleur au format RGB565.
/// 
/// Le champ `rgb565` stocke la couleur sous forme d'un entier non signé de 16 bits,
/// avec la répartition des bits suivante :
/// - 5 bits pour le rouge
/// - 6 bits pour le vert
/// - 5 bits pour le bleu
///
/// Ce format est couramment utilisé dans les systèmes embarqués et les applications graphiques
/// pour une représentation efficace des couleurs.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub rgb565: u16,
}

impl Color {
    #[inline]
    /// Créer un objet Color depuis un RGB565 **(5-bit red, 6-bit green, and 5-bit blue)**.
    pub const fn from_components(r: u8, g: u8, b: u8) -> Self {
        Color {
            rgb565: ((r as u16) << 11) | ((g as u16) << 5) | (b as u16),
        }
    }
    /// Créer un objet Color depuis un RGB888 (8-bits par canal). **(RGB normal)**
    pub const fn from_888(r: u8, g: u8, b: u8) -> Self {
        Color::from_components(r >> 3, g >> 2, b >> 3)
    }

    /// Applique un niveau de lumière à la couleur, en ajustant les composantes RGB en fonction du niveau de lumière spécifié.
    pub fn apply_light(&self, light_level: u8) -> Self {
        let light_level = light_level as u16;
        let (r, g, b) = self.get_components();
        Color::from_components(
            ((r as u16 * light_level / 255).min(31)) as u8,
            ((g as u16 * light_level / 255).min(63)) as u8,
            ((b as u16 * light_level / 255).min(31)) as u8,
        )
    }

    /// Récupère les composantes RGB de la couleur en format RGB565.
    pub const fn get_components(&self) -> (u8, u8, u8) {
        let r = self.rgb565 >> 11;
        let g = (self.rgb565 & 0b0000011111100000) >> 5;
        let b = self.rgb565 & 0b0000000000011111;

        (r as u8, g as u8, b as u8)
    }

    /// Récupère les composantes RGB de la couleur en format RGB888.
    pub const fn get_888(&self) -> (u8, u8, u8) {
        let (r, g, b) = self.get_components();

        return (
            ((r << 3) | (r >> 2)) as u8,
            ((g << 2) | (g >> 4)) as u8,
            ((b << 3) | (b >> 2)) as u8,
        );
    }
}
