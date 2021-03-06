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

pub mod util;
use util::MapItem;
use util::new_item;
use util::MapStorage;
use util::SnakeDir;
use util::is_inverse_dir;
use util::ImgLoader;

const GOD_MODE : bool = true;
const SIZE : usize = 14;	
type MapType = [[MapStorage; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
											//Dynamic array: Vec<Vec<Type>>

type Vec2 = Vector2<i32>;
type List = LinkedList<PosDir>;



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
	prev_dir : SnakeDir,
	is_food : bool
}

impl PosDir {
	pub fn new (pos : Vec2, dir : SnakeDir) -> PosDir {
		return PosDir {pos : pos,dir : dir, is_food : false, prev_dir : dir};
	}
}

pub struct Snake {
	dir : SnakeDir,
	speed : u8, // 1-10 -> [500ms,100ms] refresh
	pos : Vec2, //Head pos
	tail : List,
}

//from pos2 to pos1
pub fn create_dir_from_pos (pos1 : &Vec2, pos2 : &Vec2) -> SnakeDir {
	let dir_pos = pos2 - pos1;
	if dir_pos == Vec2::new(0, 1) {
		return SnakeDir::Down;
	} else if dir_pos == Vec2::new(0,-1) {
		return SnakeDir::Up;
	} else if dir_pos == Vec2::new(1, 0) {
		return SnakeDir::Right;
	} else if dir_pos == Vec2::new(-1, 0) {
		return SnakeDir::Left;
	}
	panic!("error invalid pos");
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
	pub fn grow_tail (&mut self, old_pos : Vec2) {
		if self.tail.is_empty () {
			self.tail.push_back(PosDir::new(old_pos,self.dir));
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
			prev_val 		= front.clone();
			front.pos 		= increase_pos_by_dir (&front.pos, front.dir);
			front.prev_dir  = front.dir;
			front.dir 		= self.dir.clone();
		}
		
		{
			let length = self.tail.len().clone();
			let mut iter = self.tail.iter_mut ();
			iter.next(); //skip first item, already done
			
			for i in 1..length {
				let iter_val = iter.next();
				if let Some(val) = iter_val {
					let temp_actual_val = val.clone();
				
					val.prev_dir = val.dir;
					val.dir = prev_val.dir;
					val.pos = prev_val.pos;
					
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
		let mut map : MapType = [[new_item(MapItem::Empty); SIZE ]; SIZE];
		for i in 0..map.len() {
			for j in 0..map[i].len() {
				if i == 0 || j == 0  || i == SIZE - 1 || j == SIZE - 1 {
					map[i][j].item = MapItem::Wall;
				}
				else {
					map[i][j].item = MapItem::Empty;
				}
			}
		}
		
		//Gen random pos for food
		let food_pos = gen_random_vec2 ();
		map[food_pos.y as usize][food_pos.x as usize].item = MapItem::Food;
		
		
		return Map{array : map};
	}
	pub fn add_snake(&mut self, speed : u8) -> Snake {
		let map_size = Vec2::new(1,  (SIZE - 1) as i32);
		
		let center_pos = Vec2::new(map_size.y / 2, map_size.y / 2);
		
		return Snake::new(speed,vec2(center_pos.y, center_pos.y ));
	}
	
	pub fn update_snake (&mut self, snake : &mut Snake) -> bool {
		let new_pos = increase_pos_by_dir (&snake.pos, snake.dir.clone());
		let old_pos = snake.pos.clone();
		
		//Save clones of the actual and next item
		//let actual_item     =  self.array[snake.pos.y as usize][snake.pos.x as usize].clone();
		let mut next_item   =  self.array[new_pos.y as usize][new_pos.x as usize].item.clone();
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
					self.add(food_pos, MapItem::Food, SnakeDir::None);
					break;
				}
			}
		}
		if next_item == MapItem::Empty {
			//Update pos
			snake.pos = new_pos;
			snake.move_tail();
		}
		
		if is_grow {
			snake.grow_tail (old_pos);
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
					if array[i][j].item == MapItem::SnakeHead || 
						array[i][j].item == MapItem::SnakePart ||
						array[i][j].item == MapItem::SnakeFood {
						array[i][j].item = MapItem::Empty;
					}
				}
		    }
		}
		self.add(snake.pos,MapItem::SnakeHead, snake.dir);
		for elem in &snake.tail {
			if elem.is_food {
				self.add(elem.pos, MapItem::SnakeFood, SnakeDir::None);
			} else {
				self.add(elem.pos,MapItem::SnakePart , elem.dir);
			}
		}
	}
	
	pub fn add (&mut self ,pos : Vec2, item : MapItem, dir : SnakeDir) {
		self.array[pos.y as usize][pos.x as usize].item = item;
		self.array[pos.y as usize][pos.x as usize].dir 	= dir;
	}
	pub fn get (&mut self, pos : Vec2) -> MapItem {
		return self.array[pos.y as usize][pos.x as usize].item;
	}
}

pub struct MapDrawer {
}

