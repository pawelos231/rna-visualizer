use std::fmt::{Display, Write};

use crate::{Codon, Nucleotide, Protein};

pub struct AminoString(Vec<Codon>);

impl AminoString {
	pub fn new() -> Self {
		Self { 0: Vec::new() }
	}

	pub fn push(&mut self, codon: Codon) {
		self.0.push(codon);
	}

	pub fn parse(source: String) -> Vec<Self> {
		let mut source = source;
		let mut res = Vec::new();
		for _ in 0..source.len() % 3 + 1 {
			let mut codons = Vec::new();
			codons.reserve(source.len() / 3);
			let size = 3;
			for i in 0..source.len() / 3 {
				let start = i * size;
				let slice = source[start..start + size].chars().collect::<String>();
				let mut iter = slice.chars();

				let a = Nucleotide::parse(iter.next().unwrap());
				let b = Nucleotide::parse(iter.next().unwrap());
				let c = Nucleotide::parse(iter.next().unwrap());

				let codon = match (a, b, c) {
					(Some(a), Some(b), Some(c)) => Codon((a, b, c)),
					_ => panic!(),
				};
				codons.push(codon);
			}
			res.push(AminoString(codons));
			source.remove(0);
		}
		res
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let start = Codon::get_start();
		let stop = Codon::get_stop();

		let mut current = Vec::new();
		let mut in_protein = false;

		let mut result = Vec::new();
		for codon in &self.0 {
			let acid = codon.get_acid();

			if acid == start {
				in_protein = true;
			}

			if acid == stop {
				if in_protein {
					result.push(Protein(current.clone()));
					current.clear();
				}
				in_protein = false;
			}

			if in_protein {
				current.push(codon.clone())
			}
		}

		result
	}
}

impl Display for AminoString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for i in &self.0 {
			f.write_char(i.get_acid())?;
			f.write_char(',')?;
		}
		Ok(())
	}
}
