# Guide de compilation de l'exemple de test

Ce guide explique comment compiler l'exemple de test fourni avec la bibliothèque `eadkp`.

## Pré-requis

- Avoir configuré l'environnement de développement selon le [Guide de setup du projet](Setup.md).

## Étapes de compilation

1. Ouvrez un terminal dans le conteneur Docker via VsCode ou un autre moyen.

2. Compilez l'exemple

Vu que l'environnement est déjà configuré, cela se fait en une seule commande:

```bash
just export
```

3. Vérifiez que la compilation s'est bien déroulée.

Si tout se passe bien, vous devriez voir un message de succès indiquant que le build est terminé.

Le message de succès devrait afficher l'emplacement du fichier NWA généré, normalement dans le dossier `./build/`.

4. Testez l'application

Utiliser l'injecteur d'application externe de Numworks ou un autre pour envoyer l'application NWA sur votre calculatrice Numworks et tester son fonctionnement.

Site officiel de l'injecteur: https://my.numworks.com/apps

## Résolution des problèmes

Si vous rencontrez des erreurs lors de la compilation, assurez-vous que:
- La cible de compilation `thumbv7em-none-eabihf` est bien ajoutée (voir Étape 4 du [Guide de setup du projet](Setup.md)).
- Vous êtes dans le bon répertoire de travail où se trouve le fichier `justfile`.
- Que les fichiers du projet n'ont pas été modifiés, supprimés, déplacés ou corrompus.
- Si vous êtes sous Windows, que vous utilisez bien WSL2 et que Docker Desktop soit installé sur votre Windows.

Si le problème persiste, n'hésitez pas à demander de l'aide !