use egui::*;

use super::*;

pub struct AcidPainter;

impl AcidPainter {
	const ACID_SCALE: f32 = 0.33;

	pub fn show(ui: &mut Ui, cache: &mut ProteinCache, base_type: BaseType, shorthand: char) {
		cache.lazy_load(shorthand);
		cache.lazy_load_base(base_type);

		let Some(base) = cache.get_base(base_type) else { return };
		let Some(body) = cache.get(shorthand) else { return };

		let base_bottom = body.get_bounds().get_bottom()[0];
		let body_top = base.get_bounds().get_top()[0];

		let base_size = base.get_size_vec2() * Self::ACID_SCALE;
		let base_rect = base.show_scaled(ui, Self::ACID_SCALE).rect;
		let mut body_rect = base_rect;
		// body_rect.min.x += base_bottom;
		body_rect.min.y += base_size.y;
		body_rect.max.x = body_rect.min.x;
		ui.allocate_rect(body_rect, Sense::hover());
		body.show_scaled(ui, Self::ACID_SCALE);
		ui.allocate_ui_at_rect(base_rect, |_| {});
		ui.add_space(base_size.x);
	}
}
