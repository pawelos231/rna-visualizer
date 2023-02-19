use std::rc::Rc;

use egui::*;

mod protein_svg;
pub use protein_svg::*;

mod assets;
pub use assets::*;

mod protein_loader;
pub use protein_loader::*;

mod acid_painter;
pub use acid_painter::*;

mod protein_cache;
pub use protein_cache::*;
use rnalib::Protein;

use super::extras::Extras;

#[derive(Default)]
pub struct ProteinViewer {
	pub protein: Option<Rc<Protein>>,
	cache: ProteinCache,
	painter: AcidPainter,
}

impl ProteinViewer {
	pub fn show(&mut self, ui: &mut Ui) {
		self.cache.load_threaded();

		if self.protein.is_none() {
			ui.centered_and_justified(|ui| ui.label("Brak białka do wyświetlenia"));
			return;
		}

		Extras::title_bar(ui, "Podgląd wykresu białka");

		self.show_protein(ui);
	}

	fn show_protein(&mut self, ui: &mut Ui) {
		let Some(protein) = &self.protein else { return };
		ScrollArea::horizontal().show(ui, |ui| {
			ui.add_space(ui.available_height() / 2.0 - 60.0 * self.painter.scale);
			ui.horizontal(|ui| {
				ui.add_space(50.0);
				let mut codon_iter = protein.get_codons().iter();
				let mut previous = None;
				let mut current = codon_iter.next();

				self.painter.flip = false;
				while let Some(codon) = current {
					let next = codon_iter.next();

					let shorthand = codon.get_acid_shorthand();
					let cache = &mut self.cache;
					let base_type = match (
						previous.is_some(),
						next.is_some(),
						shorthand.to_ascii_lowercase(),
					) {
						(false, false, 'p') => BaseType::BASE_P,
						(false, true, 'p') => BaseType::BASE_P_NO_RIGHT,
						(false, false, _) => BaseType::BASE,
						(false, true, _) => BaseType::BASE_NO_RIGHT,
						(true, false, _) => BaseType::BASE_NO_LEFT,
						(true, true, _) => BaseType::BASE_NO_SIDES,
					};

					self.painter.show(
						ui,
						cache,
						base_type,
						shorthand,
						next.map(|x| x.get_acid_shorthand()),
					);

					previous = current;
					current = next;
				}
				ui.add_space(50.0);
			});
			ui.add_space(ui.available_height() - 10.0);
		});
		ui.add_space(ui.available_height());
	}
}
