use std::fmt::{Display, Write};

mod counts;
use counts::Counts;

use crate::{Acid, Codon, Protein};

#[derive(Default, Clone)]
pub struct AminoString {
	codons: Vec<Codon>,
	counts: Counts,
}

impl AminoString {
	pub fn from(codons: Vec<Codon>) -> Self {
		let mut counts = Counts::default();
		for codon in &codons {
			counts.add(codon);
		}
		Self { codons, counts }
	}

	pub fn push(&mut self, codon: Codon) {
		self.counts.add(&codon);
		self.codons.push(codon);
	}

	pub fn slice(&self, start: usize, length: usize) -> Self {
		Self::from(
			self.codons
				.iter()
				.copied()
				.skip(start)
				.take(length)
				.collect(),
		)
	}

	pub fn len(&self) -> usize {
		self.codons.len()
	}

	pub fn is_empty(&self) -> bool {
		self.codons.len() == 0
	}

	pub fn get_codons(&self) -> &Vec<Codon> {
		&self.codons
	}

	pub fn clear(&mut self) {
		self.codons.clear();
		self.counts = Counts::default();
	}

	pub fn get_first(&self) -> Codon {
		self.codons[0]
	}

	pub fn get_last(&self) -> Codon {
		*self.codons.last().unwrap()
	}

	pub fn get_ext(&self) -> u32 {
		let cysteines = self.counts.get_c();
		let cystines = (cysteines - (cysteines % 2)) / 2;

		self.counts.get_w() as u32 * Acid::W.extco.unwrap()
			+ self.counts.get_y() as u32 * Acid::Y.extco.unwrap()
			+ cystines as u32 * Acid::C.extco.unwrap()
	}

	pub fn get_mass(&self) -> f32 {
		const H2_MASS: f32 = 18.0105;
		self.codons
			.iter()
			.map(|x| x.get_acid().map(|x| x.sc_mass).unwrap_or(0f32))
			.sum::<f32>()
			+ H2_MASS
	}
	pub fn get_neutral_charge(&self) -> f32 {
		self.net_charge(7.0)
	}
	pub fn get_isoletric_point(&self) -> f32 {
		let mut pi = 0.0;
		for ph in (0..1400).map(|x| x as f32 * 0.01) {
			pi = ph;
			if self.net_charge(ph) <= 0.0 {
				break;
			}
		}
		pi
	}

	pub fn net_charge(&self, ph: f32) -> f32 {
		let mut result = 0.0;

		let counts_acids = [
			(1, self.get_first().get_acid().unwrap().pk1),
			(self.counts.get_d(), Acid::D.pk3.unwrap()),
			(self.counts.get_e(), Acid::E.pk3.unwrap()),
			(self.counts.get_c(), Acid::C.pk3.unwrap()),
			(self.counts.get_y(), Acid::Y.pk3.unwrap()),
		];

		let counts_bases = [
			(1, self.get_last().get_acid().unwrap().pk2),
			(self.counts.get_k(), Acid::K.pk3.unwrap()),
			(self.counts.get_r(), Acid::R.pk3.unwrap()),
			(self.counts.get_h(), Acid::H.pk3.unwrap()),
		];

		for (count, pk) in counts_acids {
			let count = count as f32;
			if count > 0.0 {
				result += -count / (1.0 + f32::powf(10.0, pk - ph));
			}
		}

		for (count, pk) in counts_bases {
			let count = count as f32;
			if count > 0.0 {
				result += count / (1.0 + f32::powf(10.0, ph - pk));
			}
		}

		result
	}

	pub fn get_phob(&self, _n: usize) -> f32 {
		let mut hydrophobicity = 7.9;
		for codon in &self.codons {
			let acid_data = Codon::get_acid(codon).unwrap();
			hydrophobicity += acid_data.sc_phob;
		}
		hydrophobicity
	}

	pub fn get_polarity(&self) -> f32 {
		0.5
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let mut result = Vec::new();

		let mut current = Vec::with_capacity(30000);
		let mut protein = false;
		for codon in &self.codons {
			if *codon == Codon::STOP && protein {
				if !current.is_empty() {
					result.push(Protein::from(current.clone()));
					current.clear();
				}
				protein = false;
			}

			if protein {
				current.push(*codon);
			}

			if *codon == Codon::start() {
				protein = true;
			}
		}
		result
	}
}

impl Display for AminoString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for codon in &self.codons {
			f.write_char(codon.get_acid_shorthand())?;
		}
		Ok(())
	}
}
