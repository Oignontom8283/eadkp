## Guide de mise en place de l'environnement de développement Dockerisé

Ce guide vous aide à configurer l'environnement de développement pour Eadkp.

## Pré-requis
- **Docker** et Docker Compose installés sur votre machine (Si vous êtes sous Windows, installez Docker Desktop sur Windows et non dans WSL).
- **Git** installé pour cloner le dépôt (Si vous êtes sous Windows, installez Git dans WSL).
- Un éditeur de code, prenant en charge Rust et l'intégration avec Docker (par exemple, VSCode avec l'extension Dev Containers).

## Installation

### Cloner le dépôt

```bash
git clone https://github.com/Oignontom8283/eadkp.git
cd eadkp
```

> [!IMPORTANT]
> Sous Windows, utilisez WSL et exécutez les commandes dans WSL.

## Démarrage

Un fichier `start.sh` est fourni pour automatiser le démarrage de l'environnement Docker.

```bash
chmod +x start.sh
```
```bash
./start.sh
```

> [!NOTE]
> La construction de l'image Docker peut prendre un certain temps lors du premier lancement car elle inclut de nombreuses dépendances.

## Accéder au conteneur

Une fois le conteneur lancé, pour commencer à développer il faut entrer dans le conteneur.

### VSCode

Sur votre OS principal, ouvrez VSCode.
Téléchargez l'extension [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers).

Connectez-vous à votre conteneur Docker via VSCode en utilisant l'extension.

![Connect to container](.//assets/Tuto_dev-container_1.png)

Vous pouvez maintenant travailler à l'intérieur de votre conteneur Docker.

### Terminal
Dans votre terminal, utilisez le script `shell.sh` pour accéder au conteneur Docker.

```bash
chmod +x shell.sh
./shell.sh
```

## Étapes suivantes

- [Compiler l'example de test (exportation)](./BuildExample.md)
- [Pour utiliser l'example de test dans un simulateur](./Simulator.md)