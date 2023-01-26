use std::fmt::{Display, Write};

#[derive(Clone, Copy)]
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

	pub const fn parse_raw(from: u8) -> Option<Self> {
		use Nucleotide::*;
		match from {
			b'G' | b'g' => Some(G),
			b'U' | b'T' | b'u' | b't' => Some(U),
			b'A' | b'a' => Some(A),
			b'C' | b'c' => Some(C),
			_ => None,
		}
	}
}

impl Display for Nucleotide {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self {
			Self::G => 'G',
			Self::U => 'U',
			Self::A => 'A',
			Self::C => 'C',
		})
	}
}
