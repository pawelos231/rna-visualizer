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
			sorted_keys.push(protein.to_string());
			proteins.insert(protein.to_string(), protein);
		}

		sorted_keys.sort_by_key(|a| a.to_string().len());

		Self {
			proteins,
			sorted_keys,
		}
	}
}
