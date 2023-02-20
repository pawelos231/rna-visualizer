#[derive(Default, Clone, Copy)]
pub struct Counts {
	c: u32,
	w: u32,
	y: u32,
	d: u32,
	e: u32,
	k: u32,
	r: u32,
	h: u32,
}

impl Counts {
	pub fn add_raw(&mut self, shorthand: u8) {
		match shorthand {
			b'c' | b'C' => self.c += 1,
			b'w' | b'W' => self.w += 1,
			b'y' | b'Y' => self.y += 1,
			b'd' | b'D' => self.d += 1,
			b'e' | b'E' => self.e += 1,
			b'k' | b'K' => self.k += 1,
			b'r' | b'R' => self.r += 1,
			b'h' | b'H' => self.h += 1,
			_ => (),
		}
	}

	pub const fn get_c(&self) -> u32 {
		self.c
	}

	pub const fn get_w(&self) -> u32 {
		self.w
	}

	pub const fn get_y(&self) -> u32 {
		self.y
	}

	pub const fn get_d(&self) -> u32 {
		self.d
	}

	pub const fn get_e(&self) -> u32 {
		self.e
	}

	pub const fn get_k(&self) -> u32 {
		self.k
	}

	pub const fn get_r(&self) -> u32 {
		self.r
	}

	pub const fn get_h(&self) -> u32 {
		self.h
	}
}
