//! The module that implements [`Codon`]

use std::fmt::{Display, Write};

use crate::{Acid, Nucleotide};

/// Represents a single RNA sequence codon.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Codon {
	STOP,
	M,
	F,
	L,
	S,
	Y,
	C,
	W,
	P,
	H,
	Q,
	R,
	I,
	T,
	N,
	K,
	V,
	A,
	D,
	E,
	G,
}

impl Codon {
	/// Returns the enum representation of the STOP codon.
	///
	/// This is the same as [`Codon::STOP`].
	pub const fn stop() -> Codon {
		Codon::STOP
	}

	/// Returns the enum representation of the START codon.
	///
	/// This is the same as [`Codon::M`].
	pub const fn start() -> Codon {
		Codon::M
	}

	/// Constructs a [`Codon`] from three sequential instances
	/// of [`Nucleotide`].
	#[doc=include_str!("doc/amino_wheel.svg")]
	pub const fn new(a: Nucleotide, b: Nucleotide, c: Nucleotide) -> Self {
		use Nucleotide::*;
		match (a, b, c) {
			(U, U, U | C) => Codon::F,
			(U, U, A | G) => Codon::L,
			(U, C, _) => Codon::S,
			(U, A, U | C) => Codon::Y,
			(U, A, A | G) => Codon::STOP,
			(U, G, U | C) => Codon::C,
			(U, G, A) => Codon::STOP,
			(U, G, G) => Codon::W,

			(C, U, _) => Codon::L,
			(C, C, _) => Codon::P,
			(C, A, U | C) => Codon::H,
			(C, A, A | G) => Codon::Q,
			(C, G, _) => Codon::R,

			(A, U, A | C | U) => Codon::I,
			(A, U, G) => Codon::M,
			(A, C, _) => Codon::T,
			(A, A, C | U) => Codon::N,
			(A, A, G | A) => Codon::K,
			(A, G, C | U) => Codon::S,
			(A, G, A | G) => Codon::R,

			(G, U, _) => Codon::V,
			(G, C, _) => Codon::A,
			(G, A, U | C) => Codon::D,
			(G, A, A | G) => Codon::E,
			(G, G, _) => Codon::G,
		}
	}

	/// Returns physical properties of an amino acid
	/// coded by this [`Codon`].
	///
	/// Returns [`None`] if this [`Codon`] instance is [`Codon::STOP`].
	pub const fn get_acid(&self) -> Option<Acid> {
		Acid::from_shorthand(self.get_acid_shorthand())
	}

	/// Returns the single-letter shorthand uniquely
	/// identifying the amino acid encoded by this
	/// [`Codon`]. Defaults to uppercase letter values.
	///
	/// Returns '_' if this instance of [`Codon`] is [`Codon::STOP`].
	pub const fn get_acid_shorthand(&self) -> char {
		match self {
			Codon::STOP => '_',
			Codon::M => 'M',
			Codon::F => 'F',
			Codon::L => 'L',
			Codon::S => 'S',
			Codon::Y => 'Y',
			Codon::C => 'C',
			Codon::W => 'W',
			Codon::P => 'P',
			Codon::H => 'H',
			Codon::Q => 'Q',
			Codon::R => 'R',
			Codon::I => 'I',
			Codon::T => 'T',
			Codon::N => 'N',
			Codon::K => 'K',
			Codon::V => 'V',
			Codon::A => 'A',
			Codon::D => 'D',
			Codon::E => 'E',
			Codon::G => 'G',
		}
	}
}

impl Display for Codon {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(self.get_acid_shorthand())
	}
}
