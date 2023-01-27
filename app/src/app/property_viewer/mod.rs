use std::rc::Rc;

use egui::*;
use rnalib::Protein;

mod property;
use property::*;

use super::extras::Extras;

#[derive(Default)]
pub struct PropertyViewer {
	pub protein: Option<Rc<Protein>>,
}

impl PropertyViewer {
	pub fn show(&mut self, ui: &mut Ui) {
		let Some(protein) = &self.protein else {
			ui.centered_and_justified(|ui| ui.label("Brak danych"));
			return;
		};

		Extras::title_bar(ui, "Właściwości białka");

		let properties = [
			("Indeks hydrofobowy", None),
			("Indeks pH", None),
			("Polarność", None),
			("Punkt izoelektryczny", None),
			("Poczytalność", Some(Sanity)),
		];

		Grid::new("PROTEIN_PROPERTY_GRID")
			.min_row_height(ui.available_height() / properties.len() as f32 - 5.0)
			.num_columns(2)
			.striped(true)
			.show(ui, |ui| {
				for property in properties {
					ui.label(property.0);
					if let Some(evaluator) = property.1 {
						evaluator.show(protein, ui, 0.0, 5.0);
					} else {
						ui.horizontal(|ui| {
							ui.centered_and_justified(|ui| {
								ui.label("Brak danych ☹");
							});
						});
					}
					ui.end_row();
				}
			});
	}
}
