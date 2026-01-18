
<img src="docs/assets/eadkp.svg" alt="eadkp logo" style="vertical-align: middle; width: 50%;"/>
</h1>

`eadkp` est une bibliothèque Rust destinée au développement d’applications pour
les calculatrices **NumWorks** sous **Epsilon**.

Elle fournit des fonctionnalités de bas niveau permettant d’interagir avec le
matériel de la calculatrice, notamment la gestion de l’affichage, des entrées
utilisateur, de la batterie et du stockage.

La bibliothèque propose également des abstractions de plus haut niveau afin de
simplifier le développement d’applications en Rust, telles que la gestion du
*panic handler*, de l’allocateur global, ainsi que la déclaration des propriétés
des applications **NWA**.

## Fonctionnalités

- [x] Handlers Rust pour l'ABI Epsilon
- [x] Gestion basique de l'affichage
- [x] Gestion des entrées utilisateur (clavier)
- [x] Gestion de la batterie
- [x] Gestion du stockage (lecture/écriture de fichiers)
- [x] Macros pour déclarer les propriétés des applications NWA
- [x] Gestion simple des images (inclusion et affichage) via macro
- [x] Support des fichiers C et C++ (Non documenté)
- [x] Support du simulateur officiel Numworks
- [ ] Support des fichiers données a l'inclusion dans les applications NWA
- [ ] Support des graphiques avancés
- [ ] Débogage via USB (Pas encore évaluée la faisabilité)

## Installation

**Cette section est incomplète et en cours de rédaction.**

Configurez votre projet pour la cible `thumbv7em-none-eabihf` et les options de compilation spécifiques aux NWA.

> Voir ./cargo/config.toml pour un exemple de configuration.
> 
> Voir ./examples/eadkp_example pour un exemple de projet utilisant eadkp.
> 
> Voir  ./build.rs pour la configuration de la chaîne d'outils
> et ./docker-compose.yml, ./dockerfile pour un environnement de compilation Dockerisé.

Install Eadkp via Cargo:
```bash
cargo add eadkp
```

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir des issues ou à soumettre des pull requests.

Pour apprendre à utiliser le projet, consultez les guides suivants :
- [Guide de setup du projet](docs/GUIDS/Setup.md)
- [Guide de compilation de l'exemple de test](docs/GUIDS/BuildExample.md)
- [Guide d'utilisation du simulateur](docs/GUIDS/Simulator.md)

## Licence
Ce projet est sous licence [GPL-3.0](./LICENSE) (GNU General Public License v3.0).

Technologie de `storage.rs` basée sur [NumWorks Extapp Storage](https://framagit.org/Yaya.Cout/numworks-extapp-storage) sous [licence MIT](https://framagit.org/Yaya.Cout/numworks-extapp-storage/-/blob/master/LICENSE-MIT) (commit: 62e3d4c44437b93a8f14ce687a1c45d6dded87d9).

Projet basé sur l'eadk et utilitaire de [NumCraft version v0.1.4](https://github.com/yannis300307/NumcraftRust/tree/v0.1.4) sous [licence GPL-3.0](https://github.com/yannis300307/NumcraftRust/blob/v0.1.4/LICENSE).

## Remerciements

Merci à [Yannis300307](https://github.com/yannis300307) pour son travail sur NumCraft Rust, qui a servi de base à ce projet.

Également merci à [Yaya Cout](https://framagit.org/Yaya.Cout) pour son travail sur la manipulation du file system interne de la NumWorks par des applications externes.