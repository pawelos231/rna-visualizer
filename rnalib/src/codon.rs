use std::fmt::{Display, Write};

use crate::{Acid, Nucleotide};

#[derive(Clone, Copy)]
pub struct Codon {
	shorthand: u8,
}

impl Codon {
	pub const STOP: char = '_';
	pub const START: char = 'M';

	pub const fn new(a: Nucleotide, b: Nucleotide, c: Nucleotide) -> Self {
		use Nucleotide::*;
		let shorthand = match (a, b, c) {
			(U, U, U | C) => b'F',
			(U, U, A | G) => b'L',
			(U, C, _) => b'S',
			(U, A, U | C) => b'Y',
			(U, A, A | G) => Self::STOP as u8,
			(U, G, U | C) => b'C',
			(U, G, A) => Self::STOP as u8,
			(U, G, G) => b'W',

			(C, U, _) => b'L',
			(C, C, _) => b'P',
			(C, A, U | C) => b'H',
			(C, A, A | G) => b'Q',
			(C, G, _) => b'R',

			(A, U, A | C | U) => b'I',
			(A, U, G) => Self::START as u8,
			(A, C, _) => b'T',
			(A, A, C | U) => b'N',
			(A, A, G | A) => b'K',
			(A, G, C | U) => b'S',
			(A, G, A | G) => b'R',

			(G, U, _) => b'V',
			(G, C, _) => b'A',
			(G, A, U | C) => b'D',
			(G, A, A | G) => b'E',
			(G, G, _) => b'G',
		};
		Self { shorthand }
	}

	pub const fn get_acid(&self) -> Option<Acid> {
		Acid::from_shorthand(self.shorthand as char)
	}

	pub const fn get_acid_shorthand(&self) -> char {
		self.shorthand as char
	}

	pub const fn get_acid_shorthand_raw(&self) -> u8 {
		self.shorthand
	}
}

impl Display for Codon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.shorthand as char)
	}
}
