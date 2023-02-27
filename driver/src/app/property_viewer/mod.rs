use std::rc::Rc;

use egui::*;
use rnalib::Protein;

mod property;
use property::*;

mod cached_painter;
use cached_painter::*;

mod math;

use super::extras::Extras;

pub struct PropertyViewer {
	protein: Option<Rc<Protein>>,
	hydro: CachedPainter<Hydro>,
	pi: CachedPainter<Pi>,
	extinction: CachedPainter<Extinction>,
	charge: CachedPainter<NetCharge>,
	mass: CachedPainter<Mass>,
}

impl PropertyViewer {
	pub fn set(&mut self, protein: Rc<Protein>) {
		self.hydro.set(&protein);
		self.charge.set(&protein);
		self.extinction.set(&protein);
		self.pi.set(&protein);
		self.mass.set(&protein);
		self.protein = Some(protein);
	}

	pub fn show(&mut self, ui: &mut Ui) {
		let Some(_) = &self.protein else {
			ui.centered_and_justified(|ui| ui.label("Brak danych"));
			return;
		};

		Extras::title_bar(ui, "Właściwości białka");

		Grid::new("PROTEIN_PROPERTY_GRID")
			.min_row_height(ui.available_height() / 5.0_f32 - 5.0)
			.num_columns(3)
			.striped(true)
			.show(ui, |ui| {
				self.hydro.draw(ui);
				self.pi.draw(ui);
				self.extinction.draw(ui);
				self.charge.draw(ui);
				self.mass.draw(ui);
			});
	}
}

impl Default for PropertyViewer {
	fn default() -> Self {
		Self {
			protein: Default::default(),
			hydro: CachedPainter::new(&Hydro),
			pi: CachedPainter::new(&Pi),
			extinction: CachedPainter::new(&Extinction),
			charge: CachedPainter::new(&NetCharge),
			mass: CachedPainter::new(&Mass),
		}
	}
}
