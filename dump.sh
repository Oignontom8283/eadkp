#!/bin/bash

LEVEL=""
TAG_NAME=""
TAG_COMMENT=""
VALIDATE_ALL=false
ALLOW_DIRTY=false
AUTO_CONFIRM=false
INITIAL_STAGED=$(git diff --cached --name-only) # On mémorise ce qui était déjà indexé

cleanup() {
    echo "Nettoyage des modifications temporaires..."
    # On désindexe tout ce qui a été ajouté par le script
    git reset > /dev/null 2>&1
    # On restaure le Cargo.toml à son état initial
    git checkout Cargo.toml > /dev/null 2>&1
    # On remet l'index comme il était avant le script
    if [ -n "$INITIAL_STAGED" ]; then
        echo "$INITIAL_STAGED" | xargs git add > /dev/null 2>&1
    fi
}

while [[ $# -gt 0 ]]; do
    case $1 in
        patch|fix) LEVEL="patch"; shift ;;
        minor|feat) LEVEL="minor"; shift ;;
        major|breaking) LEVEL="major"; shift ;;
        -a|--tag-name) TAG_NAME="$2"; shift 2 ;;
        -m|--tag-comment) TAG_COMMENT="$2"; shift 2 ;;
        --all) VALIDATE_ALL=true; shift ;;
        -d|--dirty) ALLOW_DIRTY=true; shift ;;
        -y|--yes) AUTO_CONFIRM=true; shift ;;
        *) echo "Usage: $0 [patch|minor|major] [-a name] [-m msg] [--all | -d] [-y]"; exit 1 ;;
    esac
done

# Check que cargo n'était pas déjà modifié
if git status --porcelain Cargo.toml | grep -q "M"; then
    echo "Erreur : Cargo.toml a déjà des modifications. Committez-les d'abord."
    exit 1
fi

# Validité des args
if [ -z "$LEVEL" ]; then
    echo "Erreur: Précisez patch, minor ou major."
    cleanup
    exit 1
fi

# Sécurité : pas de --all et -d en meme temps
if [ "$VALIDATE_ALL" = true ] && [ "$ALLOW_DIRTY" = true ]; then
    echo "Erreur : --all et -d sont incompatibles."
    exit 1
fi

# Si --all, on indexe tout
if [ "$VALIDATE_ALL" = true ]; then
    git add .
fi

# On regarde ce qui est indexé
STAGED_FILES=$(git diff --cached --name-only)
DIRTY_FILES=$(git status --porcelain | grep -v "Cargo.toml")

# Si rien n'est indexé mais qu'il y a des modifs (Dirty)
if [ -z "$STAGED_FILES" ] && [ -n "$DIRTY_FILES" ] && [ "$ALLOW_DIRTY" = false ]; then
    echo "Dépôt dirty : des fichiers sont modifiés mais rien n'est indexé."
    echo "Utilisez --all pour inclure ou -d pour ignorer."
    exit 1
fi

if [ -n "$STAGED_FILES" ]; then
    if [ "$AUTO_CONFIRM" = false ]; then
        echo "Fichiers actuellement indexés :"
        echo "$STAGED_FILES"
        read -p "Inclure ces fichiers dans le commit de version ? (o/n) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Oo]$ ]]; then
            cleanup
            echo "Annulation effectuée."
            exit 1
        fi
    fi
fi

# Bump de la version
OUTPUT=$(cargo set-version --bump $LEVEL 2>&1)
if [ $? -ne 0 ]; then 
    echo "Erreur Cargo : $OUTPUT"
    cleanup
    exit 1
fi

NEW_VERSION=$(echo "$OUTPUT" | awk '{print $NF}')
echo "Passage à la version $NEW_VERSION"

# Indexation finale du Cargo.toml
git add Cargo.toml

# commit
if ! git commit -m "chore: bump version to $NEW_VERSION"; then
    echo "Erreur lors du commit."
    cleanup
    exit 1
fi

FINAL_TAG=${TAG_NAME:-"v$NEW_VERSION"}
COMMENT=${TAG_COMMENT:-"Release $FINAL_TAG"}

# tag
if ! git tag -a "$FINAL_TAG" -m "$COMMENT"; then
    echo "Erreur lors du tag. Annulation du commit..."
    git reset --hard HEAD~1
    exit 1
fi

echo ""
echo "TERMINER !"
echo "Dump a la version v$NEW_VERSION"
echo ""
echo ">   git push --tags"
echo ""