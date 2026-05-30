
# Export & Build

Build votre application et exportez son deux choses a différencier :
- Build : Vous ne faites que compiler votre application, vers le simulateur ou vers la calculatrice, mais vous ne générez pas de fichier d'exportation.
- Export : Vous builder et générez un fichier d'exportation au format `.nwa` pour pouvoir l'installer sur votre calculatrice ou le partager avec d'autres personnes.

## Commandes

### Build

#### Calculatrice
```bash
just build
```
#### Simulateur
```bash
just build_simulator
```

### Export
```bash
just export
```
Génère le fichier `.nwa` dans le dossier `build/` de votre projet.

> [!INFO]
> Le dossier par defaut pourrais venir a être changé dans le futur, ex: `dist/`, `out/` ou autre.

### Check
```bash
just check
```
Vérifie que votre projet ce compile sans erreur. 

> [!NOTE]
> Cette commande pourrais venir a être supprimé ou changé dans le futur, déconseillé de l'utiliser dans vos scripts personnalisés, utilisez `just build` a la place qui fait la même chose.

### Clean

```bash
just clean
```
Supprime les fichiers de build générés (`target/` et `build/`).

### Clear

```bash
just clear
```
Pareil que `clean`, mais en plus il supprime le simulateur.

### Target

```bash
just target
```
Télécharge la cible de compilation pour la calculatrice (normalement fait automatiquement).S

### Update scripts

```bash
./update.sh
```
Mes à jour les scripts locaux de votre projet : `docker.sh`, `update.sh`, `justfile`, etc ...

