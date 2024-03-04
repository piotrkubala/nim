extern crate sdl2;

use std::error::Error;

mod game;

pub fn main() -> Result<(), Box<dyn Error>> {
    let game_settings = game::system::GameSettings {
        window_width: 800,
        window_height: 600,
        microseconds_per_frame: 1_000_000 / 60,
        microseconds_per_ai_move: 1_000_000 / 2, 
        heaps_count: 25,
        max_stones_per_heap: 40,
    };
    let mut game = game::system::Game::new(game_settings)?;

    game.run()?;

    Ok(())
}