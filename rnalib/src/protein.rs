//! The module that implements [`Protein`]

use std::{
	fmt::Display,
	ops::{Deref, DerefMut},
};

use crate::{AminoString, Codon};

/// An abstraction over [`AminoString`] that is
/// guaranteed to be a proper protein sequence.
#[derive(Clone)]
pub struct Protein {
	string: AminoString,
}

impl Protein {
	/// Constructs a [`Protein`] from a vector of
	/// [`Codon`] instances.
	pub fn from(codons: Vec<Codon>) -> Self {
		Self {
			string: AminoString::from(codons),
		}
	}
}

impl Deref for Protein {
	type Target = AminoString;

	fn deref(&self) -> &Self::Target {
		&self.string
	}
}

impl DerefMut for Protein {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.string
	}
}

impl Display for Protein {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}
