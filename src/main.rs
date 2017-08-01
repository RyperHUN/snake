#[derive(Debug)]
enum MapItem {
	Wall,
	Food,
	Empty,
	SnakeHead,
	SnakePart,
}


fn main() {
	const SIZE : usize = 5;
    let array : [[MapItem; SIZE];SIZE] = [[MapItem::Wall, MapItem::Food, MapItem::Empty, MapItem::SnakeHead, MapItem::Wall],
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
