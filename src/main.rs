#[derive(Debug)]
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


const SIZE : usize = 5;	
type MapType = [[MapItem; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
											//Dynamic array: Vec<Vec<Type>>
											
pub struct Map {		
	array : MapType
}

impl Map {
	pub fn new () -> Map {
		return Map{ array : [[MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall]]};
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
		match map_item {
			_ => return '_'
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

fn main() {
	let map = Map::new();
	
	MapDrawer::draw(map);
}
