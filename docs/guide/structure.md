
# Application Structure

The template includes a base application structure with sample code to help you get started.

You can strip out the example code and keep only the bare minimum:

```rust
#![cfg_attr(target_os = "none", no_std)]
#![no_main]

// use alloc::format; // !If you have a error about missing `format!`, uncomment this line to import it from alloc.

#[macro_use]
extern crate eadkp;

eadk_setup!(name = "Your App");

#[unsafe(no_mangle)]
pub fn main() -> isize {

    _eadk_init_heap(); // Initialize the allocator, required for using: vec, string, format!, etc.

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

This does nothing beyond the minimum: clear the screen and exit on a keypress.

> [!IMPORTANT]
> Your application **must** have a way to exit. Without one, behavior is undefined — the simulator will crash, and the calculator may become unresponsive.
>
> There is no exit function in the API. The standard pattern is an infinite loop with a flag (`running`) that you set to `false` to break out.
