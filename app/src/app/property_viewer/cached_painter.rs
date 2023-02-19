use egui::{text::*, *};
use rnalib::AminoString;

use super::property::Property;

pub struct CachedPainter<T: Property + 'static> {
	drawer: Option<&'static T>,
	cache: [f32; 100],
	result: f32,
}

impl<T: Property + 'static> CachedPainter<T> {
	pub fn new(imp: &'static T, protein: &AminoString) -> Self {
		Self {
			drawer: Some(imp),
			cache: <T as Property>::generate_cache(protein),
			result: <T as Property>::evaluate(protein, 1.1),
		}
	}

	pub fn draw(&self, ui: &mut Ui) {
		let Some(drawer) = self.drawer else { return };
		let mut job = LayoutJob::default();

		job.append(
			&format!("{}\n", drawer.get_name()),
			0.0,
			TextFormat::simple(FontId::proportional(12.0), ui.style().visuals.text_color()),
		);

		job.append(
			&drawer.get_unit(),
			0.0,
			TextFormat::simple(
				FontId::monospace(12.0),
				ui.style().visuals.weak_text_color(),
			),
		);

		ui.label(job);
		ui.label(format!("{0:.2}", self.result));

		drawer.show_samples(ui, self.cache);

		ui.end_row();
	}
}

impl<T: Property + 'static> Default for CachedPainter<T> {
	fn default() -> Self {
		Self {
			drawer: None,
			cache: [0.0; 100],
			result: 0.0,
		}
	}
}
