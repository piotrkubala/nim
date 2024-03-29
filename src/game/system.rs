use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

use std::time::{Duration, Instant};
use sdl2::rect::Point;
use super::{NimGame, NimHeap};

enum GameEvent {
    Quit,
    Other(Event)
}

pub struct GameSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub microseconds_per_frame: u64,
    pub microseconds_per_ai_move: u64,
    pub heaps_count: u32,
    pub max_stones_per_heap: u32,
    pub target_colour_change_time: Duration
}

pub struct MouseState {
    pub point: Point,
    pub left_button: bool,
    pub right_button: bool
}

impl Clone for MouseState {
    fn clone(&self) -> MouseState {
        MouseState {
            point: self.point,
            left_button: self.left_button,
            right_button: self.right_button
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Player::One => write!(f, "Player 1"),
            Player::Two => write!(f, "Player 2"),
        }
    }
}

pub enum PlayerType {
    Human,
    Computer
}

impl Display for PlayerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PlayerType::Human => write!(f, "Human"),
            PlayerType::Computer => write!(f, "Computer"),
        }
    }
}

pub struct Game {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    settings: GameSettings,
    nim_game: NimGame,
    previous_mouse_state: MouseState,
    current_mouse_state: MouseState,
    players: HashMap<Player, PlayerType>,
    last_human_move_time: Option<Instant>,
    last_frame_time: Instant,
    background_colour: Color
}

impl Game {
    pub fn new(settings: GameSettings) -> Result<Game, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window(
            "Nim - the game",
            settings.window_width,
            settings.window_height
        ).position_centered().build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
        
        let heaps_count = settings.heaps_count;
        let max_stones_per_heap = settings.max_stones_per_heap;

        let default_heap = NimHeap::new(max_stones_per_heap, 10);
        let mut nim_game = NimGame::new(default_heap);

        for _ in 0..heaps_count {
            nim_game.add_random_heap();
        }
        
        let players =
            vec![(Player::One, PlayerType::Human), (Player::Two, PlayerType::Computer)]
            .into_iter()
            .collect::<HashMap<Player, PlayerType>>();
        
        let current_mouse_state = MouseState {
            point: Point::new(0, 0),
            left_button: false,
            right_button: false
        };
        
        let previous_mouse_state = current_mouse_state.clone();

        Ok(Game {
            sdl_context,
            canvas,
            settings,
            nim_game,
            previous_mouse_state,
            current_mouse_state,
            players,
            last_human_move_time: None,
            last_frame_time: Instant::now(),
            background_colour: Color::RGB(0, 0, 155)
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
        
            self.handle_ai_players();
            self.draw_frame()?;
            
            if self.handle_game_ending() {
                break 'running;
            }
            
            self.wait_to_next_frame(start_time);
        }

        Ok(())
    }
    
    fn handle_game_ending(&mut self) -> bool {
        if self.nim_game.is_game_over() {
            let player_to_move = self.nim_game.get_player_to_move();
            let winner = player_to_move.next();
            
            println!("Game over!");
            println!("{} wins!", winner);

            if let Some(winner_type) = self.players.get(&winner) {
                println!("This player is a {}", winner_type);
            }
            
            return true;
        }
        
        false
    }
    
    fn handle_ai_players(&mut self) {
        if let Some(last_human_move_time) = self.last_human_move_time {
            let elapsed_time = last_human_move_time.elapsed();
            let elapsed_micros = elapsed_time.as_micros() as u64;
            
            if elapsed_micros >= self.settings.microseconds_per_ai_move {
                self.handle_ai_move();
            }
        }
    }
    
    fn handle_player_move(&mut self) {
        let player_to_move = self.nim_game.get_player_to_move();
        
        if let Some(PlayerType::Human) = self.players.get(player_to_move) {
            let point = self.current_mouse_state.point;
            let nim_move_option = self.nim_game.prepare_player_move(point);
            
            if let Some(nim_move) = nim_move_option {
                self.nim_game.make_move(nim_move);
                self.last_human_move_time = Some(Instant::now());
            }
        }
    }
    
