# Contribuer au projet

Merci de votre intérêt pour contribuer à ce projet.

## Principe

Le projet maintient un haut niveau d’exigence sur la qualité du code.
Les contributions doivent viser un code correct, lisible et maintenable.

Le projet cible un environnement contraint. Les décisions techniques doivent toujours privilégier la performance, l’efficacité et l’usage minimal des ressources.

---

## Comment contribuer ?

Vous pouvez contribuer en :

* Créant une pull request pour proposer des changements
* Ouvrant une issue pour signaler un bug
* Ouvrant une issue pour proposer une amélioration ou une nouvelle fonctionnalité
* Participant à la revue de code des pull requests existantes
* Aidant les autres contributeur·rice·s en répondant à leurs questions ou en les guidant

> Si vous identifiez une amélioration ou une erreur, vous n’êtes pas obligé·e de la corriger vous-même :
> vous pouvez ouvrir une issue pour en discuter, ou proposer directement une pull request.


## Contraintes techniques

Ce projet implique :

* Développement en **Rust**
* Cible **bas niveau** (proche du matériel, programmation embarquée, sans OS).
* Environnement contraint (**Cortex-M7**, 64 Ko de RAM).
* Consommation minimale des ressources (la bibliothèque doit laisser un maximum de ressources à l’application).
* Optimisation forte des performances.
* Minimisation stricte de l’utilisation mémoire (critique).


### Stabilité

* Le crash CPU doit être ÉVITÉ À TOUT PRIX !
* Un crash entraîne la **réinitialisation de la calculatrice** et la **perte des données utilisateur**. Très contraignant pour les utilisateurs.
* En cas d’erreur, privilégier un retour d’erreur (`Result`).
* Un `panic` est acceptable en dernier recours.
* Les comportements indéfinis (UB), overflows non gérés ou accès mémoire invalides sont inacceptables.


## Environnement de développement

Le projet fournit un environnement de développement complet :

* Docker: Utilisez le conteneur fourni. Il garantit une configuration identique pour tous et inclut tout le nécessaire pour compiler et exécuter le simulateur
* justfile: Fournit les commandes nécessaires pour compiler, simuler, exporter, etc.
* Simulateur: Permet de tester sans réinjecter l’application sur la calculatrice à chaque modification *(ne pas utiliser depuis l’IDE)*
* CI (GitHub Actions): Vérifie automatiquement que le projet compile correctement
* Tests: Pas de tests unitaires actuellement, la CI assure la validation de compilation
* Configuration d’outils: Configuration fournie pour `rust-analyzer`
* Setup: Suivez les guides de configuration dans le README


## Exigences de code

* Le code doit être correct, lisible et maintenable
* Documenter les blocs `unsafe` dans les cas complexes/subtils.
* Le code doit fonctionner **à la fois sur la cible réelle (arch ARM Cortex-M7) et sur le simulateur (arch x86_64)**.
  * Utilisez des implémentations `dummy` pour les parties dépendantes du matériel.
* Respectez les conventions de style du projet.
* L’ajout de dépendances externes nécessite une validation préalable.
* Évitez les allocations/copies inutiles
    * privilégiez le *zero-copy* lorsque c’est possible
* N’utilisez pas uniquement la stack :
    * Embeddable allocateur fourni.
    * Si utiliser la heap est plus pertinent, alors utilisez-la. (ex: listes a taille dynamique)


## Avant de contribuer

* Vérifiez qu’il n'existe pas déjà une issue ou une pull request similaire
* Respectez les conventions et le style du projet
* Privilégiez des changements simples et ciblés
* Évitez les abstractions inutiles ou coûteuses


## Pull requests

Une pull request doit :

* Être claire et compréhensible
* Avoir un objectif précis
* Respecter les contraintes techniques
* Être prête à évoluer suite aux retours

Si vous envisagez une modification massive, complexe ou architecturale, ouvrez d'abord une issue pour en discuter.


## Revue de code

Les reviews ont pour objectif d’améliorer le code et de garantir le respect des contraintes.

* Toute personne peut participer aux discussions.
* La décision finale revient aux mainteneur(euse)(s).
* Les retours doivent être constructifs et argumentés.
* Les impacts en performance, mémoire et stabilité doivent être pris en compte.
* Les remarques doivent être traitées ou discutées.

Une pull request peut être refusée si les contraintes ne sont pas respectées.


## Refus de contribution

Une contribution peut être refusée si :

* Elle ne respecte pas les contraintes techniques.
* Elle dégrade les performances, la mémoire ou la stabilité.
* Elle introduit des risques de crash.
* Les retours ne sont pas pris en compte.
* Elle n’est pas pertinente pour le projet.

> Les contributions ne sont pas rejetées pour leur qualité initiale, mais peuvent l’être si la qualité finale attendue n’est pas atteinte après échanges.
