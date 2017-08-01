#[derive(Debug)]
enum MapItem {
	Wall,
	Food,
	Empty,
	SnakeHead,
	SnakePart,
}

const SIZE : usize = 5;
type MapType = [[MapItem; SIZE];SIZE]; 	//Fixed size array - 1D array [Type, size]
										//Dynamic array: Vec<Vec<Type>>

fn main() {
	
	//TODO better initialization
    let array : MapType = [[MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
						  [MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall]];

	for i in 0..array.len() {
		for j in 0..array[i].len() {
			print!("[{}][{}]:{:?} | ", i,j, array[i][j]);
		}
		println!("");
	}
		
}
