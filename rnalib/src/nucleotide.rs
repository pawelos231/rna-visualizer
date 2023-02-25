use std::fmt::{Display, Write};

/// Represents one of the four nucleotides
/// commonly found in DNA sequences.
#[derive(Clone, Copy)]
pub enum Nucleotide {
	/// Guanine
	G,
	/// Uracil / Thymine
	U,
	/// Adenine
	A,
	/// Cytosine
	C,
}

impl Nucleotide {
	/// Attempt to parse and return a [`Nucleotide`] variant
	/// from a [`char`]. Ignores letter-case. If an unknown
	/// letter is passed as an argument, returns [`None`].
	/// Note that the letter 'T' will be interpreted as
	/// [`Nucleotide::A`], as per RNA to DNA conversion rules.
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

	/// Attempt to parse and return a [`Nucleotide`] variant
	/// from a [`u8`]. This function takes the ASCII table
	/// byte representation of a character representing the
	/// [`Nucleotide`]. For more, refer to [`Nucleotide::parse`].
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
