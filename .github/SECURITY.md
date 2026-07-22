# Politique de sécurité

## À propos de ce projet

Ce projet est une lib/SDK/HAL destinée à créer des applications externes pour calculatrice NumWorks. Par nature :

- Il n'y a **aucun accès réseau**.
- Il n'y a **aucune donnée utilisateur** collectée, stockée ou transmise.
- Il n'y a **aucun service en ligne** associé au projet.

Autrement dit, ce projet n'a pas de surface d'attaque au sens classique du terme (pas de serveur, pas de compte, pas de données sensibles à protéger).
Il n'y a donc pas de programme de bug bounty, de procédure de divulgation coordonnée, ni de contact sécurité dédié à mettre en place ici.

## Bugs et comportements anormaux dans la lib elle-même

Si vous constatez un bug dans la lib (crash, comportement indéfini, panic inattendu, etc.),
merci d'ouvrir simplement une **issue classique** sur le dépôt, comme pour n'importe quel autre bug. Voir le `CONTRIBUTING.md` pour les modalités.

## Problèmes liés à la calculatrice NumWorks elle-même

Si vous découvrez une faille de sécurité concernant la calculatrice NumWorks, son système d'exploitation,
ou l'infrastructure NumWorks (site, épreuves, injecteur d'applications, etc.), **cela ne concerne pas ce projet**.
Merci de signaler ce type de problème directement à NumWorks, via leurs canaux officiels.