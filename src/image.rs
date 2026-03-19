
use super::*;
use alloc::{boxed::Box, slice};

/// Source de données pour les pixels d'une image. Peut être en flash ou en RAM.
pub trait PixelsSource {
    fn buffer(&self) -> &[u8];
}

/// Source de pixels en flash. Lent mais pas de stockage en RAM nécessaire.
pub struct FlashSource(pub &'static [u8]);
impl PixelsSource for FlashSource {
    fn buffer(&self) -> &[u8] {
        self.0
    }
}

/// Source de pixels en RAM. Rapide mais consomme de la RAM pour stocker les données de l'image.
pub struct RamSource(pub Box<[u8]>);
impl PixelsSource for RamSource {
    fn buffer(&self)-> &[u8] { &self.0 }
}


pub trait ImageFormat {
    /// Récupérer la largeur de l'image en pixels.
    fn width(&self) -> u16;
    /// Récupérer la hauteur de l'image en pixels.
    fn height(&self) -> u16;
    /// Dessiner l'image a l'écran a une postion donnée
    fn draw(&self, dest: Point);
}

/// Format d'image "EIF1" (EADKP Image Format 1).
/// - Pas de transparence
/// - Pas de compression
/// - Couleurs au format RGB565 (16 bits par pixel)
/// 
/// | Nom           | Taille                     |  
/// | ------------- | -------------------------- |
/// | Magic Number  | 4 bytes                    |
/// | Width         | 2 bytes                    |
/// | Height        | 2 bytes                    |
/// | Pixels (data) | (width * height * 2) bytes |
pub struct Eif1<S: PixelsSource> {
    pub width: u16,
    pub height: u16,
    pub source: S,
}
impl<S: PixelsSource> ImageFormat for Eif1<S> {
    fn width(&self) -> u16 {
        self.width
    }

    fn height(&self) -> u16 {
        self.height
    }

    fn draw(&self, _dest: Point) {

        // Construire le rectangle de destination pour l'affichage
        let rect = Rect {
            x: _dest.x,
            y: _dest.y,
            width: self.width,
            height: self.height,
        };

        // Magic number (4 bytes) + width (2 bytes) + height (2 bytes) = 8 bytes de header
        let raw_bytes = &self.source.buffer()[(4+2+2)..];

        // Interpréter les pixels comme des Color
        let pixels: &[Color] = unsafe {
            slice::from_raw_parts(
                raw_bytes.as_ptr() as *const Color,
                raw_bytes.len() / 2
            )
        };

        // Envoyer le buffer a l'écran
        display::push_rect(rect, pixels);
    }
}