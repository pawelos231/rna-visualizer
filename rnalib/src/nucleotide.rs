#[derive(Clone)]
pub enum Nucleotide {
	G,
	U,
	A,
	C,
}

impl Nucleotide {
	pub const fn parse(from: char) -> Option<Self> {
		use Nucleotide::*;
		match from.to_ascii_uppercase() {
			'G' => Some(G),
			'U' | 'T' => Some(U),
			'A' => Some(A),
			'C' => Some(C),
			_ => None,
		}
	}
}
