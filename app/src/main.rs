use std::fmt::{Display, Write};

#[derive(Clone)]
pub enum Nucleotide {
	G,
	U,
	A,
	C,
}

impl Nucleotide {
	pub fn parse(from: char) -> Option<Self> {
		use Nucleotide::*;
		match from.to_ascii_uppercase() {
			'G' => Some(G),
			'U' | 'T' => Some(U),
			'A' => Some(A),
			'C' => Some(C),
			_ => None,
		}
	}
}

#[derive(Clone)]
pub struct Codon((Nucleotide, Nucleotide, Nucleotide));

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

pub struct Protein(Vec<Codon>);

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

fn main() {
	let mut buffer = String::new();
	std::io::stdin()
		.read_line(&mut buffer)
		.expect("konsola sie wyjebała");
	buffer = buffer.trim().into();

	let amino_strings = AminoString::parse(buffer);
	for amino in amino_strings {
		println!("{amino}:");
		for protein in amino.get_proteins() {
			println!("{protein}");
		}
		println!("=========================");
	}
}
