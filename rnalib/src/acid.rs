//! This module contains easy to access, human readable
//! amino acid constants. See [`Acid`] for more information.

use concat_idents::concat_idents;
use const_str::*;
use std::fmt::{Display, Formatter, Result};

/// A bucket type holding given amino acid's
/// physical properties.
pub struct Acid {
	/// A three letter shorthand uniquely identifying
	/// a given amino acid.
	pub three_letter: &'static str,
	/// Monoisotopic mass value minus [`ALPHA_MASS`].
	pub sc_mass: f32,
	/// The α-carboxyl group value.
	pub pk1: f32,
	/// The α-ammonium ion value.
	pub pk2: f32,
	/// The side chain group value. This property is
	/// uniquely present in amino acids with ionisable
	/// side chains.
	pub pk3: Option<f32>,
	/// Hydrophobicity value.
	pub sc_phob: f32,
	/// The molar extinction coefficient. Describes
	/// how much light a molecule absorbs at a
	/// wavelength of 280nm.
	pub extco: Option<u32>,
}

impl Acid {
	/// Creates a constant amino acid physical property
	/// bucket from properties given
	pub const fn new(
		three_letter: &'static str,
		sc_mass: f32,
		pk1: f32,
		pk2: f32,
		pk3: Option<f32>,
		sc_phob: f32,
		extco: Option<u32>,
	) -> Self {
		Self {
			three_letter,
			sc_mass,
			pk1,
			pk2,
			pk3,
			sc_phob,
			extco,
		}
	}

	/// Returns the raw shorthand of the amino acid.
	/// This is equal to the first letter of its
	/// assigned three-letter shorthand.
	pub const fn get_shorthand_raw(&self) -> u8 {
		self.three_letter.as_bytes()[0]
	}
}

impl Display for Acid {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"{}, {:?}, {}, {}, {:?}, {}, {}",
			self.three_letter, self.extco, self.pk1, self.pk2, self.pk3, self.sc_phob, self.sc_mass
		)
	}
}

/// This macro constructs a table of constant
/// amino acid properties.
///
/// It's useful, as it allows to store physical
/// constants in well-formatted, human readable,
/// tabular format (see lower).
macro_rules! acid_table {
	( $( $id:ident, $three_letter:expr, $sc_mass:expr, $pk1:expr, $pk2:expr, $pk3:expr, $sc_hbob:expr, $extco:expr )* ) => {
		$(
			concat_idents!(name = $id, _CHAR {
				const name: char = to_char_array!(stringify!($id))[0];
			});
		)*

		impl Acid {
			/// A convenience method used to index amino acid's
			/// physical properties by its single-letter shorthand.
			///
			/// Returns [`None`] if no properties have been defined
			/// for an acid with given shorthand, or if such acid does
			/// not exist.
			$(#[allow(clippy::excessive_precision)] pub const $id: Acid = Acid::new($three_letter, $sc_mass, $pk1, $pk2, $pk3, $sc_hbob, $extco);)*
			pub const fn from_shorthand(code: char) -> Option<Acid> {
				match code.to_ascii_uppercase() {
					$(concat_idents!(name = $id , _CHAR { name } ) => Some(Self::$id),)*
					_ => None
				}
			}
		}
	};
}

//	Short 	Name	Mass		Pk1 	Pk2 	Pk3 			Hbob 	Extinction coef.
acid_table!(
	Y,		"Tyr",	107.0495,	2.20,	9.210,	Some(10.07),	-0.710,	Some(1490)
	A,		"Ala",	15.02340,	2.35,	9.870,	None,			0.500,	None
	R,		"Arg",	100.0873,	1.82,	8.990,	Some(12.38),	1.810,	None
	N,		"Asn",	58.02920,	2.14,	8.720,	None,			0.850,	None
	D,		"Asp",	59.01320,	1.99,	9.900,	Some(3.9),		3.640,	None
	C,		"Cys",	46.99950,	1.92,	10.70,	Some(8.3),		-0.02,	Some(125)
	Q,		"Gln",	72.04480,	2.17,	9.130,	None,			0.770,	None
	E,		"Glu",	73.02880,	2.10,	9.470,	Some(4.07),		3.630,	None
	G,		"Gly",	1.007800,	2.35,	9.780,	None,			1.150,	None
	H,		"Asp",	81.04520,	1.8,	9.33,	Some(6.04),		2.330,	None
	I,		"Ile",	57.07020,	2.32,	9.760,	None,			-1.12,	None
	L,		"Leu",	57.07020,	2.33,	9.740,	None,			-1.25,	None
	K,		"Lys",	72.08110,	2.16,	9.060,	Some(10.54),	2.800,	None
	M,		"Met",	75.02670,	2.13,	9.280,	None,			-0.67,	None
	F,		"Phe",	91.05460,	2.20,	9.310,	None,			-1.71,	None
	P,		"Pro",	41.03900,	1.95,	10.64,	None,			0.140,	None
	S,		"Ser",	31.01830,	2.19,	9.210,	None,			0.460,	None
	T,		"THR",	45.03390,	2.09,	9.100,	None,			0.250,	None
	W,		"Trp",	130.0655,	2.46,	9.410,	None,			-2.09,	Some(5500)
	V,		"Lys",	43.05460,	2.39,	9.740,	None,			-0.46,	None
);
