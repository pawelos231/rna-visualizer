use egui::*;

use crate::protein_map::ProteinMap;

#[derive(Default)]
pub struct ProteinSelector;

impl ProteinSelector {
	pub fn show(&mut self, ui: &mut Ui, proteins: &ProteinMap) {
		let min_y = ui.cursor().min.y;
		let max_y = ui.available_height();
		ScrollArea::vertical().show(ui, |ui| {
			if proteins.sorted_keys.is_empty() {
				ui.centered_and_justified(|ui| ui.label("Brak białek do wyświetlenia"));
			};

			for protein in &proteins.sorted_keys {
				let old_clip_rect = ui.clip_rect();

				let cursor = ui.cursor().min.y;

				ui.set_clip_rect(Rect::NOTHING);
				let rect = ui.add_sized([300., 30.], Button::new(protein)).rect;
				ui.set_clip_rect(old_clip_rect);

				if cursor < min_y - rect.height() || cursor > max_y + 100.0 {
					continue;
				}

				ui.allocate_ui_at_rect(rect, |ui| ui.add_sized([300., 30.], Button::new(protein)));
			}
		});
	}
}
