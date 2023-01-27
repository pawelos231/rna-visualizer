use egui::*;

use super::*;

pub struct AcidPainter {
	pub scale: f32,
	flip: bool,
}

impl Default for AcidPainter {
	fn default() -> Self {
		Self {
			scale: 0.34,
			flip: false,
		}
	}
}

impl AcidPainter {
	pub fn show(
		&mut self,
		ui: &mut Ui,
		cache: &mut ProteinCache,
		base_type: BaseType,
		shorthand: char,
	) {
		cache.lazy_load(shorthand);
		cache.lazy_load_base(base_type);

		let Some(base) = cache.get_base(base_type) else { return };
		let Some(body) = cache.get(shorthand) else { return };

		let base_size = base.get_size_vec2() * self.scale;
		let body_top = body.get_bounds().get_top()[0] * self.scale;

		let offset = match base_type {
			BaseType::BASE | BaseType::BASE_NO_RIGHT => 100.0,
			_ => 36.0,
		} * self.scale;

		let flip_scale = match self.flip {
			true => 1.0,
			false => -1.0,
		};
		base.show_size(ui, Vec2::new(base_size.x, base_size.y * flip_scale));
		/*let mut width = 0.0;
		let more_width = ui
			.vertical(|ui| {
				width = base.show(ui, self.scale).rect.width();
				ui.add_space(-base_size.y + 96.0 * self.scale);
				ui.horizontal(|ui| {
					ui.add_space(-body_top + offset);
					body.show(ui, self.scale);
				});
			})
			.response
			.rect
			.width();
		ui.add_space(f32::max(width - more_width, 0.0) - 4.0);*/
	}
}
