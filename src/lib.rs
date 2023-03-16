#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num, plot_str};
use pc_keyboard::{DecodedKey, KeyCode};
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::RngCore;
use core::default::Default;
use core::panic;

const NEW_WALL_FREQ: isize = 100;
const NEW_BOMB_FREQ: isize = 20;

const WALLS: &str = "################################################################################
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
################################################################################";

pub struct Game {
    player: Player,
    walls: Items,
    food: Items,
    tick_count: isize,
    // Obtained from: https://stackoverflow.com/questions/67627335/how-do-i-use-the-rand-crate-without-the-standard-library
    rng: SmallRng,
}

impl Game {
    pub fn new() -> Self {
        let mut food = Items::default();
        food.change_color(Color::LightRed);
        Self {player: Player::new(), food, walls: Items::new(WALLS, Color::LightGreen), tick_count: 0, rng: SmallRng::seed_from_u64(3)}
    }
    pub fn reset(&mut self){
        let mut food = Items::default();
        food.change_color(Color::LightRed);
        Self {player: Player::new(), food, walls: Items::new(WALLS, Color::LightGreen), tick_count: 0, rng: SmallRng::seed_from_u64(3)};
    }


    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(key) => {
                let mut future = self.player;
                match key {
                    KeyCode::ArrowDown => {
                        future.down();
                    }
                    KeyCode::ArrowUp => {
                        future.up();
                    } 
                    KeyCode::ArrowLeft => {
                        future.left();
                    }
                    KeyCode::ArrowRight => {
                        future.right();
                    }
                    _ => {}
                }
                if !future.is_colliding(&self.walls) {
                    plot(' ', self.player.x, self.player.y, ColorCode::new(Color::Black, Color::Black));
                    self.player = future;
                }
                if future.is_colliding(&self.walls){
                    panic!("Game Over");
                }
                if self.player.is_colliding(&self.food) {
                    self.player.bomb_count += 1;
                    self.food.remove(self.player.y, self.player.x);
                    self.food.add_random_item(&mut self.rng)
                }
            }
            DecodedKey::Unicode(char) => {
                match char{
                    'r' =>{self.reset();}
                    _ => {}
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        // if self.tick_count % NEW_WALL_FREQ == 0 {
        //     self.walls.add_random_item(&mut self.rng);
        // }
        if self.tick_count == 5 {
            self.food.add_random_item(&mut self.rng);
        }
        // self.walls.draw();
        self.food.draw();
        plot('*', self.player.x, self.player.y, ColorCode::new(Color::Green, Color::Black));
        plot_num(self.tick_count, BUFFER_WIDTH / 2, 0, ColorCode::new(Color::LightGray, Color::Black));
        plot_str("Bombs:", 60, 0, ColorCode::new(Color::LightRed, Color::Black));
        plot_num(self.player.bomb_count as isize, 66, 0, ColorCode::new(Color::LightRed, Color::Black));
    }
}

pub struct Items {
    items: [[bool; BUFFER_WIDTH]; BUFFER_HEIGHT],
    color: Color,
}

impl Default for Items {
    fn default() -> Self {
        Self { items: [[false; BUFFER_WIDTH]; BUFFER_HEIGHT], color: Color::White}
    }
}

impl Items {
    pub fn new(map: &str, color: Color) -> Self {
        let mut walls = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (row, chars) in map.split('\n').enumerate() {
            for (col, value) in chars.char_indices() {
                walls[row][col] = value == '#';
            }
        }
        Self {items: walls, color}
    }

   
    

    pub fn add_random_item(&mut self, rng: &mut SmallRng) {
        let col: usize = 1 + rng.next_u32() as usize % (BUFFER_WIDTH - 1);
        let row: usize = 1 + rng.next_u32() as usize % (BUFFER_HEIGHT - 1);
        self.add(row, col);
    }

    pub fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn draw(&self) {
        for row in 0..self.items.len() {
            for col in 0..self.items[row].len() {
                if self.occupied(row, col) {
                    plot(self.char_at(row, col), col, row, ColorCode::new(self.color, Color::Black));
                }
            }
        }
    }

    pub fn occupied(&self, row: usize, col: usize) -> bool {
        self.items[row][col]
    }

    pub fn add(&mut self, row: usize, col: usize) {
        self.items[row][col] = true;
    }

    pub fn remove(&mut self, row: usize, col: usize) {
        self.items[row][col] = false;
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        if self.items[row][col] {
            '#'
        } else {
            ' '
        }
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
    bomb_count: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {x: BUFFER_WIDTH / 2, y: BUFFER_HEIGHT / 2, bomb_count: 0}
    }

    pub fn is_colliding(&self, walls: &Items) -> bool {
        walls.occupied(self.y, self.x)
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn up(&mut self) {
        self.y -= 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }
}
