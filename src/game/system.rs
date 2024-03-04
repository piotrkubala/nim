use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use std::time::{Duration, Instant};
use super::{NimGame, NimHeap, NimMove, Player};

enum GameEvent {
    Quit,
    Other(Event)
}

pub struct GameSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub microseconds_per_frame: u64
}

pub struct Game {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    settings: GameSettings,
    nim_game: NimGame,
}

impl Game {
    pub fn new(settings: GameSettings) -> Result<Game, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("Nim - the game", 800, 600)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        
        let default_heap = NimHeap::new(15, 10);
        let mut nim_game = NimGame::new(default_heap);

        nim_game.add_default_heap();
        nim_game.add_default_heap();
        nim_game.add_default_heap();

        Ok(Game {
            sdl_context,
            canvas,
            settings,
            nim_game
        })
    }
    
    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        
        'running: loop {
            let start_time = Instant::now();
            
            for event in event_pump.poll_iter() {
                match self.handle_event(event) {
                    GameEvent::Quit => break 'running,
                    GameEvent::Other(_) => {}
                }
            }
            
            self.draw_frame()?;
            self.wait_to_next_frame(start_time);
        }
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: Event) -> GameEvent {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => GameEvent::Quit,
            _ => GameEvent::Other(event)
        }
    }
    
    fn draw_frame(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(255, 0, 255));
        self.canvas.clear();
        self.nim_game.draw_board(&mut self.canvas)?;
        
        self.canvas.present();
        
        Ok(())
    }
    
    fn wait_to_next_frame(&self, start_time: Instant) {
        let elapsed_time = start_time.elapsed();
        let elapsed_micros = elapsed_time.as_micros() as i64;
        let remaining_micros = self.settings.microseconds_per_frame as i64 - elapsed_micros;
        
        if remaining_micros > 0 {
            let remaining_duration = Duration::from_micros(remaining_micros as u64);
            ::std::thread::sleep(remaining_duration);
        }
    }
}