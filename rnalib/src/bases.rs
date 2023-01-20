use crate::Acid;

pub struct Bases {
	pub Nterm: u32,
	pub K: Acid,
	pub R: Acid,
	pub H: Acid,
}

impl Bases {
	pub fn init_bases() -> Self {
		Self {
			Nterm: 0,
			K: Acid::K,
			R: Acid::R,
			H: Acid::H,
		}
	}
}
