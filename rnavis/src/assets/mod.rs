use concat_idents::concat_idents;
use const_str::to_char_array;

macro_rules! include_many_lookup {
	( $( $name:ident ),* ) => {
		include_many!($($name),*);
		$(
			concat_idents!(id = $name, _CHAR {
				const id: char = to_char_array!(stringify!($name))[0];
			});
		)*

		pub fn get_acid_svg_by_shorthand(shorthand: char) -> Option<&'static str> {
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
include_many!(
	BASE,
	BASE_LINK,
	BASE_P_LINK,
	BASE_P
);

#[rustfmt::skip]
include_many_lookup!(
	A, C, D, E,
	F, H, I, K, 
	L, M, N, P,
	Q, R, S, T,
	U, V, W, Y
);
