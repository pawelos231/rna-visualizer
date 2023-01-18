use std::fmt::Display;

use itertools::Itertools;

use crate::{Codon, Nucleotide, Protein};

pub struct AminoString {
	codons: Vec<Codon>,
}

impl AminoString {
	pub fn new() -> Self {
		Self { codons: Vec::new() }
	}

	pub fn from(codons: Vec<Codon>) -> Self {
		Self { codons }
	}

	pub fn push(&mut self, codon: Codon) {
		self.codons.push(codon);
	}

	pub fn parse(source: &str) -> Vec<Self> {
		let mut source = source.replace(" ", "");
		let mut res = Vec::new();

		for _ in 0..3 {
			let chars = source.chars().collect::<Vec<_>>();
			res.push(AminoString::from(
				chars
					.chunks_exact(3)
					.map(|x| {
						x.iter()
							.map(|x| Nucleotide::parse(*x).unwrap())
							.next_tuple::<(_, _, _)>()
							.and_then(|x| Some(Codon::new(x.0, x.1, x.2)))
							.unwrap()
					})
					.collect::<Vec<_>>(),
			));
			source.remove(0);
		}

		return res;
	}

	pub fn get_proteins(&self) -> Vec<Protein> {
		let mut res = Vec::new();

		let mut current = Vec::new();
		let mut protein = false;
		for codon in &self.codons {
			let acid = codon.get_acid();

			if acid == Codon::STOP && protein {
				let mut new_codon = Vec::new();
				std::mem::swap(&mut current, &mut new_codon);
				res.push(Protein::from(new_codon));
			}
			if protein {
				current.push(codon.clone());
			}

			if acid == Codon::START {
				protein = true;
			}
		}
		return res;
	}
}

impl Display for AminoString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut string = String::new();
		let codons_length = self.codons.len();

		//sprawdzenie czy kodon ma zerową długość, jezeli tak nie ma sensu go wypisywać
		string.push('[');
		if codons_length == 0 {
			println!("Nie udało się stworzyć zadnego aminokwasu :(");
		} else {
			for i in &self.codons {
				string.push(i.get_acid());
				string.push(',');
				string.push(' ');
			}
			string.pop();
			string.pop();
		};
		string.push(']');
		f.write_str(&string)
	}
}
