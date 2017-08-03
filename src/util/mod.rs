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