use std::rc::Rc;

use egui::{text::LayoutJob, *};
use rnalib::Protein;

mod property;
use property::*;

use super::extras::Extras;

#[derive(Default)]
pub struct PropertyViewer {
	protein: Option<Rc<Protein>>,
	hydro: CachedDrawer<Hydro>,
	pi: CachedDrawer<Pi>,
	extinction: CachedDrawer<Extinction>,
	charge: CachedDrawer<NetCharge>,
	mass: CachedDrawer<Mass>,
}

struct CachedDrawer<T: Property + 'static> {
	drawer: Option<&'static T>,
	cache: [f32; 100],
}

impl<T: Property + 'static> CachedDrawer<T> {
	pub fn new(imp: &'static T, protein: &Protein) -> Self {
		Self {
			drawer: Some(imp),
			cache: <T as Property>::generate_cache(protein),
		}
	}

	pub fn draw(&self, ui: &mut Ui) {
		let Some(drawer) = self.drawer else { return };
		let mut job = LayoutJob::default();
		job.append(
			&format!("{}\n", drawer.get_name()),
			0.0,
			TextFormat::simple(FontId::proportional(12.0), ui.style().visuals.text_color()),
		);
		job.append(
			&drawer.get_unit(),
			0.0,
			TextFormat::simple(
				FontId::monospace(12.0),
				ui.style().visuals.weak_text_color(),
			),
		);
		ui.label(job);
		drawer.show_samples(ui, self.cache);
		ui.end_row();
	}
}

impl<T: Property + 'static> Default for CachedDrawer<T> {
	fn default() -> Self {
		Self {
			drawer: None,
			cache: [0.0; 100],
		}
	}
}

impl PropertyViewer {
	pub fn set(&mut self, protein: Rc<Protein>) {
		self.hydro = CachedDrawer::new(&Hydro, &protein);
		self.charge = CachedDrawer::new(&NetCharge, &protein);
		self.extinction = CachedDrawer::new(&Extinction, &protein);
		self.pi = CachedDrawer::new(&Pi, &protein);
		self.mass = CachedDrawer::new(&Mass, &protein);
		self.protein = Some(protein);
	}

	pub fn show(&mut self, ui: &mut Ui) {
		let Some(_) = &self.protein else {
			ui.centered_and_justified(|ui| ui.label("Brak danych"));
			return;
		};

		Extras::title_bar(ui, "Właściwości białka");

		Grid::new("PROTEIN_PROPERTY_GRID")
			.min_row_height(ui.available_height() / 5.0 as f32 - 5.0)
			.num_columns(2)
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
