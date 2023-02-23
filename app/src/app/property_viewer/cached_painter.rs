use std::{mem::swap, ops::Mul, time::SystemTime};

use egui::{text::*, *};
use rnalib::AminoString;

use super::math::qerp;
use super::property::{PointsCache, Property};

pub struct CachedPainter<T: Property + 'static> {
	drawer: Option<&'static T>,
	cache: PointsCache,
	result: f32,
	prev_cache: PointsCache,
	prev_t: SystemTime,
}

impl<T: Property + 'static> CachedPainter<T> {
	pub fn new(imp: &'static T) -> Self {
		Self {
			drawer: Some(imp),
			prev_cache: [0.0; 100],
			prev_t: SystemTime::now(),
			cache: [0.0; 100],
			result: 0.0,
		}
	}

	pub fn set(&mut self, protein: &AminoString) {
		swap(&mut self.prev_cache, &mut self.cache);
		self.cache = <T as Property>::generate_cache(protein);
		self.result = <T as Property>::evaluate(protein, 1.1);
		self.prev_t = SystemTime::now();
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

		let time = SystemTime::now()
			.duration_since(self.prev_t)
			.unwrap_or_default()
			.mul(20)
			.as_secs_f32()
			.clamp(0.0, 1.0);

		if time != 1.0 {
			ui.ctx().request_repaint();
		}

		let mut animated_cache = [0.0; 100];
		(0..100).for_each(|i| {
			animated_cache[i] = qerp(self.prev_cache[i], self.cache[i], time);
		});

		drawer.show_samples(ui, animated_cache);

		ui.end_row();
	}
}
