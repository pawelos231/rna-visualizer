use std::fmt;

struct AcidData {
	three_letter: String,
	sc_mass: f32,
	pk1: f32,
	pk2: f32,
	pk3: Option<f32>,
	sc_hbob: f32,
	extco: Option<u32>,
}

impl AcidData {
	fn new(a: String, b: f32, c: f32, d: f32, e: Option<f32>, f: f32, g: Option<u32>) -> AcidData {
		AcidData {
			three_letter: a,
			sc_mass: b,
			pk1: c,
			pk2: d,
			pk3: e,
			sc_hbob: f,
			extco: g,
		}
	}
}
impl Default for AcidData {
	fn default() -> AcidData {
		AcidData {
			three_letter: "a".to_string(),
			sc_mass: 0.0,
			pk1: 0.0,
			pk2: 0.0,
			pk3: Some(0.0),
			sc_hbob: 0.32,
			extco: Some(0),
		}
	}
}
impl fmt::Display for AcidData {
	// This trait requires `fmt` with this exact signature.
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Write strictly the first element into the supplied output
		// stream: `f`. Returns `fmt::Result` which indicates whether the
		// operation succeeded or failed. Note that `write!` uses syntax which
		// is very similar to `println!`.
		write!(
			f,
			"{}, {:?}, {}, {}, {:?}, {}, {}",
			self.three_letter, self.extco, self.pk1, self.pk2, self.pk3, self.sc_hbob, self.sc_mass
		)
	}
}
pub struct Acid_table {
	Y: AcidData,
	A: AcidData,
	N: AcidData,
	R: AcidData,
	C: AcidData,
	D: AcidData,
	Q: AcidData,
	E: AcidData,
	G: AcidData,
	H: AcidData,
	I: AcidData,
	L: AcidData,
	K: AcidData,
	M: AcidData,
	F: AcidData,
	P: AcidData,
	S: AcidData,
	T: AcidData,
	W: AcidData,
	V: AcidData,
}
pub fn initialize_Amino() -> Acid_table {
	let AminoAcids = Acid_table {
		Y: AcidData::new(
			String::from("Tyr"),
			107.0495,
			2.2,
			9.21,
			Some(10.07),
			0.71,
			Some(1490),
		),
		A: AcidData::new(String::from("Ala"), 15.0234, 2.35, 9.87, None, 0.5, None),
		R: AcidData::new(
			String::from("Arg"),
			15.0234,
			1.82,
			8.99,
			Some(12.38),
			0.5,
			None,
		),
		N: AcidData::new(String::from("Asn"), 58.0292, 2.14, 8.72, None, 0.85, None),
		D: AcidData::new(
			String::from("Asp"),
			59.0132,
			1.99,
			9.9,
			Some(3.9),
			0.85,
			None,
		),
		C: AcidData::new(
			String::from("Cys"),
			46.9995,
			1.92,
			10.7,
			Some(8.3),
			-0.02,
			Some(125),
		),
		Q: AcidData::new(String::from("Gln"), 72.0448, 2.17, 9.13, None, 0.85, None),
		E: AcidData::new(
			String::from("Glu"),
			73.0288,
			2.1,
			9.47,
			Some(4.07),
			3.63,
			None,
		),
		G: AcidData::new(String::from("Gly"), 1.0078, 2.35, 9.78, None, 1.15, None),
		H: AcidData::new(
			String::from("Asp"),
			59.0132,
			1.99,
			9.9,
			Some(3.9),
			0.85,
			None,
		),
		I: AcidData::new(String::from("Ile"), 57.0702, 2.32, 9.76, None, -1.12, None),
		L: AcidData::new(String::from("Leu"), 57.0702, 2.33, 9.74, None, -1.25, None),
		K: AcidData::new(
			String::from("Lys"),
			72.0811,
			2.16,
			9.06,
			Some(10.54),
			2.8,
			None,
		),
		M: AcidData::new(String::from("Met"), 75.0267, 2.13, 9.28, None, -0.67, None),
		F: AcidData::new(String::from("Phe"), 91.0546, 2.2, 9.31, None, -1.71, None),
		P: AcidData::new(String::from("Pro"), 41.039, 1.95, 10.64, None, 0.14, None),
		S: AcidData::new(String::from("Ser"), 31.0183, 2.19, 9.21, None, 0.14, None),
		T: AcidData::new(String::from("THR"), 45.0339, 2.09, 9.1, None, 0.25, None),
		W: AcidData::new(
			String::from("Trp"),
			130.0655,
			2.46,
			9.41,
			None,
			-2.09,
			Some(5500),
		),
		V: AcidData::new(String::from("Lys"), 43.0546, 2.39, 9.74, None, -0.46, None),
	};
	return AminoAcids;
}
