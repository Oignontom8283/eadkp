# Contribuer au projet

Merci de vouloir contribuer ! Ce document décrit les principes techniques du projet ainsi que la mise en place de l'environnement de développement. Merci de les lire attentivement avant d'ouvrir une PR (:

## Philosophie du projet

Cette lib est à la fois un **SDK**, un **HAL** et, dans une certaine mesure, un **framework** permettant de créer des applications externes pour la calculatrice **NumWorks**. Elle est pensée comme une fondation sur laquelle une application "hôte" va s'exécuter avec des ressources très limitées. Chaque octet et chaque cycle CPU économisé ici est un octet et un cycle rendu à l'application hôte. Les principes ci-dessous ne sont donc pas des recommandations mais des règles à respecter.

### Rust embedded, pas la std

Le projet cible du Rust embedded : on **ne peut pas** utiliser les outils de la std. Tout code introduit doit rester compatible avec un environnement `no_std`.

### Gestion des erreurs

On ne peut pas utiliser les mécanismes d'erreur "classiques" de Rust (IOError...). Les erreurs doivent transiter par les **enums d'erreurs définies dans `errors.rs`**. Toute nouvelle famille d'erreurs doit être ajoutée à ces enums plutôt que créée ailleurs de façon ad hoc.

### Allocation mémoire

Un allocateur embarqué est disponible, mais il doit être utilisé **le moins possible**. De façon générale :

- Allouer le moins de mémoire possible, quel que soit le type de donnée concerné.
- Privilégier systématiquement le **zero-copy**.
- Toute allocation doit se justifier : si une alternative sans allocation existe (slices, références, buffers réutilisés, types sur la pile...), c'est elle qu'il faut utiliser.

### Sobriété en ressources

La lib doit consommer le moins de mémoire et le moins de CPU possible, afin de laisser un maximum de ressources disponibles à l'application hôte. C'est un critère de revue à part entière : une PR qui fonctionne mais qui gaspille des ressources sans raison ne sera pas acceptée telle quelle.

### Compatibilité matériel / simulateur

Tout ce qui est développé doit fonctionner :

1. Sur le matériel réel (logique métier).
2. Sur le simulateur, qui est compilé en cross-compilation, sur l'architecture et l'OS de la machine hôte (l'ordinateur du développeur).

Le code spécifique au matériel qui n'a pas de sens sur le simulateur doit avoir une implémentation "dummy". Cette implémentation doit, de préférence, renvoyer une erreur `SoftwareError::SimulatorNotSupported`, afin que l'application hôte puisse la traiter proprement plutôt que de subir un comportement silencieux ou incohérent.

### Robustesse

- Il faut éviter **à tout prix** de faire planter le CPU.
- Utiliser `Result` partout où une opération peut échouer.
- Un `panic!` n'est acceptable qu'en tout dernier recours, lorsqu'aucune autre issue n'est possible.
- Aucun **comportement indéfini** (UB) n'est toléré, sous aucun prétexte.

### Contraintes matérielles spécifiques

Cette section liste les instructions, fonctions ou méthodes Rust connues pour être problématiques sur notre cible matérielle et donc interdites d'utilisation.
Si vous découvrez une instruction ou une fonction non listée ici qui pose problème, faites-le savoir dans une issue ou une PR, afin que nous puissions nous y pencher.

#### Instructions ASM LDREX/STREX proscrites

Toute instruction (ou méthode Rust générant, directement ou indirectement, des instructions asm `LDREX`/`STREX`) est **strictement interdite**.

Pour des raisons techniques, ces instructions provoquent un **bus fault** (donc un crash CPU) sur notre cible. Leur utilisation est donc formellement interdite, sans exception.

Fonctions/méthodes connues à ce jour pour générer ces instructions (liste non exhaustive) :

- `swap` sur les types atomiques (`core::sync::atomic::Atomic*::swap`)
- `compare_exchange` et `compare_exchange_weak`
- `fetch_add`, `fetch_sub`, `fetch_and`, `fetch_or`, `fetch_xor`, `fetch_nand`, `fetch_max`, `fetch_min`
- Plus généralement, toute opération atomique **read-modify-write** (RMW), ainsi que toute structure (spinlock, mutex lock-free, `Once`, `OnceCell`...) qui repose en interne sur une opération de ce type via CAS (compare-and-swap)
- Les intrinsics ARM bas niveau équivalents (`__LDREXW`/`__STREXW` et variantes)


### Blocs `unsafe`

Le projet étant un SDK bas niveau, les blocs `unsafe` sont nombreux et il n'est **pas obligatoire de les documenter systématiquement**. En revanche, chaque bloc `unsafe` doit être écrit de façon à ce que son utilité et son caractère indispensable soient compréhensibles **à la simple lecture**, sans commentaire nécessaire pour le justifier.

### Écran de panic

La lib fournit un **écran de panic** intégré : lorsqu'une application panique, un écran rouge s'affiche avec le message d'erreur, directement sur la calculatrice. C'est un système bien intégré et pratique en Rust, qui permet d'avoir un message d'erreur lisible même sur le matériel. Ce mécanisme est avant tout pensé pour le scope des applications qui utilisent la lib, mais il peut également nous servir en interne.

### Ergonomie des types de l'API

Il faut privilégier des types qui simplifient l'utilisation de l'API. Par exemple, retourner une `String` plutôt qu'une slice d'octets sera souvent plus pratique à utiliser pour l'appelant, lorsque c'est la seule option raisonnable. C'est cependant un équilibre à trouver au cas par cas avec les principes de sobriété mémoire et de zero-copy évoqués plus haut : le confort d'utilisation ne doit pas devenir une excuse pour allouer sans réfléchir.

## Environnement de développement

### Windows

Sur Windows, le développement se fait via **WSL**, avec **Docker** installé (de préférence côté Windows, c'est ce qui est conseillé).

⚠️ Sous WSL, clonez le dépôt dans votre espace de fichiers **Linux**, et non dans l'espace de fichiers Windows (`/mnt/c/...`), sous peine de tuer vos performances.

### Docker

Une fois le dépôt cloné :

```bash
chmod +x ./docker.sh
./docker.sh start
```

Commandes disponibles :

| Commande | Description |
|---|---|
| `./docker.sh shell` | Accéder au shell du conteneur |
| `./docker.sh open code` | Ouvrir VSCode dans le conteneur comme s'il s'agissait d'un dossier local (fonctionne à travers WSL pour ouvrir dans VSCode Windows). Nécessite l'extension **Dev Containers** installée dans VSCode |
| `./docker.sh open explorer` | Ouvrir le dossier où se trouve le répertoire local sur la machine (y compris sous WSL) |
| `./docker.sh stop` | Arrêter le conteneur |
| `./docker.sh remove` | Supprimer le conteneur |
| `./docker.sh restart` | Redémarrer le conteneur |

> N'essayez pas d'utiliser les commandes Docker standard, ça ne fonctionnera pas. 

## Exemples

Les exemples se trouvent dans le dossier `examples/`. Pour les sélectionner (build, simulation...), il suffit d'indiquer le nom de leur fichier, sans extension (ex : `specs`, `snake`).

Les fichiers `*_lib.rs` **ne sont pas des exemples** : ce sont des bouts de code partagés, utilisés par les exemples.

## Commandes `just`

Dans le conteneur, on utilise l'utilitaire **`just`** (équivalent moderne à Makefile) pour les différentes actions du projet.

| Commande | Description |
|---|---|
| `just check` | Vérifie que la lib compile (seulement la lib, pas les exemples) |
| `just export {nom_de_l'example}` | Compile l'exemple et génère dans `build/` un fichier `.nwa` : le fichier d'application à installer sur la calculatrice via un injecteur |
| `just doc` | **Important** : ouvre en local une prévisualisation du site web de documentation de la lib. À utiliser systématiquement si vous contribuez à la documentation |
| `just sim {example} {nombre_de_coeurs}` | Télécharge et compile le simulateur en local puis l'ouvre sur l'exemple demandé (voir ci-dessous) |
| `just clean` | Nettoie les fichiers de compilation Rust et le dossier `build/` |
| `just clear` | Nettoie les fichiers de compilation Rust, le dossier `build/`, **et supprime le simulateur** |

### Simulation

```bash
just sim {example} {nombre_de_coeurs_pour_la_compilation}
```

⚠️ Cette commande doit être lancée dans un **vrai terminal** (pas celui d'un IDE), à l'intérieur du shell du conteneur (`./docker.sh shell`).

Elle télécharge et compile le simulateur en local, puis l'ouvre sur l'exemple demandé.
Il est conseillé d'allouer **le plus de threads possible** au simulateur a la première compilation, afin d'accélérer le processus.
Attention : le nombre de threads alloué impacte directement la vitesse à laquelle le programme tourne, pas seulement le temps de compilation !

---

En cas de doute sur l'un de ces principes, ouvrez une issue avant de commencer à coder : il vaut mieux clarifier en amont que de devoir réécrire une PR entière.