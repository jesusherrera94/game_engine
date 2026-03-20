mod networking;
mod sprite_data;

use game_engine::{on_key_press, spawn_sprite, start_window_and_game_loop};
use networking::NetworkRequest;
use std::sync::mpsc;

fn main() {
    // Channel: main thread -> networking thread
    let (main_sender, net_receiver) = mpsc::channel::<NetworkRequest>();
    // Channel: networking thread -> main thread
    let (net_sender, main_receiver) = mpsc::channel::<sprite_data::SpriteData>();

    let network_handle = std::thread::spawn(move || {
        networking::networking_thread(net_receiver, net_sender);
    });

    let mut sprites: Vec<*mut game_engine::ffi::Sprite> = Vec::new();
    let mut was_space_down = false;

    start_window_and_game_loop!(
        "Sprite Game",
        800,
        600,
        {},
        {
            game_engine::ffi::safe_clear_screen();

            let window = game_engine::ffi::safe_get_window();
            let space_down =
                game_engine::ffi::safe_get_key(window, game_engine::ffi::GLFW_KEY_SPACE)
                    == game_engine::ffi::GLFW_PRESS;
            /* This condition is for detecting the space key press event 
                And avoiding saturate the network thread with requests while the space key is held down
            */         
            if space_down && !was_space_down {
                on_key_press!(window, game_engine::ffi::GLFW_KEY_SPACE, || {
                    println!("Space key pressed! Fetching new sprite data...");
                    let _ = main_sender.send(NetworkRequest::FetchSprite);
                });
            }
            was_space_down = space_down;

            
            while let Ok(sprite_data) = main_receiver.try_recv() {
                let sprite = spawn_sprite!(
                    sprite_data.x,
                    sprite_data.y,
                    sprite_data.width,
                    sprite_data.height,
                    sprite_data.r,
                    sprite_data.g,
                    sprite_data.b
                );
                sprites.push(sprite);
            }

            // Re-render all persisted sprites each frame
            for &sprite in &sprites {
                game_engine::ffi::safe_render_sprite(sprite);
            }
        },
        {
            let _ = main_sender.send(NetworkRequest::Quit);
            let _ = network_handle.join();
            println!("Window closed.");
        }
    );
}
