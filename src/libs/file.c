#include <stdint.h>

int32_t fonction_utilisee(int32_t x) {
    return x * 2;
}

int32_t grosse_fonction_inutile(int32_t x) {
    int32_t result = 0;
    for (int i = 0; i < 1000; i++) {
        result += x * i * i * i;
    }
    return result;
}

int32_t calcul_complexe_non_utilise(int32_t a, int32_t b) {
    return a * a + b * b + a * b;
}

// Fichier de test pour vérifier l'élimination de code mort.