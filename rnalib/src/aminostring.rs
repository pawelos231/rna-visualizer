use std::fmt::Display;

use crate::{Bases, Codon, Nucleotide, Protein};
use itertools::Itertools;

#[derive(Default)]
pub struct AminoString {
	codons: Vec<Codon>,
}

impl AminoString {
	pub fn from(codons: Vec<Codon>) -> Self {
		Self { codons }
	}

	pub fn push(&mut self, codon: Codon) {
		self.codons.push(codon);
	}

	pub fn parse(source: &str) -> Vec<Self> {
		let mut source = source.replace(' ', "");
		let mut res = Vec::new();
		for _ in 0..3.min(source.len()) {
			let chars = source.chars().collect::<Vec<_>>();
			res.push(AminoString::from(
				chars
					.chunks_exact(3)
					.map(|x| {
						x.iter()
							.map(|x| Nucleotide::parse(*x).unwrap())
							.next_tuple::<(_, _, _)>()
							.map(|x| Codon::new(x.0, x.1, x.2))
							.unwrap()
					})
					.collect::<Vec<_>>(),
			));
			source.remove(0);
		}

		res
	}

	pub fn get_codons(&self) -> &Vec<Codon> {
		&self.codons
	}

	pub fn get_mass(&self) -> f32 {
		let codon_len = self.codons.len() as f32;
		let sum = crate::ALPHA_MASS * codon_len + crate::H2_MASS;

		let final_mass: f32 = sum
			+ self
				.codons
				.iter()
				.map(|x| x.get_acid().map(|x| x.sc_mass).unwrap_or(0f32))
				.sum::<f32>();
		final_mass
	}

	pub fn get_isoletric_point(&self) {
		let net_val = Bases::init_bases().k;
		println!("{}", net_val)
	}

	pub fn net_charge(&self) -> f32 {
		let _c = 0.0;
		for codon in &self.codons {
			let _acid_data = codon.get_acid();
		}
		0.5
	}

	pub fn add_signum(hydrophobicity: f32) -> String {
		if hydrophobicity > 0.0 {
			format!("+{}", hydrophobicity)
		} else {
			hydrophobicity.to_string()
		}
	}

	pub fn get_phob(&self) -> String {
		let hydrophobicity = 7.9;
		let final_hydrophobicity: f32 = hydrophobicity
			+ self
				.codons
				.iter()
				.map(|x| x.get_acid().map(|x| x.sc_hbob).unwrap_or(0f32))
				.sum::<f32>();

		let mut return_val = AminoString::add_signum(final_hydrophobicity);
		return_val.push_str("Kcal * mol⁻¹");
		return_val
	}

	pub fn get_polarity(&self) -> f32 {
		0.5
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let mut result = Vec::new();

		let mut current = Vec::new();
		let mut protein = false;
		for codon in &self.codons {
			let acid = codon.get_acid_shorthand();

			if acid == Codon::STOP && protein {
				if !current.is_empty() {
					let mut new_codon = Vec::new();
					std::mem::swap(&mut current, &mut new_codon);
					result.push(Protein::from(new_codon));
				}
				protein = false;
			}

			if protein {
				current.push(codon.clone());
			}

			if acid == Codon::START {
				protein = true;
			}
		}
		result
	}
}

impl Display for AminoString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let codons = self
			.codons
			.iter()
			.map(|x| x.get_acid_shorthand())
			.join(", ");
		f.write_str(&format!("[{codons}]"))
	}
}
