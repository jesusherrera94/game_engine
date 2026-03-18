fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filter = args.iter().skip(1).find(|a| !a.starts_with('-'));

    match filter.map(|s| s.as_str()) {
        Some(f) if f.contains("test_simple_game_loop") => test_simple_game_loop(),
        // Some(f) if f.contains("test_sprite_rendering") => test_sprite_rendering(),
        // Some(f) if f.contains("test_screen_clearing") => test_screen_clearing(),
        // Some(f) if f.contains("test_key_presses") => test_key_presses(),
        // Some(f) if f.contains("test_sprite_position_update") => test_sprite_position_update(),
        Some(f) => eprintln!("Unknown test: {}", f),
        None => {
            test_simple_game_loop();
        }
    }
}

fn test_simple_game_loop() {
    game_engine::ffi::safe_create_game_window("Test Window", 800, 600);
    let sprite = game_engine::ffi::safe_create_sprite(100.0, 100.0, 50, 50, 255, 0, 0);
    while !game_engine::ffi::safe_window_should_close() {
        game_engine::ffi::safe_clear_screen();
        game_engine::ffi::safe_render_sprite(sprite);
        game_engine::ffi::safe_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}