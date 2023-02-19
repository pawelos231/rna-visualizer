use super::Property;
use egui::Color32;
use rnalib::AminoString;

pub struct Pi;
impl Property for Pi {
	fn get_name(&self) -> String {
		String::from("Punkt izoelektryczny")
	}

	fn get_show_negative(&self) -> bool {
		false
	}

	fn get_unit(&self) -> String {
		String::from("is")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(221, 221, 221)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_isoletric_point()
	}
}
