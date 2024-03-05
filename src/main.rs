extern crate sdl2;

use std::error::Error;

mod game;

pub fn main() -> Result<(), Box<dyn Error>> {
    let game_settings = game::system::GameSettings {
        window_width: 1200,
        window_height: 800,
        microseconds_per_frame: 1_000_000 / 60,
        microseconds_per_ai_move: 1_000_000 / 2, 
        heaps_count: 25,
        max_stones_per_heap: 40,
        target_colour_change_time: std::time::Duration::from_millis(500)
    };
    let mut game = game::system::Game::new(game_settings)?;

    game.run()?;

    Ok(())
}