use concat_idents::concat_idents;
use const_str::to_char_array;

macro_rules! include_many {
	( $( $name:ident ),* ) => {
		$(
			concat_idents!(name = $name, _CHAR {
				const name: char = to_char_array!(stringify!($id))[0];
			});
			pub const $name: &str = include_str!(concat!("./svg/", stringify!($name), ".svg"));
		)*

		pub fn get_acid_svg_by_shorthand(shorthand: char) -> Option<&'static str> {
			match shorthand.to_ascii_uppercase() {
				$(concat_idents!(name = $name , _CHAR { name } ) => Some($name),)*
				_ => None
			}
		}
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
