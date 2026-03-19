use game_engine::{move_sprite, on_key_press, spawn_sprite, start_window_and_game_loop};

fn main() {
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
