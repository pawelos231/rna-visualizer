use egui::{text::*, *};
use rnalib::AminoString;

use super::property::{PointsCache, Property};

pub struct CachedPainter<T: Property + 'static> {
	drawer: Option<&'static T>,
	cache: PointsCache,
	result: f32,
}

impl<T: Property + 'static> CachedPainter<T> {
	pub fn new(imp: &'static T) -> Self {
		Self {
			drawer: Some(imp),
			cache: [0.0; 100],
			result: 0.0,
		}
	}

	pub fn set(&mut self, protein: &AminoString) {
		self.cache = <T as Property>::generate_cache(protein);
		self.result = <T as Property>::evaluate(protein, 1.1);
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
