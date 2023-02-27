use super::Property;
use egui::Color32;
use rnalib::AminoString;

pub struct Extinction;
impl Property for Extinction {
	fn get_name(&self) -> String {
		String::from("Współczynnik absorbcji")
	}

	fn get_show_negative(&self) -> bool {
		false
	}

	fn get_unit(&self) -> String {
		String::from("M⁻¹ * cm⁻¹")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(255, 220, 0)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_ext() as f32
	}
}
