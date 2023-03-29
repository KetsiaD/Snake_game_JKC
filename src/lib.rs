#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]


use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable, plot_num, plot_str};
use pc_keyboard::{DecodedKey, KeyCode};
use rand::{Rng, SeedableRng};
use rand::RngCore;
use rand::rngs::SmallRng;
use core::default::Default;
use core::clone::Clone;
use core::marker::Copy;
use core::iter::Iterator;




pub struct Game {
    player: Player,
    food: Food,
    grid: Refresh,
    tick_count: isize,
    running : bool,
    score: usize,
}


impl Game {
    pub fn new() -> Self {
        Self {player: Player::new(1), food: Food::new(1), grid: Refresh::new(), tick_count: 0, running: true, score: 0}
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(key) => {
                match key {
                    KeyCode::ArrowDown => {
                        if self.player.direction != 'u'{
                            self.player.down()
                        }
                        
                    }
                    KeyCode::ArrowUp => {
                        if self.player.direction != 'd'{self.player.up()}
                        
                    } 
                    KeyCode::ArrowLeft => {
                        if self.player.direction != 'r'{self.player.left()}
                        
                    }
                    KeyCode::ArrowRight => {
                        if self.player.direction != 'l'{self.player.right()}
                        
                    }
                    
                    _ => {}
                }     
            }
            DecodedKey::Unicode(key) => {
                match key { 
                    'r' => {
                        if !self.running{
                            self.reset()
                        }
                    }
                    _ => {}
                } 
            }
        }
    }
    pub fn reset(&mut self) {
        self.player = Player::new(1);
        self.food = Food::new(1);
        self.grid = Refresh::new(); 
        self.tick_count = 0; 
        self.score = 0;
        self.running = true;
    }
    
    pub fn tick(&mut self) {
        if self.running {
            self.grid.draw();

            if self.food.total_food < self.food.max_food{
                self.food.add_food();
            }
            for col in 0..BUFFER_WIDTH-1{
                for row in 0..BUFFER_HEIGHT-1{
                    
                    if self.food.food_map[col][row]{
                        plot('*', col, row, ColorCode::new(self.food.color, Color::Black));
                    }
                }
            }
            self.player.update_location(self.tick_count);
            
            

            plot('▧', self.player.x, self.player.y, ColorCode::new(Color::Green, Color::Black));
            for i in 0..self.player.food_ate+1{
                
                plot('▧', self.player.body[i].x, self.player.body[i].y, ColorCode::new(Color::Green, Color::Black));
            }
            
            
            if self.food.food_map[self.player.x][self.player.y]{
                self.food.food_map[self.player.x][self.player.y] = false;
                self.food.add_food();
                self.player.eat();
                self.score += 1
            }
    
           
            if self.player.has_moved {
               
               // Find a way to add collision with self. I think it should be if self.player.check_collision(self.player) || self.player.edge 
                if  self.player.edge{
                    self.running = false;
                    plot_str("GAME OVER:(", 27, 10, ColorCode::new(Color::LightRed, Color::Black));

                }else if self.player.check_collision_self(){
                    self.running = false;
                    plot_str("GAME OVER:(", 27, 10, ColorCode::new(Color::LightRed, Color::Black));
                }
            }
            self.tick_count += 1

        } else {
            
            plot_str("PRESS R TO RESTART THE GAME", 22, 15, ColorCode::new(Color::LightRed, Color::Black));
        }
        plot_str("SCORE: ", 28, 0, ColorCode::new(Color::LightCyan, Color::Black));
        plot_num(self.score as isize, 35, 0, ColorCode::new(Color::LightGray, Color::Black));

        
        self.tick_count += 1
    }
    
    
}

pub struct Refresh {
    grid : [[bool; BUFFER_WIDTH]; BUFFER_HEIGHT],
    color: Color
}

impl Refresh {
    pub fn new() -> Self {
        let mut grid = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];
        
        Self {grid, color: Color::Black}
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        if self.grid[row][col] {
            '#'
        } else {
            ' '
        }
    }
    fn draw(&self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len(){
                plot(self.char_at(row, col,), col, row, ColorCode::new(Color::Red, Color::Black));
            }
        }
    }
}

