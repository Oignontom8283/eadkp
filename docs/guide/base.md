# Base of the application

> [!NOTE]
> The template provides default code in the `src/main.rs` file that you can reuse.
> Here, we detail this base code to help you understand how it works and how to get started with your project.

## Code

**The following code is MANDATORY for your project to work, it is the base of your application!**

### Environment Declaration

Let's indicate the target environment to Rust:

```rust
#![cfg_attr(target_os = "none", no_std)]
#![no_main]

```

We indicate to Rust that our project runs without an operating system (OS) and that we do not have a standard `main` entry function.

### Application Configuration

We import the macros from the `eadkp` crate and use them:

```rust
#[macro_use]
extern crate eadkp;

// Setup the NWA environment.
eadk_setup!(name = "Your App Name")

```

We use the `eadk_setup!` macro to configure our project's environment by assigning it a name.

This macro does many things behind the scenes, notably:

* Declaring a `panic_handler` to handle panics in our project and display them on a *Red Screen of ERROR* (RSE).
* Declaring a global allocator (*Embedded Allocator*) to replace Rust's default allocator.
* Configuring the project's mandatory properties: name, level, icon location, etc.

### Entry Function

Although we declared the absence of a standard `main` function, we still need an entry function that will be exposed and called by Epsilon:

```rust
#[unsafe(no_mangle)]
pub fn main() -> isize {
    
    _eadk_init_heap();

    // ...
    
    0
}

```

* `_eadk_init_heap();`: a function automatically introduced by the `eadk_setup!` macro to initialize the global allocator.
* `#[unsafe(no_mangle)]`: indicates that this function is exposed and its name must not be altered (mangled) by the compiler.
* `pub`: makes the function public to expose it outside the module.
* `0`: we return 0 because that is the behavior expected by the API.

In summary, the `main` function declared here is not a standard entry function (like on a classic OS), but a real C-compatible function. Indeed, our application is actually a library compiled in the `cdylib` format (similar to a DLL), and Epsilon is responsible for calling it. This is why it does not follow the usual conventions of OS applications.

Obviously, to use features requiring allocation, you will need to import them not from `std`, but from `core` or `alloc` (if you are using dynamic allocations), and then use them as usual.

### Code Base

In this `main` function, you are free to do whatever you want: it is the entry point of your project, where everything begins.

However, there is a minimal structure to respect so that the application at least stays open and can close properly:

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

Overall, this is a simple infinite loop that reads user inputs. If the "Back" key is pressed, the program breaks out of the loop, which terminates the application by returning 0.

There you go, this is the base of your project!