mod svg_cache;
mod svg_image;

use egui::*;
use rnalib::*;
use svg_cache::*;

#[derive(Default)]
pub struct ProteinViewer {
	pub protein: Option<Protein>,
	assets: SvgCache,
}

#[allow(clippy::or_fun_call)]
impl ProteinViewer {
	pub fn show(&mut self, ui: &mut Ui) {
		self.assets.smear_load_svg();
		let Some(protein) = &self.protein else {
			ui.centered_and_justified(|ui| ui.label("Brak białka do wyświetlenia"));
			return;
		};

		ScrollArea::horizontal().show(ui, |ui| {
			ui.centered_and_justified(|ui| {
				ui.horizontal(|ui| {
					let mut codons = protein.get_codons().iter();
					let mut prev_opt = None;
					let mut current_opt = codons.next();

					loop {
						let Some(current) = current_opt else { break };
						let next = codons.next();

						let base_target = match (prev_opt.is_some(), next.is_some()) {
							(true, true) => BaseType::NoSide,
							(false, true) => BaseType::NoRight,
							(false, false) => BaseType::Default,
							(true, false) => BaseType::NoLeft,
						};

						let Some(base) = &self.assets.get_base(base_target) else {
							break;
						};

						let base_bounds = base.get_bounds();
						let base_bottom_x = base_bounds.get_bottom()[0];

						let scale = 0.33;
						let shorthand = current.get_acid_shorthand();
						if let Some(image) = &self.assets.get_acid(shorthand) {
							let mut rect = base.show_scaled(ui, scale).rect;
							rect.min.x +=
								(base_bottom_x - image.get_bounds().get_top()[0]) * scale + 0.3;
							rect.min.y += 100.0 * scale;

							let clip = ui.clip_rect();
							ui.set_clip_rect(Rect::NOTHING);
							let mut next_rect = image.show_scaled(ui, scale).rect;
							next_rect.min.x -= 50.0 * scale;
							ui.set_clip_rect(clip);

							ui.allocate_ui_at_rect(rect, |ui| image.show_scaled(ui, scale));
							ui.allocate_ui_at_rect(next_rect, |_| {});
						}

						prev_opt = current_opt;
						current_opt = next;
					}
				})
			});
		});
	}
}
