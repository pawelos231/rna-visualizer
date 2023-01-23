use rnalib::Protein;
use std::collections::HashMap;

#[derive(Default)]
pub struct ProteinMap {
	pub proteins: HashMap<String, Protein>,
	pub sorted_keys: Vec<String>,
}

impl ProteinMap {
	pub fn new(source: Vec<Protein>) -> Self {
		let mut proteins = HashMap::new();
		let mut sorted_keys = Vec::new();

		for protein in source {
			let key = protein.to_string();
			if proteins.insert(key.clone(), protein).is_none() {
				sorted_keys.push(key);
			}
		}

		sorted_keys.sort_unstable_by_key(|a| a.len());

		Self {
			proteins,
			sorted_keys,
		}
	}
}
