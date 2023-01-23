use egui::*;

type ProteinCollection = super::ProteinMap;

#[derive(Default)]
pub struct ProteinSelector {
	page: usize,
}

impl ProteinSelector {
	const PAGINATION: usize = 100;

	pub fn show(&mut self, ui: &mut Ui, proteins: &ProteinCollection) {
		let min_y = ui.cursor().min.y;
		let max_y = ui.available_height();
		ScrollArea::vertical().show(ui, |ui| {
			self.show_empty_message(ui, proteins);
			self.show_pagination_header(ui, proteins);
			self.show_paginated_items(ui, proteins, min_y, max_y);
		});
	}

	fn show_empty_message(&self, ui: &mut Ui, proteins: &ProteinCollection) {
		if proteins.keys().len() == 0 {
			ui.centered_and_justified(|ui| ui.label("Brak białek do wyświetlenia"));
		};
	}

	fn show_pagination_header(&mut self, ui: &mut Ui, proteins: &ProteinCollection) {
		let pages = proteins.keys().len() / Self::PAGINATION;
		if pages > 0 {
			ui.horizontal(|ui| {
				let button = ui.button("<");
				if button.clicked() && self.page > 0 {
					self.page -= 1;
				}

				let text = format!("Strona {}/{}", self.page + 1, pages + 1);
				ui.add_sized(
					[ui.available_width() - button.rect.width() - 12.0, 20.],
					Label::new(text),
				);

				if ui.button(">").clicked() {
					self.page += 1;
				}
				self.page = self.page.min(pages);
			});
		} else {
			self.page = 0;
		}
	}

	fn show_paginated_items(
		&mut self,
		ui: &mut Ui,
		proteins: &ProteinCollection,
		min_y: f32,
		max_y: f32,
	) {
		let button_width = ui.available_width();
		let iter = proteins.keys().skip(self.page * Self::PAGINATION);
		for protein in iter.take(Self::PAGINATION) {
			let stringed = &protein.0;
			let old_clip_rect = ui.clip_rect();

			let cursor = ui.cursor().min.y;

			ui.style_mut().override_text_style = Some(TextStyle::Monospace);

			ui.set_clip_rect(Rect::NOTHING);
			let rect = ui
				.add_sized([button_width, 30.], Button::new(stringed))
				.rect;
			ui.set_clip_rect(old_clip_rect);

			if cursor < min_y - rect.height() || cursor > max_y + 100.0 {
				continue;
			}

			ui.allocate_ui_at_rect(rect, |ui| {
				ui.add_sized([button_width, 30.], Button::new(stringed))
			});

			ui.style_mut().override_text_style = None;
		}
	}
}
