# Rapport d'Optimisation et de Refactorisation

## 1. Analyse de Performance

### Audio (src/hardware/sound.rs)
- **Problème** : L'utilisation de `Arc<Mutex<Vec<u8>>>` pour le buffer audio peut introduire de la latence ou des micro-coupures (jitter) en raison de la contention du verrou (lock contention) entre le thread de rendu audio et le thread du CPU.
- **Solution** : Remplacer le `Mutex<Vec<u8>>` par un **RingBuffer** (buffer circulaire) sans verrouillage (lock-free) comme le crate `ringbuf`. Cela permet au CPU de pousser des données et au thread audio de les consommer de manière asynchrone sans bloquer l'un ou l'autre.

### Accès Mémoire (src/hardware/memory.rs)
- **Problème** : Les fonctions `read_16` et `read_32` effectuent des accès successifs. Bien que Rust soit sûr, les vérifications de limites répétées sur chaque accès peuvent être optimisées.
- **Solution** : Utiliser des méthodes de lecture par tranches (slices) pour les accès multiples et envisager l'utilisation de `unsafe` pour les accès mémoire directs si les limites sont garanties par le système de `Mapper`.

## 2. Corrections de Bugs (Bugfixes)

### Sécurité Mémoire (src/hardware/memory.rs)
- **Bug** : Dans `read_16`, l'accès à `address + 1` peut provoquer un dépassement de capacité (panic) si l'adresse est à la limite supérieure de la mémoire mappée.
- **Correction** : Ajouter une vérification de limite avant l'accès : `if address > MEM_SIZE - 2 { return 0; }`.

### Gestion des Erreurs (src/hardware/k7.rs)
- **Bug** : La fonction `create_k7file` retourne un `bool`. Cela ne permet pas de savoir *pourquoi* la création a échoué (ex: fichier existant, permissions, disque plein).
- **Correction** : Utiliser `Result<(), K7Error>` avec une énumération d'erreurs personnalisée.

## 3. Refactorisations et Propreté du Code

### Nettoyage du Code Mort (src/hardware/memory.rs)
- **Action** : Supprimer les blocs de code commentés dans `load_rom`. Cela améliore la lisibilité et réduit la surface d'attaque pour les erreurs de maintenance.

### Optimisation des Modificateurs Clavier (src/hardware/keyboard/mod.rs)
- **Action** : Remplacer les quatre champs `bool` de la structure `Modifiers` par un seul champ `u8` utilisant des **BitFlags**. Cela réduit l'empreinte mémoire et permet des comparaisons d'état en une seule opération logique.

### Logging et Débogage
- **Action** : Remplacer les `println!` et `eprintln!` par un système de logging structuré (crate `log` ou `tracing`). Cela permet de rediriger les messages vers des fichiers ou de les désactiver en production.

## 4. Améliorations Architecturales

### Système d'Interruptions (src/int.rs)
- **Suggestion** : S'assurer que le son et l'affichage sont pilotés par des interruptions matérielles plutôt que par une mise à jour manuelle dans la boucle principale. Cela garantit une synchronisation précise avec le cycle d'horloge du M6809.

### Découplage CPU/Son
- **Suggestion** : Au lieu que le CPU écrive directement dans le buffer de son, utiliser un système de registres de sortie. Le CPU écrit dans un registre, et le module son lit ces registres lors de son propre cycle de mise à jour.

## 5. Priorités d'Exécution
1. **Critique** : Correction du bug d'overflow mémoire.
2. **Haute** : Implémentation du RingBuffer pour l'audio (stabilité).
3. **Moyenne** : Refactorisation des BitFlags et suppression du code mort.
4. **Basse** : Migration vers un système de logging structuré.
