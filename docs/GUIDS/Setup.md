## Guide de mise en place de l'environnement de développement Dockerisé

## Pré-requis

- **Docker** et Docker Compose installés sur votre machine.
- **Git** installé pour cloner le dépôt.
- Un éditeur de code, prenant en charge Rust et l'intégration avec Docker (par exemple, VSCode avec les extensions Rust-analyzer et Dev Containers).

## Installation et configuration

### Étape 1: Cloner le dépôt

```bash
git clone https://github.com/Oignontom8283/eadkp.git
cd eadkp
```

### Étape 2: Démarrer l'environnement Docker

Un fichier `start.sh` est fourni pour automatiser le démarrage de l'environnement Docker.

```bash
chmod +x start.sh
./start.sh
```
La construction de l'image Docker peut prendre quelques minutes lors du premier lancement.

### Étape 3: Accéder au conteneur

Une fois le conteneur lancé, vous pouvez vous y connecter via VSCode en utilisant l'extension "Dev Containers".

![Connect to container](.//assets/Tuto_dev-container_1.png)

### Étape 4: Redéfinir la cible de compilation

Il arrive très souvent que pendant le montage de l'image, la cible de compilation Rust ne soit pas correctement définie.

Pour redéfinir la cible, ouvrez un terminal dans le conteneur Docker et exécutez la commande suivante:

```bash
rustup target add thumbv7em-none-eabihf
```

Sinon vous aurez probablement une erreur de type:
```bash
error[E0463]: can't find crate for `core`
```

### Étape 5: Compiler

Pour compiler le projet, utilisez la commande suivante dans le terminal du conteneur Docker:

```bash
just build
```

Cela va compiler le projet pour le matériel Numworks.

Pour vérifier que l'exemple de test se compile correctement, suivez le guide [Guide de compilation de l'exemple de test](BuildExample.md).

## Résolution des problèmes

Si vous rencontrez des problèmes lors de la mise en place de l'environnement, assurez-vous que:
- Docker et Docker Compose sont correctement installés et fonctionnels.
- Vous avez les permissions nécessaires pour exécuter Docker (parfois, il est nécessaire d'ajouter votre utilisateur au groupe Docker).
- Votre éditeur de code est correctement configuré pour travailler avec des conteneurs Docker.
- Si vous êtes sous Windows, que vous utilisez bien WSL2 et que Docker Desktop soit installé sur votre Windows.

Si le problème persiste, n'hésitez pas à demander de l'aide !