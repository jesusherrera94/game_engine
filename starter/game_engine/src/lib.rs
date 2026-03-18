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

        let sprite = ffi::safe_create_sprite(100.0, 100.0, 50, 50, 255, 0, 0);

        while !ffi::safe_window_should_close() {
            ffi::safe_clear_screen();
            ffi::safe_render_sprite(sprite);
            ffi::safe_update_game_window();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