extern crate sdl2;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;

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
	
	pub fn draw_console(map : &Map) {
		std::process::Command::new("clear").status().unwrap(); //TODO not the best solution for clearing the screen
	 	let ref array = map.array; 
		for i in 0..array.len() {
			for j in 0..array[i].len() {
				print!("{}", MapDrawer::get_char(&array[i][j].item));
			}
			println!("");
		}
	}
	pub fn ij_to_screen (i : usize, j : usize, elem_size : u32) -> Vec2 {
		return Vec2::new ((j * elem_size as usize)as i32, (i * elem_size as usize) as i32);
	}
	pub fn draw_sdl(map : &Map,snake : &Snake, renderer : &mut WindowCanvas, textures : &util::TextureStorage) {
		const ELEM_SIZE : u32 = 20;
	
		// let background = Color::RGB(87, 160, 4); old background 
		let background = Color::RGB(62, 150, 53);
		let black = Color::RGB(0,0,0);
		renderer.set_draw_color (background);
		renderer.clear();
		renderer.set_draw_color (black);
		let ref array = map.array; 
		for i in 0..array.len() {
			for j in 0..array[i].len() {
				renderer.set_draw_color (black);
				let pos = MapDrawer::ij_to_screen(i,j,ELEM_SIZE);
				if array[i][j].item == MapItem::Wall {
					renderer.fill_rect(Some(Rect::new(pos.x, pos.y, ELEM_SIZE, ELEM_SIZE))).expect("Failed to draw rect");
				}
				else if array[i][j].item == MapItem::Food {
					let offset = (ELEM_SIZE / 3) as i32;
					renderer.fill_rect(Some(Rect::new(pos.x + offset, pos.y, offset as u32, offset as u32))).expect("Failed to draw rect");
					renderer.fill_rect(Some(Rect::new(pos.x, pos.y + offset, offset as u32, offset as u32))).expect("Failed to draw rect");
					renderer.fill_rect(Some(Rect::new(pos.x + offset * 2, pos.y + offset, offset as u32, offset as u32))).expect("Failed to draw rect");
					renderer.fill_rect(Some(Rect::new(pos.x + offset, pos.y + offset * 2, offset as u32, offset as u32))).expect("Failed to draw rect");
				}
			}
		}
		let snake_pos = MapDrawer::ij_to_screen(snake.pos.y as usize,snake.pos.x as usize,ELEM_SIZE);
		let snake_rect = Some(Rect::new(snake_pos.x,snake_pos.y,ELEM_SIZE,ELEM_SIZE));
		if snake.dir == SnakeDir::Left {
			renderer.copy(&textures.head_left, None, snake_rect).unwrap();
		} else if snake.dir == SnakeDir::Right {
			renderer.copy(&textures.head_right, None, snake_rect).unwrap();
		} else if snake.dir == SnakeDir::Up {
			renderer.copy(&textures.head_up, None, snake_rect).unwrap();
		} else {
			renderer.copy(&textures.head_down, None, snake_rect).unwrap();
		}
		
		for tail in &snake.tail {
			let pos = MapDrawer::ij_to_screen(tail.pos.y as usize,tail.pos.x as usize,ELEM_SIZE);
			let rect = Some(Rect::new(pos.x,pos.y,ELEM_SIZE,ELEM_SIZE));
			if tail.dir == tail.prev_dir {
				if tail.dir == SnakeDir::Left || tail.dir == SnakeDir::Right {
					if tail.is_food {
						renderer.copy(&textures.body_right_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_right, None, rect).unwrap();
					}
				} else {
					if tail.is_food {
						renderer.copy(&textures.body_up_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_up, None, rect).unwrap();
					}
				}
			} else {
				if (tail.prev_dir == SnakeDir::Right && tail.dir == SnakeDir::Up) ||
					(tail.prev_dir == SnakeDir::Down && tail.dir == SnakeDir::Left) {
					if tail.is_food {
						renderer.copy(&textures.body_right_up_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_right_up, None, rect).unwrap();
					}
				} else if (tail.prev_dir == SnakeDir::Right && tail.dir == SnakeDir::Down)  ||
					(tail.prev_dir == SnakeDir::Up && tail.dir == SnakeDir::Left) {
					if tail.is_food {
						renderer.copy(&textures.body_right_down_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_right_down, None, rect).unwrap();
					}
				} else if (tail.prev_dir == SnakeDir::Left && tail.dir == SnakeDir::Down) ||
					(tail.prev_dir == SnakeDir::Up && tail.dir == SnakeDir::Right){
					if tail.is_food {
						renderer.copy(&textures.body_left_down_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_left_down, None, rect).unwrap();
					}
				} else if (tail.prev_dir == SnakeDir::Left && tail.dir == SnakeDir::Up ) || 
					(tail.prev_dir == SnakeDir::Down && tail.dir == SnakeDir::Right){
					if tail.is_food {
						renderer.copy(&textures.body_left_up_food, None, rect).unwrap();
					} else {
						renderer.copy(&textures.body_left_up, None, rect).unwrap();
					}
				}
			} 
		}
		
		renderer.present();
	}
}

pub mod input;
pub mod timing;


fn main() {
	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
	let mut events = sdl_context.event_pump().unwrap();

	//without window the keyboard handling is not working
	let _window = video_subsystem.window("Snake - Ryper", 800, 600)
        .position_centered()
		.opengl()
        .build()
        .unwrap();
		
	let mut renderer : WindowCanvas = _window.into_canvas().build().unwrap();
	let texture_creator = renderer.texture_creator();
	
	let textures  = ImgLoader::build_textures (&texture_creator);
	
	renderer.clear();
	renderer.copy(&textures.head_left, None, Some(Rect::new(100,100,20,20))).unwrap();
	renderer.present();

	
	let mut snake_dir 	= SnakeDir::None;
	let mut map       	= Map::new();
	let mut snake     	= map.add_snake(7);
	
	let ms_per_update 	= snake.convert_speed_to_ms ();
	let mut timer 		= timing::Timer::new();
	let mut key_handler = input::KeyHandler::new();
	
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
			MapDrawer::draw_console(&map);
			MapDrawer::draw_sdl (&map,&snake, &mut renderer, &textures);
		}
		timer.wait_fps_cap();
    }
}
