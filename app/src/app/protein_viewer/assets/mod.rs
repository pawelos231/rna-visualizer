use concat_idents::concat_idents;
use const_str::to_char_array;

macro_rules! include_bases {
	( $( $name:ident ),* ) => {
		include_many!($($name),*);

		#[derive(Clone, Copy)]
		pub enum BaseType { $($name,)* }

		pub fn get_base_svg(base_type: BaseType) -> Option<&'static str> {
			match base_type {
				$(BaseType::$name => Some($name),)*
			}
		}
	};
}

macro_rules! include_bodies {
	( $( $name:ident ),* ) => {
		include_many!($($name),*);
		$(
			concat_idents!(id = $name, _CHAR {
				const id: char = to_char_array!(stringify!($name))[0];
			});
		)*

		pub fn get_acid_svg(shorthand: char) -> Option<&'static str> {
			match shorthand.to_ascii_uppercase() {
				$(concat_idents!(id = $name , _CHAR { id } ) => Some($name),)*
				_ => None,
			}
		}
	};
}

macro_rules! include_many {
	( $( $name:ident ),* ) => {
		$(
			pub const $name: &str = include_str!(concat!("./svg/", stringify!($name), ".svg"));
		)*
	};
}

#[rustfmt::skip]
include_bases!(
	BASE,
	BASE_NO_LEFT,
	BASE_NO_SIDES,
	BASE_NO_RIGHT,
	BASE_LINK,
	BASE_P_LINK,
	BASE_P
);

#[rustfmt::skip]
include_bodies!(
	A, C, D, E,
	F, H, I, K, 
	L, M, N, P,
	Q, R, S, T,
	U, V, W, Y,
	G
);

pub const SUPPORTED_ACIDS: [char; 21] = [
	'A', 'C', 'D', 'E', 'F', 'H', 'I', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
	'Y', 'G',
];
