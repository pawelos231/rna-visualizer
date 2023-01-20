use usvg::{Options, Tree};

macro_rules! include_many {
	( $( $name:ident ),* ) => {
		$(
			pub const $name: &str = include_str!(concat!("./svg/", stringify!($name), ".svg"));
		)*
	};
}

#[rustfmt::skip]
include_many!(
	BASE,
	BASE_LINK,
	BASE_P_LINK,
	BASE_P,
	A, C, D, E,
	F, H, I, K, 
	L, M, N, P,
	Q, R, S, T,
	U, V, W, Y
);

// pub const BASE: &str = include_str!("./svg/base.svg");
// pub const R: &str = include_str!("./svg/R.svg");

pub fn get_base() -> Tree {
	Tree::from_str(BASE, &Options::default()).unwrap()
}

/*pub fn get_characteristic_structure_svg(key: char) -> Option<Tree> {
	match key.to_ascii_uppercase() {
		'R' => Some(),
		_ => None,
	}
	.and_then(|x| Some(Tree::from_str(x, &Options::default()).unwrap()))
}*/
