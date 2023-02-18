use std::{
	collections::HashMap,
	fmt::{Display, Write},
};

use crate::{Acid, Codon, Nucleotide, Protein};

#[derive(Default, Clone)]
pub struct AminoString {
	codons: Vec<Codon>,
	counts: HashMap<char, u32>,
}

impl AminoString {
	pub fn from(codons: Vec<Codon>) -> Self {
		let mut counts = HashMap::new();
		for codon in &codons {
			let short = codon.get_acid_shorthand();
			match counts.get(&short) {
				Some(k) => counts.insert(short, k + 1),
				None => counts.insert(short, 1),
			};
		}
		Self { codons, counts }
	}

	pub fn push(&mut self, codon: Codon) {
		let short = codon.get_acid_shorthand();
		match self.counts.get(&short) {
			Some(k) => self.counts.insert(short, k + 1),
			None => self.counts.insert(short, 1),
		};
		self.codons.push(codon);
	}
	pub fn slice(&self, start: usize, length: usize) -> Self {
		Self::from(
			self.codons
				.iter()
				.map(|x| *x)
				.skip(start)
				.take(length)
				.collect(),
		)
	}
	pub fn get_codon_count(&self, key: char) -> u32 {
		*self.counts.get(&key).unwrap_or(&0)
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
		self.counts.clear();
	}

	pub fn get_first(&self) -> Codon {
		self.codons[0]
	}

	pub fn get_last(&self) -> Codon {
		*self.codons.last().unwrap()
	}

	// physical properties

	pub fn get_ext(&self) -> u32 {
		let cysteines = self.get_codon_count('C');
		let cystines = (cysteines - (cysteines % 2)) / 2;

		self.get_codon_count('W') * Acid::W.extco.unwrap()
			+ self.get_codon_count('Y') * Acid::Y.extco.unwrap()
			+ cystines * Acid::C.extco.unwrap()
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
			(self.get_codon_count('D'), Acid::D.pk3.unwrap()),
			(self.get_codon_count('E'), Acid::E.pk3.unwrap()),
			(self.get_codon_count('C'), Acid::C.pk3.unwrap()),
			(self.get_codon_count('Y'), Acid::Y.pk3.unwrap()),
		];

		let counts_bases = [
			(1, self.get_last().get_acid().unwrap().pk2),
			(self.get_codon_count('K'), Acid::K.pk3.unwrap()),
			(self.get_codon_count('R'), Acid::R.pk3.unwrap()),
			(self.get_codon_count('H'), Acid::H.pk3.unwrap()),
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

	pub fn get_phob(&self, n: usize) -> f32 {
		let mut hydrophobicity = 7.9;
		for codon in &self.codons {
			let acid_data = Codon::get_acid(&codon).unwrap();
			hydrophobicity += acid_data.sc_hbob;
		}
		return hydrophobicity;
	}

	pub fn get_polarity(&self) -> f32 {
		0.5
	}

	pub fn parse(source: &str) -> Vec<Self> {
		let mut temp = [Nucleotide::A, Nucleotide::A, Nucleotide::A];
		let mut temp_idx = 0;

		let mut res = Vec::new();

		for index in 0..3.min(source.len()) {
			let mut codons = Vec::with_capacity(source.len() / 3);
			source
				.chars()
				.skip(index)
				.filter(|x| *x != ' ')
				.for_each(|x| {
					temp[temp_idx] = Nucleotide::parse(x).unwrap();
					temp_idx += 1;
					if temp_idx == 3 {
						codons.push(Codon::new(temp[0], temp[1], temp[2]));
						temp_idx = 0;
					}
				});
			res.push(AminoString::from(codons));
			temp_idx = 0;
		}
		res
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let mut result = Vec::new();

		let mut current = Vec::with_capacity(30000);
		let mut protein = false;
		for codon in &self.codons {
			let acid = codon.get_acid_shorthand();

			if acid == Codon::STOP && protein {
				if !current.is_empty() {
					result.push(Protein::from(current.clone()));
					current.clear();
				}
				protein = false;
			}

			if protein {
				current.push(*codon);
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
		for codon in &self.codons {
			f.write_char(codon.get_acid_shorthand())?;
		}
		Ok(())
	}
}
