extern crate rand; //For random number generation
use rand::Rng;

extern crate cgmath;
use cgmath::prelude::*;

#[derive(Debug, Copy, Clone)]
enum MapItem {
	Wall,
	Food,
	Empty,
	SnakeHead,
	SnakePart,
}

// enum SnakeDir {
	// Up,Left,Down, Right
// }


const SIZE : usize = 10;	
type MapType = [[MapItem; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
											//Dynamic array: Vec<Vec<Type>>
											
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
		
		// let map_size = cgmath::vec2{x: 1, y: SIZE - 1};
		let map_size = cgmath::vec2(1,  SIZE - 1);
		
		//Gen random pos for food
		let i = rng.gen_range(map_size.x, map_size.y); 
		let j = rng.gen_range(map_size.x, map_size.y);
		map[i][j] = MapItem::Food;
		
		
		let center_pos = map_size / 2;
		map[center_pos.y][center_pos.y] = MapItem::SnakeHead;
		
		return Map{array : map};
	}
}

pub struct MapDrawer {
}

impl MapDrawer {
	pub fn draw_debug(map : Map) {
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
			MapItem::SnakeHead | MapItem::SnakePart => return 'x',
			// _ => return '*'
		}
	}
	
	pub fn draw(map : Map) {
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

fn main() {
	let map = Map::new();
	
	MapDrawer::draw(map);
	
	
}
