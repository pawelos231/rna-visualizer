macro_rules! include_strs {
	( $( $name:ident ),* ) => {
		$(
			const $name: &str = include_str!(concat!("./svg/", stringify!($name), ".svg"));
		)*
	};
}

macro_rules! include_bases {
	( $( $name:ident ),* ) => {
		include_strs!($($name),*);

		#[derive(Clone, Copy)]
		#[allow(non_camel_case_types)]
		#[allow(clippy::upper_case_acronyms)]
		pub enum BaseType { $($name,)* }

		pub const BASES: [BaseType; 8] = [$(BaseType::$name),*];

		pub fn get_base_svg(base_type: BaseType) -> Option<&'static str> {
			match base_type {
				$(BaseType::$name => Some($name),)*
			}
		}
	};
}

pub struct Body {
	regular: &'static str,
	flipped: Option<&'static str>,
}

impl Body {
	const fn new(regular: &'static str) -> Self {
		Self {
			regular,
			flipped: None,
		}
	}

	const fn new_flipped(regular: &'static str, flipped: &'static str) -> Self {
		Self {
			regular,
			flipped: Some(flipped),
		}
	}

	pub const fn get_regular(&self) -> &'static str {
		self.regular
	}

	pub const fn get_flipped(&self) -> Option<&'static str> {
		self.flipped
	}
}

pub const fn get_body(index: char) -> Option<Body> {
	Some(match index {
		'A' => Body::new(A),
		'C' => Body::new_flipped(C, C_FLIP),
		'D' => Body::new(D),
		'E' => Body::new(E),
		'F' => Body::new(F),
		'G' => Body::new(G),
		'H' => Body::new_flipped(H, H_FLIP),
		'I' => Body::new(I),
		'K' => Body::new_flipped(K, K_FLIP),
		'L' => Body::new(L),
		'M' => Body::new_flipped(M, M_FLIP),
		'N' => Body::new_flipped(N, N_FLIP),
		'P' => Body::new(P),
		'Q' => Body::new_flipped(Q, Q_FLIP),
		'R' => Body::new_flipped(R, R_FLIP),
		'S' => Body::new(S),
		'T' => Body::new(T),
		'U' => Body::new_flipped(U, U_FLIP),
		'V' => Body::new(V),
		'W' => Body::new_flipped(W, W_FLIP),
		'Y' => Body::new(Y),
		_ => return None,
	})
}

#[rustfmt::skip]
include_bases!(
	BASE,
	BASE_NO_LEFT,
	BASE_NO_SIDES,
	BASE_NO_RIGHT,
	BASE_LINK,
	BASE_P,
	BASE_P_NO_RIGHT,
	BASE_P_LINK
);

#[rustfmt::skip]
include_strs!(
	A, C, C_FLIP, D, E, F, H, H_FLIP, I, 
	K, K_FLIP, L, M, M_FLIP, N, N_FLIP, P,
	Q, Q_FLIP, R, R_FLIP, S, T, U, U_FLIP,
	V, W, W_FLIP, Y, G
);
