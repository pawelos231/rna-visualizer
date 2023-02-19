use std::{
	collections::{btree_map::Keys, BTreeMap},
	rc::Rc,
	thread::scope,
};

mod key;
use key::Key;

use crate::*;

#[derive(Default)]
pub struct ProteinMap {
	proteins: BTreeMap<Key, Protein>,
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
		let mut current = Vec::with_capacity(30000);
		let mut current_str = String::with_capacity(30000);
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
					current.shrink_to_fit();
					current_str.shrink_to_fit();
					result.insert(Key(current_str), Protein::from(current));
					current = Vec::with_capacity(30000);
					current_str = String::with_capacity(30000);
				}
				protein = false;
			}

			if protein {
				current_str.push(codon.get_acid_shorthand());
				current.push(codon);
			}

			if acid == START {
				protein = true;
			}
		}

		result
	}

	pub fn get(&self, key: &Key) -> Option<Rc<Protein>> {
		self.proteins.get(key).map(|x| Rc::new(x.clone()))
	}

	pub fn get_by_string(&self, key: String) -> Option<Rc<Protein>> {
		self.proteins.get(&Key(key)).map(|x| Rc::new(x.clone()))
	}

	pub fn keys(&self) -> Keys<Key, Protein> {
		self.proteins.keys()
	}
}
