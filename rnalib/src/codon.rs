use std::fmt::{Display, Write};

use crate::Nucleotide;

#[derive(Clone)]
pub struct Codon(pub (Nucleotide, Nucleotide, Nucleotide));

impl Codon {
	pub const fn get_stop() -> char {
		'_'
	}

	pub const fn get_start() -> char {
		'M'
	}

	pub fn get_acid(&self) -> char {
		use Nucleotide::*;
		match self.0 {
			(U, U, U | C) => 'F',
			(U, U, A | G) => 'L',
			(U, C, _) => 'S',
			(U, A, U | C) => 'Y',
			(U, A, A | G) => Self::get_stop(),
			(U, G, U | C) => 'C',
			(U, G, A) => Self::get_stop(),
			(U, G, G) => 'W',

			(C, U, _) => 'L',
			(C, C, _) => 'P',
			(C, A, U | C) => 'H',
			(C, A, A | G) => 'Q',
			(C, G, _) => 'R',

			(A, U, A | C | U) => 'I',
			(A, U, G) => 'M',
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
		.into()
	}
}

impl Display for Codon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.get_acid())
	}
}
