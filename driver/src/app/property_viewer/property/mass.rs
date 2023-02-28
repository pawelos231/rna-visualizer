//! The module that defines [`Mass`]

use super::Property;
use rnalib::AminoString;

pub struct Mass;
impl Property for Mass {
	fn get_name(&self) -> String {
		String::from("Masa")
	}

	fn get_unit(&self) -> String {
		String::from("Dalton")
	}

	fn get_show_negative(&self) -> bool {
		false
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_mass()
	}
}
