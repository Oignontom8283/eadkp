
use crate::{
    common::{Rect, Point, Color},
    constant, display,
    ImageError,
};
use alloc::{boxed::Box, slice, string::ToString};
use core::{ptr};


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


/// Trait pour les objets d'image
pub trait ImageFormat {
    /// Récupérer la largeur de l'image en pixels.
    fn width(&self) -> u16;
    /// Récupérer la hauteur de l'image en pixels.
    fn height(&self) -> u16;
    /// Dessiner l'image a l'écran a une postion donnée
    fn draw(&self, dest: Point);
}


/// Représentation d'une image.
/// - Abstraction de formats et sources de pixels.
pub enum Image {
    Eif1Flash(Eif1<FlashSource>),
    Eif1Ram(Eif1<RamSource>),
    // Futurs formats
}
impl ImageFormat for Image {

    // Rajouter manuellement les méthodes pour chaque format et source.
    // (Pas trouver de meilleur solution)

    fn width(&self) -> u16 {
        match self {
            Image::Eif1Flash(eif) => eif.width(),
            Image::Eif1Ram(eif) => eif.width(),
        }
    }

    fn height(&self) -> u16 {
        match self {
            Image::Eif1Flash(eif) => eif.height(),
            Image::Eif1Ram(eif) => eif.height(),
        }
    }

    fn draw(&self, dest: Point) {
        match self {
            Image::Eif1Flash(eif) => eif.draw(dest),
            Image::Eif1Ram(eif) => eif.draw(dest),
        }
    }
}


/// Chargeur d'images intelligent.
/// - Gère les formats d'images et les sources de pixels.
/// - Permet de créer une instance d'image qui possède ces données (pixels) en RAM ou non (en flash).
pub struct ImageLoader;

impl ImageLoader {

    /// Créer une image à partir de données en flash. Sans charger les pixels en mémoire RAM.
    /// - Ne possède pas les données de l'image (En flash)
    /// - Vitesse de lecture lente
    /// - Pas de consommation de RAM
    /// - Parfait pour les grandes images a accès peu fréquents (ex: décors/backgrounds)
    /// - Ne convient pas pour les images a accès fréquents/très fréquents (ex: sprites/animations)
    pub fn from_flash(data: &'static [u8]) -> Result<Image, ImageError> {

        // Si trop court, probablement corrompu ou pas valide
        if data.len() < 8 {
            return Err(ImageError::CorruptedData { details: "Data too short to contain valid header".to_string() });
        }

        unsafe {
            // Lire le magic number (4 bytes) de manière non alignée
            let magic = ptr::read_unaligned(data.as_ptr() as *const u32);

            // Identifier le format d'image à partir du magic number et créer l'instance correspondante
            match magic {
                EIF1_MAGIC_NUMBER => Ok( Image::Eif1Flash(Eif1::new(FlashSource(data))) ),
                // Futurs formats

                _ => Err(ImageError::UnsupportedFormat { magic_number: magic }),
            }
        }
    }

    /// Créer une image et copier les données en RAM. Nécessite de charger les pixels en mémoire RAM.
    /// - Possède les données de l'image (En RAM - Lui appartient)
    /// - Vitesse de lecture rapide
    /// - Consommation de RAM pour stocker les données de l'image
    /// - Parfait pour les images a accès fréquents/très fréquents (ex: sprites/animations)
    /// - Ne convient pas grandes images a accés peu fréquents (ex: décors/backgrounds)
    pub fn from_ram(data: &[u8]) -> Result<Image, ImageError> {

        if data.len() < 8 {
            return Err(ImageError::CorruptedData { details: "Data too short to contain valid header".to_string() });
        }

        // Copier les données dans un Box
        let buffer: Box<[u8]> = Box::from(data);

        unsafe {
            let magic = ptr::read_unaligned(buffer.as_ptr() as *const u32);

            match magic {
                EIF1_MAGIC_NUMBER => Ok( Image::Eif1Ram(Eif1::new(RamSource(buffer))) ),
                // Futurs formats

                _ => Err(ImageError::UnsupportedFormat { magic_number: magic }),
            }
        }
    }

    /// Créer une image et copier les données en RAM. Nécessite de charger les pixels en mémoire RAM.
    /// - Possède les données de l'image (En RAM - Lui appartient)
    /// - Vitesse de lecture rapide
    /// - Consommation de RAM pour stocker les données de l'image
    /// - Parfait pour les images a accès fréquents/très fréquents (ex: sprites/animations)
    /// - Ne convient pas grandes images a accés peu fréquents (ex: décors/backgrounds)
    /// 
    /// Lien symbolique pour `from_ram`
    pub fn to_ram(data: &'static [u8]) -> Result<Image, ImageError> {
        Self::from_ram(data)
    }
}




// ======== Immplementation des formats d'images ========


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
impl<S: PixelsSource> Eif1<S> {
    /// Créer une instance d'Eif1 a partir d'une source de pixels. Ne **vérifie pas la validité des données !**
    pub unsafe fn new(source: S)-> Self {
        let buf = source.buffer();

        // Créer une instance de Eif1 en donnant les metadonnées
        Self {
            width: u16::from_le_bytes([buf[4], buf[5]]),
            height: u16::from_le_bytes([buf[6], buf[7]]),
            source,
        }
    }
}