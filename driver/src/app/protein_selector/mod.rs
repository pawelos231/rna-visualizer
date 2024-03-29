//! The module that implements [`ProteinSelector`]

use std::rc::Rc;

use egui::*;
use rnalib::{Protein, ProteinMap};

use super::extras::Extras;

/// A ui widget that displays a list of proteins
/// to choose from.
#[derive(Default)]
pub struct ProteinSelector {
	/// Current page to display
	page: usize,
	/// Previously rendered page
	last_render_page: usize,
	/// A cache of paginated items
	paginated: Vec<String>,
	/// Index of the selected protein (page, entry)
	selected_index: Option<(usize, usize)>,
}

impl ProteinSelector {
	/// The number of proteins to draw per page.
	const PAGINATION: usize = 100;
	/// The background color of selected paginated results
	const LIGHT_BUTTON: Color32 = Color32::from_gray(64);
	/// The background color of paginated results
	const DARK_BUTTON: Color32 = Color32::from_gray(46);

	/// Draws self to the ui.
	pub fn show(&mut self, ui: &mut Ui, proteins: &ProteinMap) -> Option<Rc<Protein>> {
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

	/// Clears the cached pagination results.
	pub fn clear_cache(&mut self) {
		self.page = 0;
		self.paginated.clear();
		self.selected_index = None;
	}

	/// A helper function that displays an appropriate message
	/// if no proteins have been loaded.
	fn show_empty_message(&self, ui: &mut Ui, proteins: &ProteinMap) {
		if proteins.keys().len() == 0 {
			ui.centered_and_justified(|ui| ui.label("Brak białek do wyświetlenia"));
		};
	}

	/// A helper function that paginates results into chunks
	/// of [`ProteinSelector::PAGINATION`] items size each.
	fn update_pagination(&mut self, proteins: &ProteinMap) {
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

	/// A helper function that shows the controls necessary
	/// to navigate between pages.
	fn show_pagination_header(&mut self, ui: &mut Ui, proteins: &ProteinMap) {
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

	/// A helper function that shows the paginated items.
	fn show_paginated_items(
		&mut self,
		ui: &mut Ui,
		proteins: &ProteinMap,
		min_y: f32,
		max_y: f32,
	) -> Option<Rc<Protein>> {
		self.page = self.page.min(proteins.keys().len() / Self::PAGINATION);
		if self.last_render_page != self.page || self.paginated.is_empty() {
			self.update_pagination(proteins);
		}

		let mut result = None;
		let button_width = ui.available_width();

		for (index, stringed) in self.paginated.iter().enumerate() {
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
				let selected = match self.selected_index {
					Some((page, idx)) => page == self.page && idx == index,
					None => false,
				};

				let color = match selected {
					true => Self::LIGHT_BUTTON,
					false => Self::DARK_BUTTON,
				};

				if ui
					.add_sized([button_width, 30.], Button::new(stringed).fill(color))
					.clicked()
				{
					result = proteins.get_by_string(stringed.clone());
					self.selected_index = Some((self.page, index));
				}
			});

			ui.style_mut().override_text_style = None;
		}
		result
	}
}
