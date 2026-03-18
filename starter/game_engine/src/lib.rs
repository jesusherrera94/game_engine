pub mod ffi;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_simple_game_loop() {
        // Create a game window
        ffi::safe_create_game_window("Test Window", 800, 600);

        while !ffi::safe_window_should_close() {
            ffi::safe_clear_screen();
            ffi::safe_update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[ignore]
    #[test]
    fn test_sprite_rendering() {
        ffi::safe_create_game_window("Sprite Test", 400, 400);
        let sprite = ffi::safe_create_sprite(150.0, 150.0, 100, 100, 0, 255, 0);

        while !ffi::safe_window_should_close() {
            ffi::safe_clear_screen();
            ffi::safe_render_sprite(sprite);
            ffi::safe_update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[ignore]
    #[test]
    fn test_screen_clearing() {
        ffi::safe_create_game_window("Clear Test", 400, 400);
        let red_sprite = ffi::safe_create_sprite(150.0, 150.0, 100, 100, 255, 0, 0);
        let green_sprite = ffi::safe_create_sprite(200.0, 200.0, 100, 100, 0, 255, 0);
        let mut should_render_red_sprite = true;
        while !ffi::safe_window_should_close() {
            ffi::safe_clear_screen();
            if should_render_red_sprite {
                ffi::safe_render_sprite(red_sprite);
                should_render_red_sprite = false;
                ffi::safe_update_game_window();
                std::thread::sleep(std::time::Duration::from_secs(5));
            } else {
                ffi::safe_render_sprite(green_sprite);
                ffi::safe_update_game_window();
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
    }

    #[ignore]
    #[test]
    fn test_key_presses() {
        ffi::safe_create_game_window("Key Press Test", 400, 400);
        let window = ffi::safe_get_window();
        let mut up_key_pressed = false;
        let mut down_key_pressed = false;
        while ! up_key_pressed || ! down_key_pressed {
             if ffi::safe_get_key(window, ffi::GLFW_KEY_UP) == 1 { // Up key
                println!("Up key pressed!");
                up_key_pressed = true;
            }
            if ffi::safe_get_key(window, ffi::GLFW_KEY_DOWN) == 1 { // Down key
                println!("Down key pressed!");
                down_key_pressed = true;
            }
            ffi::safe_clear_screen();
            ffi::safe_update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    #[ignore]
    #[test]
    fn test_sprite_position_update() {
        ffi::safe_create_game_window("Sprite Position Update Test", 400, 400);
        let sprite = ffi::safe_create_sprite(50.0, 50.0, 50, 50, 255, 0, 0);
        let mut x = 50.0;
        while !ffi::safe_window_should_close() {
            ffi::safe_clear_screen();
            ffi::safe_render_sprite(sprite);
            ffi::safe_update_game_window();
            x += 1.0;
            ffi::safe_update_sprite_position(sprite, x, 50.0);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

}
