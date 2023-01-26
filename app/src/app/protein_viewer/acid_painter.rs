use egui::*;

use super::*;

pub struct AcidPainter;

impl AcidPainter {
	const ACID_SCALE: f32 = 0.36;

	pub fn show(ui: &mut Ui, cache: &mut ProteinCache, base_type: BaseType, shorthand: char) {
		cache.lazy_load(shorthand);
		cache.lazy_load_base(base_type);

		let Some(base) = cache.get_base(base_type) else { return };
		let Some(body) = cache.get(shorthand) else { return };

		let base_size = base.get_size_vec2() * Self::ACID_SCALE;

		let body_top = body.get_bounds().get_top()[0] * Self::ACID_SCALE;

		let offset = match base_type {
			BaseType::BASE | BaseType::BASE_NO_RIGHT => 100.0,
			_ => 36.0,
		} * Self::ACID_SCALE;

		let mut width = 0.0;
		let more_width = ui
			.vertical(|ui| {
				width = base.show(ui, Self::ACID_SCALE).rect.width();
				ui.add_space(-base_size.y + 96.0 * Self::ACID_SCALE);
				ui.horizontal(|ui| {
					ui.add_space(-body_top + offset);
					body.show(ui, Self::ACID_SCALE);
				});
			})
			.response
			.rect
			.width();
		ui.add_space(f32::max(width - more_width, 0.0) - 4.0);
	}
}
