
# Format de fichier EIF (Eadkp Image Format)

Le format de fichier `.eif` est un format d'image binaire non compressé utilisé pour l'integration des images dans les builds des application Numworks utilisant la librairie eadkp. 

## Spécifications

- **Extension de fichier** : `.eif`
- **Type MIME** : `application/x-eif`
- **Couleurs** : RGB 565 (16 bits par pixel)
- **Compression** : Aucune (format non compressé)
- **Dimensions** : Variable, définie dans l'en-tête du fichier
- **Transparence** : Non supportée

## Version du format

La version actuelle du format EIF est la version **1**.

- [Version 1 : Ne supporte pas la transparence.](./EIF1.md)