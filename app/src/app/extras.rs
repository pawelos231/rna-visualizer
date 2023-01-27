use egui::*;

pub struct Extras;

impl Extras {
	pub fn title_bar(ui: &mut Ui, title: &str) {
		ui.label(title);
		ui.separator();
	}
}
