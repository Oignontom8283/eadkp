
- Documentation sur le support des fichiers C et C++

- Repertoire de travail dinamiquement configurable pour le script de build.

- Synchronisé .cargo/config.toml du projet utilisateur avec les exigences d'eadkp

## Bugs

- Lors de l'écriture d'un fichier, le premier caractère (octet) est mystérieusement supprimé. Cela peut être contourné en écrivant un caractère en plus au début, avant le contenu réel.
- Les fichuiers C et C++ ne sont pas compilés et liés l'ors de l'utilisation d'eadkp comme dépendance dans un autre projet Rust.