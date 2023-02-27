//! The module that implements [`ProteinViewer`]

use std::rc::Rc;

use egui::*;

mod protein_svg;
pub use protein_svg::*;

mod assets;
pub use assets::*;

mod threaded_loader;
pub use threaded_loader::*;

mod acid_painter;
pub use acid_painter::*;

mod viewer_cache;
pub use viewer_cache::*;

use rnalib::Protein;

use super::extras::Extras;

/// A ui widget that displays a given protein's structure.
#[derive(Default)]
pub struct ProteinViewer {
	/// The protein to display.
	pub protein: Option<Rc<Protein>>,
	/// Cached texture assets.
	cache: ViewerCache,
	/// The acid painter.
	painter: AcidPainter,
}

impl ProteinViewer {
	/// Draws self to the ui.
	pub fn show(&mut self, ui: &mut Ui) {
		self.cache.load_threaded();

		if self.protein.is_none() {
			ui.centered_and_justified(|ui| ui.label("Brak białka do wyświetlenia"));
			return;
		}

		Extras::title_bar(ui, "Podgląd wykresu białka");

		self.show_protein(ui);
	}

	/// Determines the protein base type to use depending on
	/// whether there exists a preceding / next protein.
	fn determine_base(previous: bool, next: bool, shorthand: char) -> BaseType {
		match (previous, next, shorthand) {
			(false, false, 'p' | 'P') => BaseType::BASE_P,
			(false, true, 'p' | 'P') => BaseType::BASE_P_NO_RIGHT,
			(false, false, _) => BaseType::BASE,
			(false, true, _) => BaseType::BASE_NO_RIGHT,
			(true, false, _) => BaseType::BASE_NO_LEFT,
			(true, true, _) => BaseType::BASE_NO_SIDES,
		}
	}

	/// Internal rendering method. Calculates layout and draws
	/// widgets to the ui accordingly.
	fn show_protein(&mut self, ui: &mut Ui) {
		let Some(protein) = &self.protein else { return };
		ScrollArea::horizontal()
			.enable_scrolling(ui.is_enabled())
			.show(ui, |ui| {
				ui.add_space(ui.available_height() / 2.0 - 60.0 * self.painter.scale);
				ui.horizontal(|ui| {
					ui.add_space(50.0);
					let mut codon_iter = protein.get_codons().iter();
					let mut previous = None;
					let mut current = codon_iter.next();

					self.painter.flip = false;
					while let Some(codon) = current {
						let next = codon_iter.next();

						let cache = &mut self.cache;
						let shorthand = codon.get_acid_shorthand();
						let next_shorthand = next.map(|x| x.get_acid_shorthand());
						let base_type =
							Self::determine_base(previous.is_some(), next.is_some(), shorthand);

						self.painter
							.show(ui, cache, base_type, shorthand, next_shorthand);

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
