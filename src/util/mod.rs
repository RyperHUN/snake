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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MapStorage {
	pub dir : SnakeDir,
	pub item : MapItem,
}

pub fn new_item (item : MapItem) -> MapStorage {
	return MapStorage {dir : SnakeDir::None, item : item};
}
impl MapStorage {
	pub fn new (item : MapItem, dir : SnakeDir) -> MapStorage { 
		return MapStorage {dir : dir, item : item};
	}
}

extern crate sdl2;
extern crate lodepng;
extern crate rgb;
use self::rgb::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::render::Texture;

pub struct TextureStorage<'r> {
	pub head_down	 : Texture<'r>,
	pub head_up 	 : Texture<'r>,
	pub head_left 	 : Texture<'r>,
	pub head_right	 : Texture<'r>,
	
	pub body_right 	: Texture<'r>,
	pub body_up 	: Texture<'r>,
	
	pub body_left_up	: Texture<'r>,
	pub body_right_up 	: Texture<'r>,
	pub body_left_down 	: Texture<'r>,
	pub body_right_down : Texture<'r>,
}

pub struct ImgLoader {

}

impl ImgLoader {
	pub fn build_textures<'a> (texture_creator : &'a TextureCreator<WindowContext>) -> TextureStorage {
		let mut image = lodepng::decode24_file("snake.png").unwrap();
		let bytes: &[u8] = image.buffer.as_ref().as_bytes();
	
		let head_right 	= ImgLoader::create_texture (bytes, &texture_creator, 0, 0);
		let head_up 	= ImgLoader::create_texture (bytes, &texture_creator, 0, 1);
		let head_left 	= ImgLoader::create_texture (bytes, &texture_creator, 1, 0);
		let head_down 	= ImgLoader::create_texture (bytes, &texture_creator, 1, 1);
		
		let body_right 	= ImgLoader::create_texture (bytes, &texture_creator, 0, 2);
		let body_up 	= ImgLoader::create_texture (bytes, &texture_creator, 1, 2);
		
		let body_left_up 	= ImgLoader::create_texture (bytes, &texture_creator, 2, 2);
		let body_right_up 	= ImgLoader::create_texture (bytes, &texture_creator, 2, 3);
		let body_left_down 	= ImgLoader::create_texture (bytes, &texture_creator, 2, 1);
		let body_right_down	= ImgLoader::create_texture (bytes, &texture_creator, 0, 3);
		
		return TextureStorage{	head_right : head_right, 		head_up : head_up, 				head_left :  head_left,
								head_down : head_down, 			body_right : body_right, 		body_up : body_up,
								body_left_up : body_left_up, 	body_right_up : body_right_up,	body_left_down : body_left_down,
								body_right_down : body_right_down};
	}
	fn create_texture<'r> (img_bytes : &[u8],texture_creator : &'r TextureCreator<WindowContext>,
	selectedi : usize, selectedj : usize) -> sdl2::render::Texture<'r> {
		let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 20, 20).unwrap();
		// Create a red-green gradient
		texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
			for y in 0..20 {
				let block_size = 20;
				let width = 80;
				let pic_start_offset = y * width * 3 + selectedj * block_size * 3 + selectedi * width * 3;
				for x in 0..20 {
					let offset = y*pitch + x*3;
					let pic_offset = pic_start_offset + x * 3;
					buffer[offset + 0] = img_bytes[pic_offset + 0];
					buffer[offset + 1] = img_bytes[pic_offset + 1];
					buffer[offset + 2] = img_bytes[pic_offset + 2];
				}
			}
		}).unwrap();
		
		return texture;
	}
}
