use crate::Acid;

pub struct Terminals {
	pub count: u32,
	pub pk: f32,
}

impl Terminals {
	pub fn init_terminal(first_amino: &f32) -> Self {
		Self {
			count: 1,
			pk: *first_amino,
		}
	}
}

pub struct Bases {
	pub n_term: Terminals,
	pub k: Acid,
	pub r: Acid,
	pub h: Acid,
}

impl Bases {
	pub fn init_bases(second_pk: &f32) -> Self {
		Self {
			n_term: Terminals::init_terminal(second_pk),
			k: Acid::K,
			r: Acid::R,
			h: Acid::H,
		}
	}
}

pub struct Acids {
	pub c_term: Terminals,
	pub d: Acid,
	pub e: Acid,
	pub c: Acid,
	pub y: Acid,
}

impl Acids {
	pub fn init_acids(first_pk: &f32) -> Self {
		Self {
			c_term: Terminals::init_terminal(first_pk),
			d: Acid::D,
			e: Acid::E,
			c: Acid::C,
			y: Acid::Y,
		}
	}
}
/*
pub fn log_bases(hash_map: &HashMap<String, Acid>) {
	for (key, value) in *hash_map {
		println!("{}, {}", key, value)
	}
}
*/
