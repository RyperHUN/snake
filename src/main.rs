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
	
	pub fn draw (&self) {
		let ref array = self.array; 
		//let array = &self.array; //Same as the above line, just another syntax
		for i in 0..array.len() {
			for j in 0..array[i].len() {
				print!("[{}][{}]:{:?} | ", i,j, array[i][j]);
			}
			println!("");
		}
	}
}

pub struct MapDrawer {
}

impl MapDrawer {
	pub fn draw(map : Map) {
		
	}
}

fn main() {
	let map = Map::new();
	
	map.draw();
}
