# Le format d'image EIF

EIF (*Eadkp Image Format*) est le format d'image d'eadkp. Un fichier `.eif` contient les pixels d'une image tels que l'écran les attend, sans compression, ce qui permet de les envoyer à l'écran sans les décoder.

## En résumé

| Caractéristique | Valeur |
| --------------- | ------ |
| Extension | `.eif` |
| Couleurs | [RGB 565](./rgb565), 2 octets par pixel |
| Compression | aucune |
| Transparence | non |
| Taille maximale | 65 535 × 65 535 pixels |
| Versions | une seule : [EIF1](./eif-v1) |

## D'où viennent les fichiers `.eif`

On ne les écrit pas à la main. À la compilation, eadkp ([builder.rs](https://github.com/Oignontom8283/eadkp/blob/dev/src/builder.rs)) convertit chaque `.png` du dossier `assets/` de ton projet :

- `assets/pain.png` devient `pain.png.eif`, rangé dans le dossier de sortie de Cargo ;
- seuls les `.png` placés directement dans `assets/` sont convertis (pas dans un sous-dossier) ;
- `assets/icon.png` est à part : c'est l'icône de l'application, qui doit faire exactement 55 × 56 pixels et suit un autre format (`.nwi`), pas EIF.

## L'utiliser dans une application

La macro `include_image!` inclut le fichier converti dans le programme. `ImageLoader` le lit et dit s'il est valide.

```rust
use eadkp::{include_image, ImageFormat, ImageLoader, Point};

let image = ImageLoader::from_flash(include_image!("pain.png"))
    .expect("image EIF invalide");

image.draw(Point { x: 10, y: 10 });
```

Deux façons de charger l'image, selon la manière dont on s'en sert :

| Fonction | Où sont les pixels | Lecture | Convient à |
| -------- | ------------------ | ------- | ---------- |
| `from_flash` | restent dans la mémoire flash | plus lente | grands décors, peu redessinés |
| `from_ram` | copiés en RAM | rapide | sprites, animations |

La copie en RAM coûte la taille de l'image : une image plein écran (320 × 240) prend 153 608 octets, soit environ 150 Kio, sur une RAM d'environ 256 Kio. À réserver aux images petites ou très utilisées.

## Comment le format est reconnu

Les quatre premiers octets d'un fichier EIF forment l'identifiant de sa version (`EIF1` pour la version 1). `ImageLoader` les lit pour savoir comment interpréter la suite. Si l'identifiant est inconnu, il renvoie `ImageError::UnsupportedFormat` ; si le fichier fait moins de 8 octets, il renvoie `ImageError::CorruptedData`.

Le code est dans [image.rs](https://github.com/Oignontom8283/eadkp/blob/dev/src/image.rs).

## Voir aussi

- [EIF1 : la version 1, octet par octet](./eif-v1)
- [Le format de couleur RGB 565](./rgb565)
- [Redimensionner une image avant de l'ajouter](../../tool/image-resizing)
