use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use std;

pub struct Timer {
	pub sum_elapsed_time 	: u64,
	actual_elapsed_time : u64,
	last_frame_time  	: u64,
}

impl Timer {
	pub fn new () -> Timer {
		return Timer {	sum_elapsed_time 	: 0, 
						last_frame_time 	: Timer::get_time_in_ms (),
						actual_elapsed_time : 0};
	}
	pub fn get_time_in_ms () -> u64 {
		let start 			= SystemTime::now();
		let since_the_epoch = start.duration_since(UNIX_EPOCH)
			.expect("Time went backwards");
		let in_ms = since_the_epoch.as_secs() * 1000 +
					since_the_epoch.subsec_nanos() as u64 / 1_000_000;
		return in_ms;
	}
	pub fn update (&mut self) {
		self.actual_elapsed_time  	= Timer::get_time_in_ms() - self.last_frame_time;
		self.last_frame_time 		= Timer::get_time_in_ms();
		self.sum_elapsed_time 		+= self.actual_elapsed_time;
	}
	pub fn wait_fps_cap (&mut self) {
		let frame_per_sec_cap    : u64 = 1000 / 60;
		
		let mut sleep_time = 0;
		if self.actual_elapsed_time < frame_per_sec_cap { //60 fps cap
			sleep_time = frame_per_sec_cap - self.actual_elapsed_time;
		}
        std::thread::sleep(Duration::from_millis(sleep_time)); //Sleep for 60 fps
	}
}
