use std::rc::Rc;

use egui::*;
use rnalib::Protein;

use super::extras::Extras;

type ProteinCollection = super::ProteinMap;

#[derive(Default)]
pub struct ProteinSelector {
	page: usize,
	last_render_page: usize,
	paginated: Vec<String>,
}

impl ProteinSelector {
	const PAGINATION: usize = 100;

	pub fn show(&mut self, ui: &mut Ui, proteins: &ProteinCollection) -> Option<Rc<Protein>> {
		let mut result = None;
		let min_y = ui.cursor().min.y;
		let max_y = ui.available_height();
		self.show_pagination_header(ui, proteins);
		ScrollArea::vertical().show(ui, |ui| {
			self.show_empty_message(ui, proteins);
			result = self.show_paginated_items(ui, proteins, min_y, max_y);
		});
		result
	}

	pub fn clear_cache(&mut self) {
		self.page = 0;
		self.paginated.clear();
	}

	fn show_empty_message(&self, ui: &mut Ui, proteins: &ProteinCollection) {
		if proteins.keys().len() == 0 {
			ui.centered_and_justified(|ui| ui.label("Brak białek do wyświetlenia"));
		};
	}

	fn update_pagination(&mut self, proteins: &ProteinCollection) {
		self.paginated.clear();
		self.last_render_page = self.page;

		let mut iter = proteins.keys();
		for _ in 0..self.page * Self::PAGINATION {
			iter.next();
		}

		for protein in iter.take(Self::PAGINATION) {
			self.paginated.push(protein.0.clone());
		}
	}

	fn show_pagination_header(&mut self, ui: &mut Ui, proteins: &ProteinCollection) {
		let pages = proteins.keys().len() / Self::PAGINATION;
		if pages > 0 {
			Extras::title_bar(ui, "Wybór białka");
			ui.horizontal(|ui| {
				let button = ui.button("<");
				if button.clicked() && self.page > 0 {
					self.page -= 1;
				}

				let mut end = |ui: &mut Ui| {
					ui.horizontal(|ui| {
						ui.label("Strona");
						self.page += 1;
						ui.add(DragValue::new(&mut self.page).suffix(format!(" z {}", pages + 1)));
						if self.page > 0 {
							self.page -= 1;
						}
					});
				};

				let size = Extras::measure(ui, &mut end);
				let margin = ui.available_width()
					- size.width() - button.rect.width()
					- ui.spacing().item_spacing.x
					- 6.0;

				ui.add_space(margin / 2.0);
				end(ui);
				ui.add_space(margin / 2.0);

				if ui.button(">").clicked() {
					self.page += 1;
				}
			});
			ui.add_space(7.0);
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
	) -> Option<Rc<Protein>> {
		self.page = self.page.min(proteins.keys().len() / Self::PAGINATION);
		if self.last_render_page != self.page || self.paginated.is_empty() {
			self.update_pagination(proteins);
		}

		let mut result = None;
		let button_width = ui.available_width();

		for stringed in &self.paginated {
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
				if ui
					.add_sized([button_width, 30.], Button::new(stringed))
					.clicked()
				{
					result = proteins.get_by_string(stringed.clone());
				}
			});

			ui.style_mut().override_text_style = None;
		}
		result
	}
}
