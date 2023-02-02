use std::rc::Rc;

use egui::{text::LayoutJob, *};
use rnalib::Protein;

mod property;
use property::*;

use super::extras::Extras;

type ShowPtr = fn(p: &Protein, ui: &mut Ui, s: f32, e: f32);

#[rustfmt::skip]
const PROPERTIES: [(&str, &str, Option<ShowPtr>); 5] = [
	("Indeks hydrofobowy",		"Kcal * mol⁻¹",		Some(Hydro::show)),
	("Indeks pH",				"_",				None),
	("Polarność",				"_",				None),
	("Punkt izoelektryczny",	"_",				None),
	("Poczytalność", 			"°C",				Some(Sanity::show)),
];

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

		Grid::new("PROTEIN_PROPERTY_GRID")
			.min_row_height(ui.available_height() / PROPERTIES.len() as f32 - 5.0)
			.num_columns(2)
			.striped(true)
			.show(ui, |ui| {
				for property in PROPERTIES {
					let mut job = LayoutJob::default();
					job.append(
						&format!("{}\n", property.0),
						0.0,
						TextFormat::simple(
							FontId::proportional(12.0),
							ui.style().visuals.text_color(),
						),
					);
					job.append(
						property.1,
						0.0,
						TextFormat::simple(
							FontId::monospace(12.0),
							ui.style().visuals.weak_text_color(),
						),
					);
					ui.label(job);
					if let Some(evaluator) = property.2 {
						evaluator(protein, ui, 0.0, 5.0);
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
