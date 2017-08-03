extern crate sdl2;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

type KeyMap = HashSet<sdl2::keyboard::Keycode>;

use util::SnakeDir;

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