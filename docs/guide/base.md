
# Base of app

> [!NOTE]
> La template fournie du code par defaut dans le fichier `src/main.rs` que vous pouvez réutiliser.
> 
> Ici nous détailleons le code de base pour vous aider à comprendre comment il fonctionne, et comment commencer votre projet.

## Code

**Le code qui suit est OBLIGATOIRE pour que votre projet fonctionne, c'est la base de votre projet!**

### Déclaration de l'environnement

Déclaration de l'environnement a Rust :
```rust
#![cfg_attr(target_os = "none", no_std)]
#![no_main]
```
On indique a Rust que notre projet fonctionne sans OS, et que nous n'avons pas de fonction d'entrée standard `main` classique.

### Configuration de l'application

On importe les macros de la crate `eadkp` et on l'utilise :
```rust
#[macro_use]
extern crate eadkp;

// Setup the NWA environment.
eadk_setup!(name = "Your App Name")
```
On utilise la macro `eadk_setup!` pour configurer l'environnement de notre projet, en lui donnant un nom.

Cette macro fait beaucoup de choses en coulisse, notament :
- Déclarer un `panic_handler` pour gérer les panics de notre projet et les afficher sur un Red Screnn of ERROR (RSE).
- Déclare un allocateur global (Embedded Allocator) pour remplacer l'allocateur par défaut de Rust.
- Configurée les proriétés obligatoires du projet : nom, level, emplacement de l'icon, etc ...
- etc ...

### Fonction d'entrée

Bien que nous avons déclaré que nous n'avons pas de fonction d'entrée standard `main`,
nous avont besouin d'une fonction d'entrée qui sera expossée et appellé par Epsilon :

```rust
#[unsafe(no_mangle)]
pub fn main() -> isize {
    
    _eadk_init_heap();

    // ...
    
    0
}
```
- `_eadk_init_heap();` est une fonction automatiquement introduite par la macro `eadk_setup!` pour initialiser l'allocateur global.
- `#[unsafe(no_mangle)]` pour indiquer que cette fonction est expossée et ne doit pas être manglé par le compilateur.
- `pub` pour exposer la fonction à l'extérieur du module.
- `0`, renvoie 0 par ce que c'est comme ça que ça fonctionne, c'est l'api

En gros, la fonction `main` qu'on déclare ici, n'est pas une fonction d'entrée standard, comme sur OS, mais une simple vrai fonction (compatible avec le C),
car nottre app est en vérité une librairie compilé en format `cdylib` (comme une DLL),
et c'est Epsilon qui va l'appeler. C'est pour cela que ça ne respecte pas les conventions des application pour OS.

Évidament, pour utiliser tout fonctionnalité provenante de l'allocateur normale, il va fallouare non pas les importer depuis `std`, mais depuis `core` ou `alloc` (si vous utilisez des fonctionnalités d'allocation dynamique), et les utiliser comme d'habitude.

### Base du code

Dans la fonction `main`, on peut faire ce que l'on veut, c'est le point d'entrée de notre projet, c'est ici que tout commence.

Mais il y a une structure minimale a respecter pour que notre application ne fasse au moins que restée ouverte, et pouvoir ce fermer proprement :

```rust
#[unsafe(no_mangle)]
pub fn main() -> isize {

    // Initialize the heap allocator
    _eadk_init_heap();

    let mut prev = eadkp::input::KeyboardState::scan(); // Initial keyboard state

    loop {
        let now = eadkp::input::KeyboardState::scan(); // Scan the current keyboard state
        let just = now.get_just_pressed(prev); // Get keys that were just pressed
        if just.key_down(eadkp::input::Key::Back) { break 0; }; // Exit if Back key is pressed

        // Clear the screen to white
        eadkp::display::wait_for_vblank(); // Wait for VBlank before updating the display


        // Your own logic here ...


        // Update previous keyboard state
        prev = now;
    }

    return 0;
}
```

Globalement, c'est une simple boucle infinie qui vérifie les entrées de l'utilisateur,
et si la touche "Back" est pressé, elle sort de la boucle ce qui termine l'application en renvoyant 0.

Voila, c'est la base de votre projet.