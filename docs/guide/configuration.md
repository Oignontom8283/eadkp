
# Configuration (FR - Not translated yet)

Une fois votre projet crée, il sera préconfiguré pour fonctionner dés le départ.

Cependant, il a plusieurs choses que vous devez savoir pour pouvoir personnaliser votre projet.


## Nom

Pour changer le nom de votre projet, c'est très simple, il vous suffit de change le champ `name` de `[package]` dans le fichier `Cargo.toml` :

```toml {3}
...
[package]
name = "my_apa"
version = "0.1.0"
edition = "2024"
...
```

Le reste du fichier `Cargo.toml` est préconfiguré pour fonctionner avec les utilitaires de votre projet,
il est déconseillé de le modifier, sauf si vous savez ce que vous faites.


## .eadkp/

Le dossier `.eadkp/` contient principalement des fichiers de script et de configuration.
**Il ne faut pas les modifier, supprimer ou déplacer ces fichiers**, les script son téléchargés en local pour fonctionner hors ligne.
Il sont mise a jour par le le script d'update, et restoré en cas de suppression ou de modification.

Le fichier `config.env` contient basiquement de la configuration du script de mise à jour des utilitaires de votre projet, il est déconseillé de la changer, sauf si vous savez ce que vous faites.


## justfile

Le `justfile` (un équivalent à un Makefile) fournit ne contient que l'importation des commandes de base fournies dans `.eadkp/` : `export`, `build`, `sim`, etc ...

C'est ici a la suite de l'importation que vous pouvez ajouter vos propres commandes personnalisées, ou même écraser les commandes de base si vous le souhaitez.