use std::{fmt::Display, ops::Deref};

use crate::{AminoString, Codon};

pub struct Protein {
	string: AminoString,
}

impl Protein {
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

impl Display for Protein {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}
