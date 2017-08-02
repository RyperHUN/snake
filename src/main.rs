extern crate rand; //For random number generation
use rand::Rng;

extern crate cgmath;
use cgmath::Vector2;
use cgmath::vec2;

use std::collections::LinkedList;

const SIZE : usize = 10;	
type MapType = [[MapItem; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
											//Dynamic array: Vec<Vec<Type>>

type Vec2 = Vector2<i32>;
type List = LinkedList<PosDir>;
							

#[derive(Debug, Copy, Clone, PartialEq)]
enum MapItem {
	Wall,
	Food,
	Empty,
	SnakeHead,
	SnakePart,
}

#[derive(Debug, Copy, Clone)]
pub enum SnakeDir {
	Up,Left,Down, Right
}

pub fn increase_pos_by_dir (pos : &Vec2,dir : SnakeDir) -> Vec2 {
	let new_pos;
	match dir {
		SnakeDir::Up => new_pos    = pos + vec2(0,-1),
		SnakeDir::Down => new_pos  = pos + vec2(0,1),
		SnakeDir::Right => new_pos = pos + vec2(1,0),
		SnakeDir::Left => new_pos  = pos + vec2(-1,0),
	}
	return new_pos;
}

pub struct PosDir {
	pos : Vec2,
	dir : SnakeDir,
}

impl PosDir {
	pub fn new (pos : Vec2, dir : SnakeDir) -> PosDir {
		return PosDir {pos : pos,dir : dir};
	}
}

pub struct Snake {
	dir : SnakeDir,
	speed : u8, // 1-10 -> [500ms,100ms] refresh
	pos : Vec2, //Head pos
	tail : List,
}

impl Snake {
	pub fn new (speed : u8, pos : Vector2<i32>) -> Snake {
		return Snake {dir : SnakeDir::Up, 
		speed : speed, pos : pos, tail : List::new()};
	}
	pub fn convert_speed_to_ms (&self) -> u64 {
		match self.speed { //TODO better solution, and complete for all speed
		    1  => 500,
			10 => 100,
			_ => 250,
		}
	}
	pub fn grow_tail (&mut self, old_pos : Vec2, dir : SnakeDir) {
		if self.tail.is_empty () {
			self.tail.push_back(PosDir::new(old_pos,dir));
		}
	}
	pub fn move_tail (&mut self) {
		if self.tail.is_empty() {
			return;
		}
		let &mut front;
		match self.tail.front_mut() {
			Some(i) => front = i,
			None => return,
		}
		front.pos = increase_pos_by_dir (&front.pos, front.dir);
		front.dir = self.dir.clone();
		
		//TODO
		// for i in 1..self.tail.len() {
			
		// }
	}
}

 

											
pub struct Map {		
	array : MapType
}

impl Map {
	pub fn new () -> Map {
		let mut rng = rand::thread_rng();
		let mut map : MapType = [[MapItem::Empty; SIZE ]; SIZE];
		for i in 0..map.len() {
			for j in 0..map[i].len() {
				if i == 0 || j == 0  || i == SIZE - 1 || j == SIZE - 1 {
					map[i][j] = MapItem::Wall;
				}
				else {
					map[i][j] = MapItem::Empty;
				}
			}
		}
		
		let map_size = cgmath::vec2(1,  SIZE - 1);
		
		//Gen random pos for food
		let i = rng.gen_range(map_size.x, map_size.y); 
		let j = rng.gen_range(map_size.x, map_size.y);
		map[i][j] = MapItem::Food;
		
		
		return Map{array : map};
	}
	pub fn add_snake(&mut self) -> Snake {
		let map_size = vec2(1,  SIZE - 1);
		
		let center_pos = map_size / 2;
		self.array[center_pos.y][center_pos.y] = MapItem::SnakeHead;
		
		return Snake::new(1,vec2(center_pos.y as i32, center_pos.y as i32));
	}
	
	pub fn update_snake (&mut self, snake : &mut Snake) {
		let new_pos = increase_pos_by_dir (&snake.pos, snake.dir.clone());
		let old_pos = snake.pos.clone();
		
		//Save clones of the actual and next item
		let actual_item     =  self.array[snake.pos.y as usize][snake.pos.x as usize].clone();
		let mut next_item   =  self.array[new_pos.y as usize][new_pos.x as usize].clone();
		let mut is_grow = false;
		
		if next_item == MapItem::Wall {
			;//TODO Diee
		}
		if next_item == MapItem::Food {
			next_item = MapItem::Empty;
			is_grow = true;
			//TODO gen new food
		}
		if next_item == MapItem::Empty {
			//Update pos
			snake.pos = new_pos;
			snake.move_tail();
		}
		
		if is_grow {
			let dir = snake.dir.clone();
			snake.grow_tail (old_pos, dir);
			println!("Added snake tail at {}{}", old_pos.y, old_pos.x);
		}
		self.refresh_map(&snake);
	}
	pub fn refresh_map (&mut self, snake : &Snake) {
		let ref mut array = self.array; 
		for i in 0..array.len() { //TODO Refactor with lambda
			for j in 0..array[i].len() {
				if array[i][j] == MapItem::SnakeHead || array[i][j] == MapItem::SnakePart {
					array[i][j] = MapItem::Empty;
				}	
			}
		}
		array[snake.pos.y as usize][snake.pos.x as usize] = MapItem::SnakeHead;
		for elem in &snake.tail {
			array[elem.pos.y as usize][elem.pos.x as usize] = MapItem::SnakePart;
		}
	}
}

pub struct MapDrawer {
}

impl MapDrawer {
	pub fn draw_debug(map : &Map) {
		let ref array = map.array; 
		//let array = &self.array; //Same as the above line, just another syntax
		for i in 0..array.len() {
			for j in 0..array[i].len() {
				print!("[{}][{}]:{:?} | ", i,j, array[i][j]);
			}
			println!("");
		}
	}
	fn get_char (map_item : &MapItem) -> char {
		match *map_item {
			MapItem::Wall => return '#',
			MapItem::Food => return '0',
			MapItem::Empty => return ' ',
			MapItem::SnakeHead => return 'x',
			MapItem::SnakePart  => return '*',
			// _ => return '*'
		}
	}
	
	pub fn draw(map : &Map) {
		std::process::Command::new("clear").status().unwrap(); //TODO not the best solution for clearing the screen
		let ref array = map.array; 
		for i in 0..array.len() {
			for j in 0..array[i].len() {
				print!("{}", MapDrawer::get_char(&array[i][j]));
			}
			println!("");
		}
	}
}

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};


fn handle_input (input : &HashSet<sdl2::keyboard::Keycode>, prev_dir : SnakeDir) -> SnakeDir {
	if !input.is_empty() {
		if input.contains (&Keycode::W) {
			return SnakeDir::Up;
		}
		if input.contains (&Keycode::A) {
			return SnakeDir::Left;
		}
		if input.contains (&Keycode::D) {
			return SnakeDir::Right;
		}
		if input.contains (&Keycode::S) {
			return SnakeDir::Down;
		}
	}
	return prev_dir;
}

fn get_time_in_ms () -> u64 {
	let start = SystemTime::now();
	let since_the_epoch = start.duration_since(UNIX_EPOCH)
		.expect("Time went backwards");
	let in_ms = since_the_epoch.as_secs() * 1000 +
			since_the_epoch.subsec_nanos() as u64 / 1_000_000;
	return in_ms;
}

fn main() {
	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
	let mut events = sdl_context.event_pump().unwrap();

	//without window the keyboard handling is not working
	let _window = video_subsystem.window("Keyboard", 800, 600)
        .position_centered()
        .build()
        .unwrap();

	
	let mut snake_dir = SnakeDir::Up;
	let mut map = Map::new();
	let mut snake = map.add_snake();
	
	let ms_per_update = snake.convert_speed_to_ms ();
	
	MapDrawer::draw(&map);
	
	let mut last_frame_time = get_time_in_ms();
	let mut sum_elapsed_time : u64 = 0;
	let frame_per_sec_cap : u64 = 1000 / 60;
	
	let mut prev_keys = HashSet::new();
    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        }

        // Create a set of pressed Keys.
        let keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        // Get the difference between the new and old sets.
        let key_press = &keys - &prev_keys;
        let key_released = &prev_keys - &keys;
		snake_dir = handle_input(&keys, snake_dir);
		snake.dir = snake_dir.clone();

        if !key_press.is_empty() || !key_released.is_empty() {
            //println!("key_press: {:?}\t key_released:{:?}", key_press, key_released);
			println!("Snake dir: {:?}",snake.dir);
        }
		

        prev_keys = keys;
		
		let elapsed_time = get_time_in_ms() - last_frame_time;
		last_frame_time  = get_time_in_ms();
		sum_elapsed_time += elapsed_time;
		//println!("{:?}", sum_elapsed_time);
		if sum_elapsed_time > ms_per_update {
			sum_elapsed_time -= ms_per_update;
			map.update_snake (&mut snake);
			MapDrawer::draw(&map);
		}

		//TODO sleep for 60 fps
		let mut sleep_time = 0;
		if elapsed_time < frame_per_sec_cap {
			sleep_time = frame_per_sec_cap - elapsed_time;
		}
        std::thread::sleep(Duration::from_millis(sleep_time));
    }
}
