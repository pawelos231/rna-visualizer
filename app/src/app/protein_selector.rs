use egui::*;
use std::collections::HashMap;

use crate::protein_map::ProteinMap;

#[derive(Default)]
pub struct ProteinSelector {
	cached_heights: HashMap<u32, f32>,
}

impl ProteinSelector {
	pub fn show(&mut self, ui: &mut Ui, proteins: &ProteinMap) {
		let min_y = ui.cursor().min.y;
		let max_y = ui.available_height();
		ScrollArea::vertical().show(ui, |ui| {
			if proteins.sorted_keys.is_empty() {
				ui.centered_and_justified(|ui| ui.label("Brak białek do wyświetlenia"));
			};

			let mut index = 0u32;
			for protein in &proteins.sorted_keys {
				index += 1;

				let cursor = ui.cursor().min.y;
				match self.cached_heights.get(&index) {
					Some(last) => {
						if cursor < min_y - *last || cursor > max_y + 100.0 {
							ui.add_space(*last);
							continue;
						}
					}
					None => {}
				};
				let pre = ui.cursor().min.y;
				ui.add_sized([300., 30.], Button::new(protein));
				self.cached_heights.insert(index, ui.cursor().min.y - pre);
			}
		});
	}
}
