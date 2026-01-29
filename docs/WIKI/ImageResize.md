
# Redimensionner des images avec ImageMagick

ImageMagick est un outil puissant en ligne de commande pour manipuler des images, y compris le redimensionnement.

Voici quelques exemples de commandes pour redimensionner des images en fonction du rendu souhaité :


## Installation

```
sudo apt install imagemagick
```


## Exemples

**Recommander (Net + peu d’artefacts + bon contraste) :**
```
convert input.png -resize 64x64 -filter Lanczos -define filter:blur=0.9 output.png
```
---


**Rendu plus doux (moins de netteté) :**
```
convert input.png -resize 64x64 -filter Mitchell output.png
```
---


**Rendu photo réaliste (Préserve les détails sans trop accentuer) :**
```
convert input.png -resize 64x64 -filter Cubic output.png
```
---


**Pixel art / icônes nettes (AUCUN lissage, pixel-perfect) :**
```
convert input.png -resize 64x64 -filter Point output.png
```
---


**Ultra net (Très sharp, parfois trop, attention aux halos) :**
```
convert input.png -resize 64x64 -filter LanczosSharp output.png
```
---


**Réduction parfaite (Mathématiquement, pour une image en 128x128) :**
```
convert input.png -sample 64x64 output.png
```