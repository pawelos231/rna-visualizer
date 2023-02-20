use super::Property;
use egui::Color32;
use rnalib::AminoString;

pub struct NetCharge;
impl Property for NetCharge {
	fn get_name(&self) -> String {
		String::from("Suma ładunków")
	}

	fn get_unit(&self) -> String {
		String::from("zakłada pH 7")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(52, 186, 186)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_neutral_charge()
	}
}
