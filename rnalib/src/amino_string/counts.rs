#[derive(Default, Clone, Copy)]
pub struct Counts {
	c: u16,
	w: u16,
	y: u16,
	d: u16,
	e: u16,
	k: u16,
	r: u16,
	h: u16,
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

	pub const fn get_c(&self) -> u16 {
		self.c
	}

	pub const fn get_w(&self) -> u16 {
		self.w
	}

	pub const fn get_y(&self) -> u16 {
		self.y
	}

	pub const fn get_d(&self) -> u16 {
		self.d
	}

	pub const fn get_e(&self) -> u16 {
		self.e
	}

	pub const fn get_k(&self) -> u16 {
		self.k
	}

	pub const fn get_r(&self) -> u16 {
		self.r
	}

	pub const fn get_h(&self) -> u16 {
		self.h
	}
}
