//! The module that implements [`ProteinMap`]

use std::{
	collections::{btree_map::Keys, BTreeMap},
	rc::Rc,
};

mod key;
use key::Key;

pub mod loader;

use crate::*;

/// Holds a collection of proteins in optimal
/// data format.
#[derive(Default)]
pub struct ProteinMap {
	proteins: BTreeMap<Key, Protein>,
}

impl ProteinMap {
	/// Constructs a [`ProteinMap`] from a vector of
	/// [`Protein`] instances.
	pub fn new(source: Vec<Protein>) -> Self {
		let mut proteins = BTreeMap::new();
		for protein in source {
			proteins.insert(Key(protein.to_string()), protein);
		}

		Self { proteins }
	}

	/// Constructs a [`ProteinMap`] from a [`BTreeMap`].
	pub fn from(proteins: BTreeMap<Key, Protein>) -> Self {
		Self { proteins }
	}

	/// Parses a string containing an RNA sequence into a [`ProteinMap`].
	/// Uses a [`ThreadedProteinLoader`] under the hood.
	///
	/// Returns [`Err`] if any issue was encountered while
	/// parsing.
	pub fn parse(source: String) -> Result<Self, String> {
		let mut importer = ThreadedProteinLoader::default();
		importer.start(source);

		while !importer.is_ready() {}

		match importer.take() {
			Some(x) => Ok(x),
			None => Err(String::from("Wystąpił problem podczas odczytywania pliku. Upewnij się, że dane wejściowe są poprawne, lub skorzystaj z opcji 'wytnij niepoprawne znaki'.")),
		}
	}

	/// Returns an [`Rc`] wrapped [`Protein`] from this map,
	/// given its key.
	///
	/// Returns [`None`] if such a protein does not exist
	/// in this map.
	pub fn get(&self, key: &Key) -> Option<Rc<Protein>> {
		self.proteins.get(key).map(|x| Rc::new(x.clone()))
	}

	/// Returns an [`Rc`] wrapped [`Protein`] from this map,
	/// given its textual representation.
	///
	/// See [`ProteinMap::get`] for more.
	pub fn get_by_string(&self, key: String) -> Option<Rc<Protein>> {
		self.proteins.get(&Key(key)).map(|x| Rc::new(x.clone()))
	}

	/// Return a list of all the keys present in this map.
	pub fn keys(&self) -> Keys<Key, Protein> {
		self.proteins.keys()
	}
}
