extern crate rand; //For random number generation
use rand::Rng;

//RUST mutability guide
// Rust          C/C++
    // a: &T     == const T* const a; // can't mutate either
// mut a: &T     == const T* a;       // can't mutate what is pointed to
    // a: &mut T == T* const a;       // can't mutate pointer
// mut a: &mut T == T* a;             // can mutate both

extern crate cgmath;
use cgmath::Vector2;
use cgmath::vec2;

use std::collections::LinkedList;

const GOD_MODE : bool = true;
const SIZE : usize = 14;	
type MapType = [[MapItem; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
											//Dynamic array: Vec<Vec<Type>>

type Vec2 = Vector2<i32>;
type List = LinkedList<PosDir>;
							

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapItem {
	Wall,
	Food,
	Empty,
	SnakeHead,
	SnakePart,
	SnakeFood,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SnakeDir {
	Up,Left,Down, Right, None
}

pub fn is_inverse_dir (dir1 : SnakeDir, dir2 : SnakeDir) -> bool {
	if dir1 == SnakeDir::Up && dir2 == SnakeDir::Down {
		return true;
	}
	if dir1 == SnakeDir::Down && dir2 == SnakeDir::Up {
		return true;
	}	
	if dir1 == SnakeDir::Left && dir2 == SnakeDir::Right {
		return true;
	}
	if dir1 == SnakeDir::Right && dir2 == SnakeDir::Left {
		return true;
	}
	return false;
}

pub fn increase_pos_by_dir (pos : &Vec2,dir : SnakeDir) -> Vec2 {
	let new_pos;
	match dir {
		SnakeDir::Up => new_pos    = pos + vec2(0,-1),
		SnakeDir::Down => new_pos  = pos + vec2(0,1),
		SnakeDir::Right => new_pos = pos + vec2(1,0),
		SnakeDir::Left => new_pos  = pos + vec2(-1,0),
		_ => new_pos = pos.clone(),
	}
	return new_pos;
}

pub fn gen_random_vec2 () -> Vec2 {
	let mut rng = rand::thread_rng();
	let map_size = cgmath::vec2(1,  SIZE - 1);
	let i = rng.gen_range(map_size.x, map_size.y); 
	let j = rng.gen_range(map_size.x, map_size.y);
	return Vec2::new (j as i32, i as i32);
}

#[derive(Debug, Copy, Clone)]
pub struct PosDir {
	pos : Vec2,
	dir : SnakeDir,
	is_food : bool
}

impl PosDir {
	pub fn new (pos : Vec2, dir : SnakeDir) -> PosDir {
		return PosDir {pos : pos,dir : dir, is_food : false};
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
		match self.speed {
		    1  	=> 550,
			2 	=> 500,
			3	=> 450,
			4	=> 400,
			5	=> 350,
			6	=> 300,
			7	=> 250,
			8	=> 200,
			9	=> 150,
			10 	=> 100,
			_ 	=> 250,
		}
	}
	pub fn grow_tail (&mut self, old_pos : Vec2, snake_tail : PosDir, dir : SnakeDir) {
		if self.tail.is_empty () {
			self.tail.push_back(PosDir::new(old_pos,dir));
		} else {
			if let Some(val) = self.tail.front_mut() {
				val.is_food = true;
			}
		}
	}
	pub fn move_tail (&mut self) {
		if self.tail.is_empty() {
			return;
		}
		let mut prev_val;
		{
			let &mut front;
			match self.tail.front_mut() {
				Some(i) => front = i,
				None => return,
			}
			prev_val = front.clone();
			front.pos = increase_pos_by_dir (&front.pos, front.dir);
			front.dir = self.dir.clone();
		}
		
		{
			let length = self.tail.len().clone();
			let mut iter = self.tail.iter_mut ();
			iter.next(); //skip first item, already done
			
			for i in 1..length {
				let iter_val = iter.next();
				if let Some(val) = iter_val {
					let temp_actual_val = val.clone();
				
					val.pos = prev_val.pos;
					val.dir = prev_val.dir.clone();
					
					prev_val = temp_actual_val;
				} 
			}
		}
		self.move_food(prev_val);
	}
	pub fn move_food (&mut self, mut last_elem : PosDir) {
		let length = self.tail.len().clone();
		let mut vec : Vec<usize> = Vec::new();
		{
			let mut iter = self.tail.iter_mut ();
			for i in 0..length {
				let iter_val = iter.next();
				if let Some(val) = iter_val {
					if val.is_food {
						vec.push(i + 1);
						val.is_food = false;
					}
				}
			}
		}
		{
			let mut iter = self.tail.iter_mut ();
			for i in 0..length {
				let iter_val = iter.next();
				if vec.contains(&i) {
					if let Some(val) = iter_val {
						val.is_food = true;
					}
				}
			}
		}
		if !vec.is_empty () {
			if vec[vec.len() - 1] == length {
				last_elem.is_food = false;
				self.tail.push_back(last_elem);
			}
		}
	}
	pub fn get_last_tail (&self) -> PosDir {
		if self.tail.is_empty()  {
			return PosDir::new (Vec2::new (-1,-1), SnakeDir::Up);
		} else {
			match self.tail.back() {
				Some(i) => return i.clone(),
				None => panic!("error"),
			}
		}
	}
}

 

											
pub struct Map {		
	array : MapType
}

impl Map {
	pub fn new () -> Map {
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
		
		//Gen random pos for food
		let food_pos = gen_random_vec2 ();
		map[food_pos.y as usize][food_pos.x as usize] = MapItem::Food;
		
		
		return Map{array : map};
	}
	pub fn add_snake(&mut self) -> Snake {
		let map_size = Vec2::new(1,  (SIZE - 1) as i32);
		
		let center_pos = Vec2::new(map_size.y / 2, map_size.y / 2);
		self.add(center_pos, MapItem::SnakeHead); //TODO this can be removed
		
		return Snake::new(6,vec2(center_pos.y, center_pos.y ));
	}
	
	pub fn update_snake (&mut self, snake : &mut Snake) -> bool {
		let new_pos = increase_pos_by_dir (&snake.pos, snake.dir.clone());
		let old_pos = snake.pos.clone();
		
		//Save clones of the actual and next item
		//let actual_item     =  self.array[snake.pos.y as usize][snake.pos.x as usize].clone();
		let mut next_item   =  self.array[new_pos.y as usize][new_pos.x as usize].clone();
		let mut is_grow = false;
		
		if next_item == MapItem::Wall || next_item == MapItem::SnakePart || next_item == MapItem::SnakeFood {
			if !GOD_MODE {
				return false;
			}	
		}
		if next_item == MapItem::Food {
			next_item = MapItem::Empty;
			is_grow = true;
			//TODO better food gen
			loop { //add new food
				let food_pos = gen_random_vec2 ();
				if self.get(food_pos) == MapItem::Empty && food_pos != new_pos {
					self.add(food_pos, MapItem::Food);
					break;
				}
			}
		}
		let last_tail = snake.get_last_tail ();
		if next_item == MapItem::Empty {
			//Update pos
			snake.pos = new_pos;
			snake.move_tail();
		}
		
		if is_grow {
			let dir = snake.dir.clone();
			snake.grow_tail (old_pos, last_tail, dir);
			println!("Added snake tail at {}{}", old_pos.y, old_pos.x);
		}
		self.refresh_map(&snake);
		return true;
	}
	pub fn refresh_map (&mut self, snake : &Snake) {
		{
			let ref mut array = self.array;
		    for i in 0..array.len() { //TODO Refactor with lambda
				for j in 0..array[i].len() {
					if array[i][j] == MapItem::SnakeHead || 
						array[i][j] == MapItem::SnakePart ||
						array[i][j] == MapItem::SnakeFood {
						array[i][j] = MapItem::Empty;
					}
				}
		    }
		}
		self.add(snake.pos,MapItem::SnakeHead);
		for elem in &snake.tail {
			if elem.is_food {
				self.add(elem.pos, MapItem::SnakeFood);
			} else {
				self.add(elem.pos,MapItem::SnakePart);
			}
		}
	}
	
	pub fn add (&mut self ,pos : Vec2, item : MapItem) {
		self.array[pos.y as usize][pos.x as usize] = item;
	}
	pub fn get (&mut self, pos : Vec2) -> MapItem {
		return self.array[pos.y as usize][pos.x as usize];
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
			MapItem::SnakeHead => return 'C',
			MapItem::SnakePart => return 'x',
			MapItem::SnakeFood => return 'z',
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

type KeyMap = HashSet<sdl2::keyboard::Keycode>;

pub struct KeyHandler {
    prev_keys 		: KeyMap,
	keys 			: KeyMap,
	key_press 		: KeyMap,
	key_released 	: KeyMap,
}

impl KeyHandler {
	pub fn new () -> KeyHandler {
		return KeyHandler {	prev_keys : HashSet::new()	, keys : HashSet::new(),
							key_press: HashSet::new()	, key_released : HashSet::new()};
	}
	pub fn update (&mut self, events : &mut sdl2::EventPump) {
		self.prev_keys = self.keys.clone();
		self.keys = events.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();
    
        self.key_press = &self.keys - &self.prev_keys;
        self.key_released = &self.prev_keys - &self.keys;
	}
	
	pub fn handle_input (&self , prev_dir : SnakeDir) -> SnakeDir {
		let ref input = self.keys;
		if !input.is_empty() {
			if input.contains (&Keycode::W) || input.contains(&Keycode::Up) {
				return SnakeDir::Up;
			}
			if input.contains (&Keycode::A) || input.contains(&Keycode::Left) {
				return SnakeDir::Left;
			}
			if input.contains (&Keycode::D) || input.contains(&Keycode::Right) {
				return SnakeDir::Right;
			}
			if input.contains (&Keycode::S ) || input.contains (&Keycode::Down) {
				return SnakeDir::Down;
			}
		}
		return prev_dir;
	}
}


pub mod timing;

fn main() {
	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
	let mut events = sdl_context.event_pump().unwrap();

	//without window the keyboard handling is not working
	let _window = video_subsystem.window("Snake - Ryper", 800, 600)
        .position_centered()
        .build()
        .unwrap();

	
	let mut snake_dir 	= SnakeDir::None;
	let mut map       	= Map::new();
	let mut snake     	= map.add_snake();
	
	let ms_per_update 	= snake.convert_speed_to_ms ();
	let mut timer 		= timing::Timer::new();
	let mut key_handler = KeyHandler::new();
	
    'running: loop {
        for event in events.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            };
        }

        // Create a set of pressed Keys.
        key_handler.update(&mut events);
		let new_dir = key_handler.handle_input(snake_dir);
		if !is_inverse_dir (snake.dir.clone(), new_dir.clone()) {
			snake_dir = new_dir;
		}
		
		timer.update();
		if ms_per_update < timer.sum_elapsed_time {
			timer.sum_elapsed_time -= ms_per_update;
			
			snake.dir = snake_dir.clone();
			if !map.update_snake (&mut snake) {
				break;
			}
			MapDrawer::draw(&map);
		}
		timer.wait_fps_cap();
    }
}
