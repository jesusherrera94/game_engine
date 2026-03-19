
use game_engine::{start_window_and_game_loop, spawn_sprite, on_key_press, move_sprite};

fn main() {
    start_window_and_game_loop!(
        "Sprite Demo", 400, 400,
        {
            // init: nothing extra needed here
        },
        {
            // each frame
            game_engine::ffi::safe_clear_screen();

            let sprite = spawn_sprite!(50.0, 50.0, 50, 50, 255, 0, 0);

            let window = game_engine::ffi::safe_get_window();
            on_key_press!(window, game_engine::ffi::GLFW_KEY_RIGHT, || {
                move_sprite!(sprite, 1.0, 0.0, true);
            });
        },
        {
            println!("Window closed.");
        }
    );
}