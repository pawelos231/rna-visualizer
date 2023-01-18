use usvg::{Options, Tree};

pub const BASE: &str = include_str!("./svg/base.svg");
pub const R: &str = include_str!("./svg/R.svg");
pub const H: &str = include_str!("./svg/H.svg");
pub const K: &str = include_str!("./svg/K.svg");
pub const D: &str = include_str!("./svg/D.svg");
pub const E: &str = include_str!("./svg/E.svg");

pub fn get_base() -> Tree {
	Tree::from_str(BASE, &Options::default()).unwrap()
}

pub fn get_characteristic_structure_svg(key: char) -> Option<Tree> {
	match key.to_ascii_uppercase() {
		'R' => Some(R),
		'H' => Some(H),
		'K' => Some(K),
		'D' => Some(D),
		'E' => Some(E),
		_ => None,
	}
	.and_then(|x| Some(Tree::from_str(x, &Options::default()).unwrap()))
}
