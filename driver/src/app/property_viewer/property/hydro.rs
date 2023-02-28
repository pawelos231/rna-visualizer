//! The module that defines [`Hydro`]

use super::Property;
use egui::Color32;
use rnalib::AminoString;

pub struct Hydro;
impl Property for Hydro {
	fn get_name(&self) -> String {
		String::from("Indeks hydrofobowy")
	}

	fn get_unit(&self) -> String {
		String::from("Kcal * mol⁻¹")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(0, 116, 217)
	}

	fn evaluate(protein: &AminoString, x: f32) -> f32 {
		let n = (x * protein.len() as f32) as usize;
		protein.get_phob(n)
	}
}
