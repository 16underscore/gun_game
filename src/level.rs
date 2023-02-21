use valence::prelude::*;

#[derive(Component, Default)]
pub struct Level(u8);

impl Level {
	pub fn increase(&mut self) {
		self.0 += 1;
	}
	pub fn decrease(&mut self) {
		self.0 /= 2;
	}
}
