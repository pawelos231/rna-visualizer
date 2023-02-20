use std::{
	collections::{btree_map::Keys, BTreeMap},
	rc::Rc,
};

mod key;
use key::Key;

pub mod loader;

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

	pub fn from(proteins: BTreeMap<Key, Protein>) -> Self {
		Self { proteins }
	}

	pub fn parse(source: String) -> Self {
		let mut importer = ThreadedProteinLoader::default();
		importer.start(source);

		while !importer.is_ready() {}

		importer.take().unwrap()
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
