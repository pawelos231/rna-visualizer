use egui::*;

use super::*;

pub struct AcidPainter {
	pub scale: f32,
	pub flip: bool,
}

impl Default for AcidPainter {
	fn default() -> Self {
		Self {
			scale: 0.34,
			flip: true,
		}
	}
}

impl AcidPainter {
	pub fn show(
		&mut self,
		ui: &mut Ui,
		cache: &mut ViewerCache,
		base_type: BaseType,
		shorthand: char,
		next_shorthand: Option<char>,
	) {
		cache.lazy_load(shorthand);
		cache.lazy_load_base(base_type);

		let Some(base) = cache.get_base(base_type) else { return };
		let Some(body) = cache.get(shorthand) else { return };

		let base_size = base.get_size_vec2() * self.scale;

		if !ui
			.clip_rect()
			.expand(150.0 * self.scale)
			.contains(ui.next_widget_position())
		{
			self.flip = !self.flip;
			ui.add_space(base_size.x);
			ui.add_space(60.0 * self.scale);
			return;
		}

		let body = match self.flip {
			true => body.get_flipped(),
			false => body.get(),
		};
		let body_size = body.get_size_vec2() * self.scale;
		let body_top = body.get_bounds().get_top()[0] * self.scale;

		let offset = match base_type {
			BaseType::BASE | BaseType::BASE_NO_RIGHT => 100.0,
			_ => 36.0,
		} * self.scale;

		let mut base_rect = Rect::NOTHING;

		if !self.flip {
			ui.vertical(|ui| {
				base_rect = base.show(ui, self.scale).rect;
				ui.add_space(-base_size.y + 66.0 * self.scale);
				ui.horizontal(|ui| {
					ui.add_space(-body_top + offset);
					body.show_no_alloc(ui, self.scale);
				});
			});
		} else {
			ui.add_space(-body_top + offset);
			ui.vertical(|ui| {
				ui.add_space(72.0 * self.scale);
				body.show_size(ui, Vec2::new(body_size.x, body_size.y * -1.0));
				ui.add_space(body_size.y + 18.0 * self.scale);
				ui.horizontal(|ui| {
					ui.add_space(body_top - offset);
					base_rect = base
						.show_size(ui, Vec2::new(base_size.x, base_size.y * -1.0))
						.rect;
				});
			});
		}

		if let Some(next_shorthand) = next_shorthand {
			let link_type = match next_shorthand {
				'p' | 'P' => BaseType::BASE_P_LINK,
				_ => BaseType::BASE_LINK,
			};

			cache.lazy_load_base(link_type);
			if let Some(link) = cache.get_base(link_type) {
				ui.add_space(-30.0 * self.scale);
				ui.vertical(|ui| {
					ui.add_space(50.0 * self.scale);
					link.show(ui, self.scale);
				});
			}
		}

		ui.allocate_rect(base_rect, Sense::hover());
		ui.add_space(60.0 * self.scale);

		self.flip = !self.flip;
	}
}
