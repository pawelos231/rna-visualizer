use std::fmt::{Display, Write};

use crate::{Acid, Nucleotide};

#[derive(Clone)]
pub struct Codon {
	nucleotides: (Nucleotide, Nucleotide, Nucleotide),
}

impl Codon {
	pub const STOP: char = '_';
	pub const START: char = 'M';

	pub fn new(a: Nucleotide, b: Nucleotide, c: Nucleotide) -> Self {
		Self {
			nucleotides: (a, b, c),
		}
	}

	pub fn get_acid(&self) -> Option<Acid> {
		return Acid::from_shorthand(self.get_acid_shorthand());
	}

	pub fn get_acid_shorthand(&self) -> char {
		use Nucleotide::*;
		match self.nucleotides {
			(U, U, U | C) => 'F',
			(U, U, A | G) => 'L',
			(U, C, _) => 'S',
			(U, A, U | C) => 'Y',
			(U, A, A | G) => Self::STOP,
			(U, G, U | C) => 'C',
			(U, G, A) => Self::STOP,
			(U, G, G) => 'W',

			(C, U, _) => 'L',
			(C, C, _) => 'P',
			(C, A, U | C) => 'H',
			(C, A, A | G) => 'Q',
			(C, G, _) => 'R',

			(A, U, A | C | U) => 'I',
			(A, U, G) => Self::START,
			(A, C, _) => 'T',
			(A, A, C | U) => 'N',
			(A, A, G | A) => 'K',
			(A, G, C | U) => 'S',
			(A, G, A | G) => 'R',

			(G, U, _) => 'V',
			(G, C, _) => 'A',
			(G, A, U | C) => 'D',
			(G, A, A | G) => 'E',
			(G, G, _) => 'G',
		}
	}
}

impl Display for Codon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.get_acid_shorthand())
	}
}
