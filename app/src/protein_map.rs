use rnalib::Protein;
use std::collections::{btree_map::Keys, BTreeMap};

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

	pub fn get_cloned(&self, key: &Key) -> Option<Protein> {
		self.proteins.get(key).map(ToOwned::to_owned)
	}

	pub fn keys(&self) -> Keys<Key, Protein> {
		self.proteins.keys()
	}
}
