use std::sync::mpsc::{Receiver, Sender};

use crate::sprite_data::SpriteData;

pub enum NetworkRequest {
    FetchSprite,
    Quit,
}

pub fn networking_thread(receiver: Receiver<NetworkRequest>, sender: Sender<SpriteData>) {
    loop {
        match receiver.recv() {
            Ok(NetworkRequest::FetchSprite) => match fetch_sprite_data() {
                Ok(sprite_data) => {
                    if sender.send(sprite_data).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Failed to fetch sprite data: {e}");
                }
            },
            Ok(NetworkRequest::Quit) | Err(_) => {
                break;
            }
        }
    }
}

fn fetch_sprite_data() -> Result<SpriteData, Box<dyn std::error::Error>> {
    let api_url = std::env::var("SPRITE_API_URL").unwrap_or_else(|_| {
        "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler".to_string()
    });
    let sprite_data: SpriteData = reqwest::blocking::get(&api_url)?.json()?;
    Ok(sprite_data)
}
