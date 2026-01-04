# Guide d'utilisation du simulateur

Ce guide explique comment utiliser le simulateur Numworks officiel pour tester le fonctionnement de l'exemple de test.

## Pré-requis

- Avoir configuré l'environnement de développement selon le [Guide de setup du projet](Setup.md).
- Avoir compilé l'exemple de test selon le [Guide de compilation de l'exemple de test](BuildExample.md).

## Étapes pour utiliser le simulateur

### 1. Ouvrez un terminal natif
Sur **Windows**, utilisez PowerShell ou l'invite de commande. **Pas le terminal intégré de VSCode !**
Sur **Linux** ou **MacOS**, utilisez votre terminal habituel. **Pas le terminal intégré de VSCode !**

### 2. Entrez dans WSL2 (**Windows Only**, other OS skip this step):

```bash
wsl
```

### 3. Accédez au répertoire du projet eadkp.

**Dans WSL2**, pas dans Docker, ni Windows. (Windows Only)

Par exemple:

```bash
cd /home/user/projets/eadkp
```

### 4. Entrez dans le Docker via le script `shell.sh`:

```
chmod +x shell.sh
./shell.sh
```

Le script initialise une connexion entre le serveur X de Windows et le conteneur Docker, puis ouvre un terminal utilisant ce lien dans le conteneur Docker.

Pour linux, assurez-vous d'avoir un serveur X fonctionnel (Je crois en vous !) et le lien se fera automatiquement entre le conteneur et votre bureau.

### 5. Lancez le simulateur Numworks avec `just sim`;

```bash
just sim
```

Si le simulateur n'a pas encore été utilisé, il va se télécharger depuis le dépôt officiel de Numworks et se compiler. 

La compilation va être longue et gourmande en ressources, soyez patient !

Une fenêtre représentant la calculatrice Numworks devrait s'ouvrir avec l'application de test automatiquement lancée.

## Résolution des problèmes

Si vous rencontrez des erreurs lors du lancement du simulateur, assurez-vous que:
- Vous avez bien suivi les étapes précédentes.
- Votre serveur X fonctionne correctement (sous Windows, assurez-vous d'utiliser WSL 2 et pas WSL 1, qui ne fournit pas de serveur X par défaut).
- Que Docker et les bonnes permissions sont en place pour permettre la connexion au serveur X.
- Si vous êtes sous Windows, que vous utilisez bien WSL2 et que Docker Desktop soit installé sur votre Windows (Très important !).