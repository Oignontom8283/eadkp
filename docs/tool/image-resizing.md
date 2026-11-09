# Redimensionner une image

L'écran de la NumWorks fait 320 × 240 pixels, et une image prend 2 octets par pixel une fois convertie en [EIF](../reference/formats/eif). Mieux vaut donc redimensionner les images avant de les mettre dans `assets/`, plutôt que de laisser un grand fichier occuper de la place pour rien.

[ImageMagick](https://imagemagick.org) le fait en une ligne de commande.

## Installation

```bash
sudo apt install imagemagick
```

Selon la version installée, la commande s'appelle `convert` (version 6, celle de la plupart des distributions) ou `magick` (version 7). Les exemples ci-dessous utilisent `convert` ; remplace-la par `magick` si besoin.

## Choisir la taille

`-resize 64x64` fait tenir l'image dans un carré de 64 × 64 en gardant ses proportions : une image 200 × 100 devient 64 × 32. Pour obtenir exactement 64 × 64 quitte à la déformer, ajoute `!` : `-resize '64x64!'`.

Pour l'icône de l'application (`assets/icon.png`), la taille doit être exactement 55 × 56 :

```bash
convert icone.png -resize '55x56!' assets/icon.png
```

## Choisir le filtre

Le filtre est la méthode de calcul des nouveaux pixels. Selon le type d'image, le résultat n'est pas le même.

| Image | Commande | Effet |
| ----- | -------- | ----- |
| Cas général (recommandé) | `convert in.png -resize 64x64 -filter Lanczos -define filter:blur=0.9 out.png` | net, peu de défauts, bon contraste |
| Rendu plus doux | `convert in.png -resize 64x64 -filter Mitchell out.png` | moins net |
| Photo | `convert in.png -resize 64x64 -filter Cubic out.png` | garde les détails sans les accentuer |
| Pixel art, icônes nettes | `convert in.png -resize 64x64 -filter Point out.png` | aucun lissage, les pixels restent carrés |
| Très net | `convert in.png -resize 64x64 -filter LanczosSharp out.png` | peut créer des halos autour des contours |
| Réduction d'un facteur entier | `convert in.png -sample 64x64 out.png` | garde un pixel sur deux (si l'image fait 128 × 128), sans aucun calcul |

Pour du pixel art, utilise `Point` ou `-sample`, sinon les bords nets deviennent flous.

## Vérifier le résultat

```bash
identify assets/pain.png
```

La commande affiche la taille de l'image en pixels, par exemple `64x64`.
