
# EIF1

## Spécifications de la version 1

- **Magic Number** : `0x31464945` — `EIF1` (ASCII) sur 4 premiers octets

- **Transparence** : Non supportée
- **Format initial** : Première version du format EIF
- **Structure du fichier** : En-tête suivi des données d'image en RGB 565


## Structure du fichier

Structure de fichier Format EIF1 (Eadkp. Image. Format. 1.) :

| **Nom**           | **Adresse**                        | **Taille**          | **Description**                                                                                                  |
| ----------------- | ---------------------------------- | ------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **Magic Number**  | `0x000` — `0x003`                  | 4 Octets            | Nombre d'identification du format du fichier (`0x31464945` en hex, correspond à "EIF1" en ASCII)                 |
| **Width**         | `0x004` — `0x005`                  | 2 Octets            | Largeur de l'image en pixels, format `u16` (entier non signé sur 2 octets)                                       |
| **Height**        | `0x006` — `0x007`                  | 2 Octets            | Hauteur de l'image en pixels, format `u16` (entier non signé sur 2 octets)                                       |
| **Pixels (data)** | `0x008` — `0x007 + (N_pixels × 2)` | N pixels × 2 Octets | Données des pixels, format `RGB565` (chaque pixel sur 2 octets). Chaque pixel est un `u16`. N_pixels × 2 Octets. |
