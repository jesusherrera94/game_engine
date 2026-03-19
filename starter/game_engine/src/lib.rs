pub mod ffi;
#[macro_use]
pub mod r#macro;

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_simple_game_loop() {
        // Create a game window
        start_window_and_game_loop!("Open blank window", 400, 400, {}, {}, {
            println!("Window closed.");
        });
    }

    #[ignore]
    #[test]
    fn test_sprite_rendering() {
        start_window_and_game_loop!(
            "Open window /w sprite",
            400,
            400,
            {},
            {
                game_engine::ffi::safe_clear_screen();
                let sprite = spawn_sprite!(50.0, 50.0, 50, 50, 255, 0, 0);
                let window = game_engine::ffi::safe_get_window();
            },
            {
                println!("Window closed.");
            }
        );
    }

    #[ignore]
    #[test]
    fn test_screen_clearing() {
        let mut red_sprite = std::ptr::null_mut();
        let mut green_sprite = std::ptr::null_mut();
        let start_time = std::time::Instant::now();

        start_window_and_game_loop!(
            "Open window /w screen clearing",
            400,
            400,
            {
                red_sprite = spawn_sprite!(50.0, 50.0, 50, 50, 255, 0, 0);
                green_sprite = spawn_sprite!(50.0, 50.0, 50, 50, 0, 255, 0);
            },
            {
                game_engine::ffi::safe_clear_screen();
                if start_time.elapsed().as_secs() < 5 {
                    game_engine::ffi::safe_render_sprite(red_sprite);
                } else {
                    game_engine::ffi::safe_render_sprite(green_sprite);
                }
            },
            {
                println!("Window closed.");
            }
        );
    }

    #[ignore]
    #[test]
    fn test_key_presses() {
        let mut up_key_pressed = false;
        let mut down_key_pressed = false;
        start_window_and_game_loop!(
            "Key events window",
            400,
            400,
            {},
            {
                // each frame
                game_engine::ffi::safe_clear_screen();

                let window = game_engine::ffi::safe_get_window();
                on_key_press!(window, game_engine::ffi::GLFW_KEY_UP, || {
                    println!("Up!");
                    up_key_pressed = true;
                });
                on_key_press!(window, game_engine::ffi::GLFW_KEY_DOWN, || {
                    println!("Down!");
                    down_key_pressed = true;
                });
                if up_key_pressed && down_key_pressed {
                    break;
                }
            },
            {
                println!("Window closed.");
            }
        );
    }

    #[ignore]
    #[test]
    fn test_sprite_position_update() {
        let mut red_sprite = std::ptr::null_mut();
        let mut y_start = 50.0;
        start_window_and_game_loop!(
            "Open window /w moving sprite",
            400,
            400,
            {
                red_sprite = spawn_sprite!(50.0, 50.0, 50, 50, 255, 0, 0);
            },
            {
                game_engine::ffi::safe_clear_screen();
                move_sprite!(red_sprite, 100.0, y_start);
                y_start += 1.0;
            },
            {
                println!("Window closed.");
            }
        );
    }
}
