use std::cmp::min;
use sdl2::mouse;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct NimHeap {
    size: u32,
    count: u32,
    corner_x: i32,
    corner_y: i32,
    stone_width: u32,
    stone_height: u32
}

impl NimHeap {
    pub fn new(size: u32, count: u32) -> NimHeap {
        NimHeap {
            size,
            count: min(size, count),
            corner_x: 0,
            corner_y: 0,
            stone_width: 1,
            stone_height: 1,
        }
    }

    fn get_nth_stone_rect(&self, n: usize) -> Rect {
        let x = self.corner_x;
        let y = self.corner_y + (n as u32 * self.stone_height) as i32;

        Rect::new(x, y, self.stone_width, self.stone_height)
    }

    fn set_heap_sizes(&mut self, area_rectangle: Rect, stone_height: f64) {
        self.corner_x = area_rectangle.x();
        self.corner_y = area_rectangle.y();
        self.stone_width = area_rectangle.width();
        self.stone_height = (area_rectangle.height() as f64 / stone_height) as u32;
    }

    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let colour_not_hovered = Color::RGB(100, 100, 100);
        let colour_hovered = Color::RGB(200, 100, 100);

        let mut colour = colour_not_hovered;

        for i in 0..self.count {

        }

        Ok(())
    }
}

impl Clone for NimHeap {
    fn clone(&self) -> NimHeap {
        NimHeap {
            size: self.size,
            count: self.count,
            corner_x: self.corner_x,
            corner_y: self.corner_y,
            stone_width: self.stone_width,
            stone_height: self.stone_height,
        }
    }
}

pub enum Player {
    One,
    Two,
}

pub struct NimMove {
    heap_index: usize,
    count_to_remove: u32,
}

pub struct NimGame {
    heaps: Vec<NimHeap>,
    player: Player,
    default_heap: NimHeap,
}

impl NimGame {
    pub fn new(default_heap: NimHeap) -> NimGame {
        NimGame {
            heaps: Vec::new(),
            player: Player::One,
            default_heap
        }
    }

    pub fn add_heap(&mut self, heap: NimHeap) {
        self.heaps.push(heap);
    }

    pub fn add_default_heap(&mut self) {
        self.heaps.push(self.default_heap.clone());
    }

    pub fn remove_last_heap(&mut self) {
        self.heaps.pop();
    }

    pub fn make_move(&mut self, nim_move: NimMove) -> bool {
        if nim_move.heap_index >= self.heaps.len() || nim_move.count_to_remove < 1 {
            return false;
        }

        let heap = &mut self.heaps[nim_move.heap_index];

        if heap.count < nim_move.count_to_remove {
            return false;
        }

        heap.count -= nim_move.count_to_remove;

        true
    }

    pub fn is_game_over(&self) -> bool {
        self.heaps.iter().all(|heap| heap.count == 0)
    }

    pub fn draw_board(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let margin_top = 100;

        let window_size = canvas.output_size()?;
        let game_area_width = window_size.0 as f64 * 0.9;
        let game_area_height = (window_size.1 - margin_top) as f64 * 0.9;

        let margin_x = (window_size.0 as f64 - game_area_width) / 2.0;
        let margin_between_heaps = 10.0;

        let half_margin_between_heaps = margin_between_heaps * 0.5;

        let count_of_stones = self.heaps.iter()
            .map(|heap| heap.count as usize).max().unwrap_or(1);

        let heap_width_with_margin = game_area_width / self.heaps.len() as f64 - margin_between_heaps;
        let heap_height = game_area_height;

        let stone_height = heap_height / count_of_stones as f64;

        for (i, heap) in self.heaps.iter_mut().enumerate() {
            let x = i as f64 * (heap_width_with_margin + half_margin_between_heaps)
                + margin_x + half_margin_between_heaps;
            let y = margin_top as f64 + game_area_height - heap_height;

            let colour = Color::RGB(0, 0, 0);
            let rectangle =
                Rect::new(x as i32, y as i32, heap_width_with_margin as u32, heap_height as u32);

            canvas.set_draw_color(colour);
            canvas.draw_rect(rectangle)?;

            heap.set_heap_sizes(rectangle, stone_height);
            heap.draw(canvas)?;
        }

        Ok(())
    }
}