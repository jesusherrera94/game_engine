//! Game engine helper macros.
//!
//! These macros wrap frequent FFI-based operations such as spawning sprites,
//! moving them, ticking frames, handling input, and managing the main game loop.
//! Their goal is to make Rust-side game scripting concise and expressive.

/// Spawn a sprite, render it immediately, and return its pointer.
///
/// # Example
/// ```ignore
/// let player = spawn_sprite!(50.0, 60.0, 48, 48, 255, 100, 0);
/// ```
#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {{
        let s_ptr = ffi::rust_create_sprite($x, $y, $w, $h, $r, $g, $b);
        ffi::rust_render_sprite(s_ptr);
        s_ptr
    }};
}

/// Move an existing sprite to a new position.  
/// Optionally clears the screen before movement.
///
/// # Example
/// ```ignore
/// move_sprite!(sprite, 250.0, 180.0);
/// move_sprite!(sprite, 250.0, 180.0, true);
/// ```
#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {{
        ffi::rust_update_sprite_position($sprite, $x, $y);
        ffi::rust_render_sprite($sprite);
    }};
    ($sprite:expr, $x:expr, $y:expr, $do_clear:expr) => {{
        if $do_clear {
            ffi::rust_clear_screen();
        }
        ffi::rust_update_sprite_position($sprite, $x, $y);
        ffi::rust_render_sprite($sprite);
    }};
}

/// Run one frame of the engine loop (update window + maintain ~60 FPS).  
///
/// # Example
/// ```ignore
/// tick!();
/// ```
#[macro_export]
macro_rules! tick {
    () => {{
        ffi::rust_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }};
}

/// Invoke a closure when a key is detected as pressed.
///
/// # Example
/// ```ignore
/// on_key_press!(window, ffi::GLFW_KEY_UP, || move_up());
/// ```
#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $action:expr) => {{
        if ffi::rust_get_key($window, $key) == ffi::GLFW_PRESS {
            $action();
        }
    }};
}

/// Create a window and run the main game loop with custom init, update, and cleanup logic.
///
/// # Example
/// ```ignore
/// start_window_and_game_loop!(
///     "Asteroid Run", 1280, 720,
///     {
///         println!("Game initialized!");
///     },
///     {
///         println!("Frame running!");
///     },
///     {
///         println!("Game ended!");
///     }
/// );
/// ```
///
/// If no name or dimensions are provided, defaults are used:
/// ```ignore
/// start_window_and_game_loop!({},{},{});
/// ```
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $init:block, $frame:block, $cleanup:block) => {{
        ffi::rust_create_game_window($title, $width, $height);
        $init
        while !ffi::rust_window_should_close() {
            $frame
            tick!();
        }
        $cleanup
    }};
    ($init:block, $frame:block, $cleanup:block) => {{
        ffi::rust_create_game_window("Untitled Demo", 1024, 768);
        $init
        while !ffi::rust_window_should_close() {
            $frame
            tick!();
        }
        $cleanup
    }};
}