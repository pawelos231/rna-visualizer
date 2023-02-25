use crate::Codon;

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
	pub fn add(&mut self, codon: &Codon) {
		match codon {
			Codon::C => self.c += 1,
			Codon::W => self.w += 1,
			Codon::Y => self.y += 1,
			Codon::D => self.d += 1,
			Codon::E => self.e += 1,
			Codon::K => self.k += 1,
			Codon::R => self.r += 1,
			Codon::H => self.h += 1,
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
