# EIF1 : la version 1 du format EIF

EIF1 est la seule version du [format EIF](./eif) pour l'instant. Un fichier est composé d'un en-tête de 8 octets, suivi des pixels.

## Contenu du fichier

| Position (octets) | Taille | Nom | Contenu |
| ----------------- | ------ | --- | ------- |
| 0 à 3 | 4 | Identifiant | les quatre caractères `EIF1` (octets `0x45 0x49 0x46 0x31`) |
| 4 à 5 | 2 | Largeur | largeur en pixels, entier de 16 bits |
| 6 à 7 | 2 | Hauteur | hauteur en pixels, entier de 16 bits |
| 8 à la fin | largeur × hauteur × 2 | Pixels | un pixel par groupe de 2 octets, en [RGB 565](./rgb565) |

La taille totale d'un fichier est donc `8 + largeur × hauteur × 2` octets.

## Ordre des octets

Tous les nombres de plus d'un octet sont écrits avec l'octet de poids faible en premier (*petit-boutiste*, ou *little-endian*). C'est l'ordre natif du processeur de la calculatrice, ce qui permet d'envoyer les pixels à l'écran sans les réorganiser.

Exemple : une largeur de 300 vaut `0x012C` ; elle est écrite `0x2C 0x01`.

## Ordre des pixels

Les pixels sont rangés ligne par ligne, de haut en bas, et dans chaque ligne de gauche à droite. Le premier pixel est celui du coin en haut à gauche.

## Exemple complet

Une image de 2 pixels de large et 1 de haut : un pixel rouge, puis un pixel bleu.

- rouge pur en RGB 565 : `0xF800`, écrit `0x00 0xF8`
- bleu pur en RGB 565 : `0x001F`, écrit `0x1F 0x00`

```
45 49 46 31   identifiant « EIF1 »
02 00         largeur : 2
01 00         hauteur : 1
00 F8         pixel 1 : rouge
1F 00         pixel 2 : bleu
```

Le fichier fait 12 octets : 8 d'en-tête et 2 × 1 × 2 = 4 de pixels.

## Transparence

Le format ne gère pas la transparence : chaque pixel est une couleur pleine. Lors de la conversion d'un PNG, les pixels totalement transparents (opacité à 0) deviennent blancs. Une opacité partielle est ignorée : la couleur du pixel est gardée telle quelle.

## Ce que la lecture vérifie, et ce qu'elle ne vérifie pas

`ImageLoader` contrôle que le fichier fait au moins 8 octets et que l'identifiant est connu. Il ne contrôle pas que le nombre de pixels correspond à la largeur et à la hauteur annoncées : un fichier tronqué est accepté. Les fichiers produits par la compilation sont toujours complets, donc ce n'est un risque que pour un fichier écrit ou modifié à la main.

## Où est le code

- Écriture : `convert_image` dans [builder.rs](https://github.com/Oignontom8283/eadkp/blob/dev/src/builder.rs)
- Lecture et affichage : la structure `Eif1` dans [image.rs](https://github.com/Oignontom8283/eadkp/blob/dev/src/image.rs)
