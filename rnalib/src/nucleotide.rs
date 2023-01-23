use std::fmt::{Display, Write};

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
		match from {
			'G' | 'g' => Some(G),
			'U' | 'T' | 'u' | 't' => Some(U),
			'A' | 'a' => Some(A),
			'C' | 'c' => Some(C),
			_ => None,
		}
	}
}

impl Display for Nucleotide {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self {
			G => 'G',
			U => 'U',
			A => 'A',
			C => 'C',
		})
	}
}
