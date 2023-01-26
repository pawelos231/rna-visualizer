use egui::*;

mod assets;
pub use assets::*;

mod protein_loader;
pub use protein_loader::*;

mod acid_painter;
pub use acid_painter::*;

mod protein_cache;
pub use protein_cache::*;
use rnalib::Protein;

#[derive(Default)]
pub struct ProteinViewer {
	pub protein: Option<Protein>,
	cache: ProteinCache,
}

impl ProteinViewer {
	pub fn show(&mut self, ui: &mut Ui) {
		if self.protein.is_none() {
			ui.centered_and_justified(|ui| ui.label("Brak białka do wyświetlenia"));
			return;
		}

		self.show_protein(ui);
	}

	fn show_protein(&mut self, ui: &mut Ui) {
		let Some(protein) = &self.protein else { return };
		ScrollArea::horizontal().show(ui, |ui| {
			ui.vertical_centered_justified(|ui| {
				ui.horizontal(|ui| {
					let mut codon_iter = protein.get_codons().iter();
					let mut previous = None;
					let mut current = codon_iter.next();
					while let Some(codon) = current {
						let next = codon_iter.next();

						let shorthand = codon.get_acid_shorthand();
						let cache = &mut self.cache;
						let base_type = match (previous.is_some(), next.is_some()) {
							(false, false) => BaseType::BASE,
							(false, true) => BaseType::BASE_NO_RIGHT,
							(true, false) => BaseType::BASE_NO_LEFT,
							(true, true) => BaseType::BASE_NO_SIDES,
						};

						AcidPainter::show(ui, cache, base_type, shorthand);

						previous = current;
						current = next;
					}
				});
			});
		});
	}
}
