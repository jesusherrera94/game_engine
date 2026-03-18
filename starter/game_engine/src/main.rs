
fn main() {
    game_engine::ffi::safe_create_game_window("Test Window", 800, 600);
    let sprite = game_engine::ffi::safe_create_sprite(100.0, 100.0, 50, 50, 255, 0, 0);
    while !game_engine::ffi::safe_window_should_close() {
        game_engine::ffi::safe_clear_screen();
        game_engine::ffi::safe_render_sprite(sprite);
        game_engine::ffi::safe_update_game_window();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}