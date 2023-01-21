use crate::Acid;

pub struct Bases {
	pub n_term: u32,
	pub k: Acid,
	pub r: Acid,
	pub h: Acid,
}

impl Bases {
	pub fn init_bases() -> Self {
		Self {
			n_term: 0,
			k: Acid::K,
			r: Acid::R,
			h: Acid::H,
		}
	}
}
