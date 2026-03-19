/// Spawn a sprite at the given position/size/color, render it, and return its pointer.
///
/// ```ignore
/// let player = spawn_sprite!(50.0, 60.0, 48, 48, 255, 100, 0);
/// ```
#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $w:expr, $h:expr, $r:expr, $g:expr, $b:expr) => {{
        let sprite = $crate::ffi::safe_create_sprite($x, $y, $w, $h, $r, $g, $b);
        $crate::ffi::safe_render_sprite(sprite);
        sprite
    }};
}

/// End-of-frame tick: swap buffers and sleep ~16 ms (~60 FPS).
///
/// ```ignore
/// tick!();
/// ```
#[macro_export]
macro_rules! tick {
    () => {{
        $crate::ffi::safe_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }};
}

/// Move an existing sprite to a new position and re-render it.
/// An optional fourth argument (`true`) clears the screen first.
///
/// ```ignore
/// move_sprite!(sprite, 100.0, 200.0);          // move only
/// move_sprite!(sprite, 100.0, 200.0, true);     // clear then move
/// ```
#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr) => {{
        $crate::ffi::safe_update_sprite_position($sprite, $x, $y);
        $crate::ffi::safe_render_sprite($sprite);
    }};
    ($sprite:expr, $x:expr, $y:expr, $do_clear:expr) => {{
        if $do_clear {
            $crate::ffi::safe_clear_screen();
        }
        $crate::ffi::safe_update_sprite_position($sprite, $x, $y);
        $crate::ffi::safe_render_sprite($sprite);
    }};
}

/// Execute an action when a key is currently pressed.
///
/// ```ignore
/// let window = game_engine::ffi::safe_get_window();
/// on_key_press!(window, game_engine::ffi::GLFW_KEY_UP, || {
///     println!("Up!");
/// });
/// ```
#[macro_export]
macro_rules! on_key_press {
    ($window:expr, $key:expr, $action:expr) => {{
        if $crate::ffi::safe_get_key($window, $key) == $crate::ffi::GLFW_PRESS as i32 {
            $action();
        }
    }};
}

/// Create a window, run an init block, loop until the window closes
/// (calling tick! each frame), then run a cleanup block.
///
/// ```ignore
/// start_window_and_game_loop!(
///     "My Game", 1280, 720,
///     { /* init */ },
///     { /* each frame */ },
///     { /* cleanup */ }
/// );
/// ```
///
/// A shorter form uses defaults (title "Game", 1024×768):
/// ```ignore
/// start_window_and_game_loop!({ }, { }, { });
/// ```
#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $init:block, $frame:block, $cleanup:block) => {{
        $crate::ffi::safe_create_game_window($title, $width, $height);
        $init
        while !$crate::ffi::safe_window_should_close() {
            $frame
            $crate::tick!();
        }
        $cleanup
    }};
    ($init:block, $frame:block, $cleanup:block) => {{
        $crate::ffi::safe_create_game_window("Game", 1024, 768);
        $init
        while !$crate::ffi::safe_window_should_close() {
            $frame
            $crate::tick!();
        }
        $cleanup
    }};
}