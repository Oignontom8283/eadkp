# Base of the application

La template que nous avons utilisée inclut une structure de base pour l'application et du code d'exemple pour vous aider à démarrer. 

Vous pouvez supprimer le code suivant et ne garder que la structure de base si vous le souhaitez :
```rust 18-31,47-57,61-99
#![cfg_attr(target_os = "none", no_std)]
#![no_main]

// use alloc::format; // !If you have a error about missing `format!`, uncomment this line to import it from alloc.

#[macro_use]
extern crate eadkp;

eadk_setup!(name = "Your App");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    _eadk_init_heap();

    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE);

    // ------------------------------------------------------------------------- {

    eadkp::display::draw_string(
        "Hello world!",
        eadkp::Point { x: 10, y: 10 },
        true,            // use large font
        eadkp::COLOR_BLACK,
        eadkp::COLOR_WHITE,
    );

    let mut number: i32 = 0;
    let mut actualize = true;

    // ------------------------------------------------------------------------- }

    let mut prev = eadkp::input::KeyboardState::scan();

    let mut running = true; // Application main loop flag. Set to false to exit.

    while running {
        let now = eadkp::input::KeyboardState::scan();

        let just = now.get_just_pressed(prev);

        if just.key_down(eadkp::input::Key::Back) {
            running = false;
        }


        // ------------------------------------------------------------------------- {

        if just.key_down(eadkp::input::Key::Plus) {
            number += 1;
            actualize = true;
        } else if just.key_down(eadkp::input::Key::Minus) {
            number -= 1;
            actualize = true;
        }

        // ------------------------------------------------------------------------- }

        eadkp::display::wait_for_vblank();

        // ------------------------------------------------------------------------- {

        if actualize {
            eadkp::display::push_rect_uniform(
                eadkp::Rect {
                    x: 10,
                    y: 30,
                    width: eadkp::LARGE_FONT.width
                        * ((eadkp::SCREEN_RECT.width - 10) / eadkp::LARGE_FONT.width),
                    height: eadkp::LARGE_FONT.height,
                },
                eadkp::COLOR_WHITE,
            );

            let text_color = if number > 0 {
                eadkp::COLOR_BLACK
            } else {
                eadkp::COLOR_WHITE
            };
            let bg_color = if number > 0 {
                eadkp::COLOR_GREEN
            } else if number < 0 {
                eadkp::COLOR_RED
            } else {
                eadkp::COLOR_GRAY
            };

            eadkp::display::draw_string(
                &format!("Number: {}", number),
                eadkp::Point { x: 10, y: 30 },
                true, // use large font
                text_color,
                bg_color,
            );

            actualize = false;
        }

        // ------------------------------------------------------------------------- }

        prev = now;
    }

    0
}
```

Ce qui donne approximativement ceci :
```rust
#![cfg_attr(target_os = "none", no_std)]
#![no_main]

// use alloc::format; // !If you have a error about missing `format!`, uncomment this line to import it from alloc.

#[macro_use]
extern crate eadkp;

eadk_setup!(name = "Your App");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    _eadk_init_heap(); // Initialize the allocator, required for using : vec, string, format!, etc.

    eadkp::display::push_rect_uniform(eadkp::SCREEN_RECT, eadkp::COLOR_WHITE); // Clear the screen

    let mut prev = eadkp::input::KeyboardState::scan();
    let mut running = true; // Application main loop flag. Set to false to exit.

    while running {
        let now = eadkp::input::KeyboardState::scan();

        let just = now.get_just_pressed(prev);

        if just.key_down(eadkp::input::Key::Back) {
            running = false;
        }

        // Your code logic here . . .

        eadkp::display::wait_for_vblank();

        // Your code rendering here . . .

        prev = now;
    }

    0
}
```

Le code ci-dessus **ne fait rien** a part le minimum, soit : clear l'écran, exit par une touche.

> [!NOTE]
> Il est obligatoire d'avoir une méthode de sortie de l'application, sinon vous allez avoir des comportements indéfinis (ex: le simulateur va planter).
> 
> Il n'y a pas de fonction d'exit dans l'API, il faut donc faire une boucle infinie et sortir de celle-ci pour quitter l'application.