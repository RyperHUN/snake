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

fn main() {
	let map = Map::new();
	
	MapDrawer::draw(map);
}
