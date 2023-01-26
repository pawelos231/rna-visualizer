use std::{
	collections::{btree_map::Keys, BTreeMap},
	thread::scope,
};

use crate::*;

#[derive(Default)]
pub struct ProteinMap {
	proteins: BTreeMap<Key, Protein>,
}

pub struct Key(pub String);

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		self.0.len() == other.0.len() && self.0 == other.0
	}
}

impl Eq for Key {}

impl PartialOrd for Key {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		use std::cmp::Ordering::*;
		match self.0.len().partial_cmp(&other.0.len()) {
			None => None,
			Some(ord) => match ord {
				Less => Some(Less),
				Greater => Some(Greater),
				Equal => self.0.partial_cmp(&other.0),
			},
		}
	}
}

impl Ord for Key {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		use std::cmp::Ordering::*;
		match self.0.len().cmp(&other.0.len()) {
			Less => Less,
			Greater => Greater,
			Equal => self.0.cmp(&other.0),
		}
	}
}

impl ProteinMap {
	pub fn new(source: Vec<Protein>) -> Self {
		let mut proteins = BTreeMap::new();
		for protein in source {
			proteins.insert(Key(protein.to_string()), protein);
		}

		Self { proteins }
	}

	pub fn parse_multithreaded(source: &str) -> Self {
		let mut proteins = BTreeMap::new();

		scope(|scope| {
			let threads = [
				scope.spawn(|| Self::load_skip(source, 0)),
				scope.spawn(|| Self::load_skip(source, 1)),
				scope.spawn(|| Self::load_skip(source, 2)),
			];

			for thread in threads {
				if let Ok(mut data) = thread.join() {
					proteins.append(&mut data);
				}
			}
		});

		Self { proteins }
	}

	fn load_skip(source: &str, skip: usize) -> BTreeMap<Key, Protein> {
		const STOP: u8 = Codon::STOP as u8;
		const START: u8 = Codon::START as u8;
		const SPACE: u8 = b' ';

		let mut result = BTreeMap::new();
		let mut current = Protein::from(Vec::with_capacity(30000));
		let mut protein = false;

		let mut iter = source
			.as_bytes()
			.iter()
			.filter(|&&x| x != SPACE)
			.map(|&x| Nucleotide::parse_raw(x));

		for _ in 0..skip {
			iter.next();
		}

		while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {
			let codon = Codon::new(a, b, c);
			let acid = codon.get_acid_shorthand_raw();

			if protein && acid == STOP {
				if !current.is_empty() {
					result.insert(Key(current.to_string()), current.clone());
					current.get_codons_mut().clear();
				}
				protein = false;
			}

			if protein {
				current.push(codon);
			}

			if acid == START {
				protein = true;
			}
		}

		result
	}

	pub fn get_cloned(&self, key: &Key) -> Option<Protein> {
		self.proteins.get(key).map(ToOwned::to_owned)
	}

	pub fn keys(&self) -> Keys<Key, Protein> {
		self.proteins.keys()
	}
}
