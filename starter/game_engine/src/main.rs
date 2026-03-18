
fn main() {
        game_engine::ffi::safe_create_game_window("Sprite Position Update Test", 400, 400);
        let sprite = game_engine::ffi::safe_create_sprite(50.0, 50.0, 50, 50, 255, 0, 0);
        let mut x = 50.0;
        while !game_engine::ffi::safe_window_should_close() {
            game_engine::ffi::safe_clear_screen();
            game_engine::ffi::safe_render_sprite(sprite);
            game_engine::ffi::safe_update_game_window();
            x += 1.0;
            game_engine::ffi::safe_update_sprite_position(sprite, x, 50.0);
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
}