pub struct Duple {
    x: usize,
    y: usize,
}
impl Copy for Duple {}
impl Clone for Duple{
    fn clone(&self)-> Duple {
        *self
    }

    fn clone_from(&mut self, source: &Self)
    where
        Self: ~const core::marker::Destruct,
    {
        *self = source.clone()
    }
}
impl Default for Duple{
    fn default() -> Self {
        Self{x:0, y: 0}
    }
}
impl Duple{
    pub fn new(xt: usize, yt: usize) -> Self{
        Self{x: xt, y: yt}
    }
}
#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
    direction: char,
    food_ate: usize,
    body: [Duple; 8000],
    has_moved: bool,
    edge: bool,
}

impl Player {
    pub fn new(state : u64) -> Self {
        let mut small_rng = SmallRng::seed_from_u64(state);
        let x = small_rng.next_u64() as usize % BUFFER_WIDTH ; 
        let y = small_rng.next_u64() as usize % BUFFER_HEIGHT;
        let  body: [Duple; 8000] = [Duple::new(0, 0); 8000];

        Self {x, y , food_ate: 0, body, has_moved: false, direction: 'n', edge: false}
        
    }
    
    pub fn eat(&mut self){
        self.food_ate +=1;
    }

    pub fn check_collisions(&mut self, op: Food) -> bool{
        op.occupied(self.y, self.x)
    }
    

    pub fn check_collision_self(&self) -> bool {
        for i in 1..self.food_ate+1 { 
            if self.body[i].x == self.body[0].x && self.body[i].y == self.body[0].y {
                return true;
            }
        }
        false
    }
    

    pub fn down(&mut self) {
        self.has_moved = true;
        if self.y + 1 < BUFFER_HEIGHT {
            self.direction = 'd';
            self.y += 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
            
        }
        else{
            self.edge = true;
        }
    }

    pub fn up(&mut self) {
        self.has_moved = true;
        if self.y > 1 {
            self.direction = 'u';
            self.y -= 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
        else{
            self.edge = true;
        }
    }   

    pub fn left(&mut self) {
        self.has_moved = true;
        if self.x > 0 {
            self.direction = 'l';
            self.x -= 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let mut tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
        else{
            self.edge = true;
        }
    }

    pub fn right(&mut self) {
        self.has_moved = true;
        if self.x + 1 < BUFFER_WIDTH {
            self.direction = 'r';
            self.x += 1;
            let mut temp: &Duple = &Duple::new(0,0);
            let mut temp2: &Duple = &Duple::new(0,0);

            let tempbod = self.body.clone();
            for (spot, dup) in tempbod.iter().enumerate(){
                if spot==0{
                    temp = dup;
                    self.body[0] = Duple::new(self.x, self.y).clone();
                }
                else{
                    temp2 = dup;
                    self.body[spot] = *temp;
                    temp = temp2;
                }
            }
        }
        else{
            self.edge = true;
        }
    }

    fn update_location(&mut self, tick_count : isize) {
        if tick_count % 3 == 0 {
            if self.direction == 'r' {
                self.right();
            } else if self.direction == 'l' {
                self.left();
            } else if self.direction == 'd' {
                self.down()
            }  else if  self.direction == 'u' {
                self.up();
            }  
       }
        
    }
    
    
}


pub struct Food{
    food_map: [[bool; BUFFER_HEIGHT]; BUFFER_WIDTH],
    color: Color,
    total_food: usize,
    max_food: usize,
    rng: SmallRng,
}

impl Food {
    pub fn new(max: usize) -> Self{
        let temp = [[false; BUFFER_HEIGHT];BUFFER_WIDTH];
        let c = Color::White;
        Self {food_map: temp, color: c, total_food: 0, max_food: max, rng: SmallRng::seed_from_u64(1)}
        
    }

    pub fn occupied(&self, row: usize, col: usize) -> bool {
        self.food_map[row][col]
    }

    pub fn add_food(&mut self){
        let col: usize = 1+ self.rng.next_u32() as usize % (BUFFER_WIDTH - 1);
        let row: usize = 1 + self.rng.next_u32() as usize % (BUFFER_HEIGHT -1);
        while true{
            if !self.food_map[col][row]{
                self.food_map[col][row] = true;
                self.total_food +=1;
                break;
            }
        }
    }
}