    fn handle_ai_move(&mut self) {
        let player_to_move = self.nim_game.get_player_to_move();
        
        if let Some(PlayerType::Computer) = self.players.get(player_to_move) {
            let nim_move_option = self.nim_game.prepare_ai_move();
            
            if let Some(nim_move) = nim_move_option {
                self.nim_game.make_move(nim_move);
                self.last_human_move_time = None;
            }
        }
    }
    
    fn move_mouse_states(&mut self) {
        self.previous_mouse_state = self.current_mouse_state.clone();
    }
    
    fn handle_potential_mouse_moved(&mut self, event: &Event) {
        if let Event::MouseMotion { x, y, .. } = event {
            self.current_mouse_state.point = Point::new(*x, *y);
        }
    }
    
    fn handle_left_click_up(&mut self) {
        self.handle_player_move();
    }
    
    fn handle_potential_mouse_button(&mut self, event: &Event) {
        match event {
            Event::MouseButtonDown {..} | Event::MouseButtonUp {..} => {
                self.move_mouse_states();
                
                match event {
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => self.current_mouse_state.left_button = true,
                            sdl2::mouse::MouseButton::Right => self.current_mouse_state.right_button = true,
                            _ => {}
                        }
                    },
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        match mouse_btn {
                            sdl2::mouse::MouseButton::Left => {
                                self.current_mouse_state.left_button = false;
                                self.handle_left_click_up();
                            },
                            sdl2::mouse::MouseButton::Right => self.current_mouse_state.right_button = false,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    fn handle_event(&mut self, event: Event) -> GameEvent {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => GameEvent::Quit,
            _ => {
                self.handle_potential_mouse_moved(&event);
                self.handle_potential_mouse_button(&event);
                
                GameEvent::Other(event)
            }
        }
    }
    
    fn draw_background(&mut self) {
        fn subtract_colour(colour1: Color, colour2: Color) -> (f64, f64, f64) {
            let r1 = colour1.r as f64;
            let g1 = colour1.g as f64;
            let b1 = colour1.b as f64;
            
            let r2 = colour2.r as f64;
            let g2 = colour2.g as f64;
            let b2 = colour2.b as f64;
            
            (r1 - r2, g1 - g2, b1 - b2)
        }
        
        fn multiply_colour((r, g, b): (f64, f64, f64), factor: f64) -> (f64, f64, f64) {
            (r * factor, g * factor, b * factor)
        }
        
        fn add_colour(colour1: Color, (r, g, b): (f64, f64, f64)) -> Color {
            let r = (colour1.r as f64 + r) as u8;
            let g = (colour1.g as f64 + g) as u8;
            let b = (colour1.b as f64 + b) as u8;
            
            Color::RGB(r, g, b)
        }
        
        let first_player_background_colour = Color::RGB(100, 155, 0);
        let second_player_background_colour = Color::RGB(155, 100, 0);
        let current_player = self.nim_game.get_player_to_move();
        
        let time_since_last_frame = self.last_frame_time.elapsed();
        let target_colour_change_time = self.settings.target_colour_change_time;
        
        let time_ratio = time_since_last_frame.as_secs_f64() / target_colour_change_time.as_secs_f64();
                
        self.background_colour =
            add_colour(
                self.background_colour,
                multiply_colour(
                    subtract_colour(
                        match current_player {
                            Player::One => first_player_background_colour,
                            Player::Two => second_player_background_colour
                        },
                        self.background_colour
                    ),
                    time_ratio
                )
            );
        
        self.canvas.set_draw_color(self.background_colour);
        self.canvas.clear();
    }

    fn draw_frame(&mut self) -> Result<(), String> {
        self.draw_background();
        self.nim_game.draw_board(&mut self.canvas, &self.current_mouse_state)?;

        self.canvas.present();

        Ok(())
    }

    fn wait_to_next_frame(&mut self, start_time: Instant) {
        self.last_frame_time = Instant::now();
        
        let elapsed_time = start_time.elapsed();
        let elapsed_micros = elapsed_time.as_micros() as i64;
        let remaining_micros = self.settings.microseconds_per_frame as i64 - elapsed_micros;

        if remaining_micros > 0 {
            let remaining_duration = Duration::from_micros(remaining_micros as u64);
            ::std::thread::sleep(remaining_duration);
        }
    }
}