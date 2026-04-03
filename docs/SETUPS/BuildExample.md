# Guide de compilation de l'exemple de test

Ce guide vous montre comment compiler et exporter le projet d'exemple/teste pour le matériel NumWorks.

## Prérequis
- [Setup](./Setup.md)

## Compilation

Ne fait que compiler, pas exploitable.

```
just build
```

## Exportation

Exporter le projet sous forme d'archive `.nwa` (format d'archive pour les applications NumWorks).

```
just export
```

Le `.nwa` est généré dans le dossier `build/`. Pour l'installer sur votre calculatrice, [suivez ce guide](https://yaya-cout.github.io/Nwagyu/guide/help/how-to-install.html)