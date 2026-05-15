
# Configuration (FR - Not translated yet)

Une fois votre projet crée, il sera préconfiguré pour fonctionner dés le départ.

Cependant, il a plusieurs choses que vous devez savoir pour pouvoir personnaliser votre projet.


## Nom

Pour changer le nom de votre projet, c'est très simple, il vous suffit de change le champ `name` de `[package]` dans le fichier `Cargo.toml`.

```toml {3}
...
[package]
name = "my_apa"
version = "0.1.0"
edition = "2024"
...
```

Cela changera automatiquement le nom utilisé par toutes les commandes de la CLI et les scripts de votre projet.

## Cargo.toml

Pour ce qui est des autres champs de votre `Cargo.toml`, il est déconseillé de les changer,
le tout est préconfiguré pour fonctionner avec des paramètres spécifiques. 

Si vous ne savez pas ce que vous faites, il est préférable de ne pas toucher à ces champs.