use std::fmt::{Display, Write};

use crate::Codon;

pub struct Protein(pub Vec<Codon>);

impl Protein {
	pub fn new() -> Self {
		Self { 0: Vec::new() }
	}
}

impl Display for Protein {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in &self.0 {
			f.write_char(i.get_acid())?;
			f.write_char(',')?;
		}
		Ok(())
	}
}
