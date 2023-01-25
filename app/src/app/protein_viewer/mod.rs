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
					let mut current_opt = codons.next();
					loop {
						let Some(current) = current_opt else { break };
						let next = codons.next();

						let Some(base) = &self.assets.get_base(match next {
							Some(_) => BaseType::Default,
							None => BaseType::P
						}) else {
							break;
						};

						let scale = 0.33;
						let shorthand = current.get_acid_shorthand();
						if let Some(image) = &self.assets.get_acid(shorthand) {
							let mut rect = base.show_scaled(ui, scale).rect;
							rect.min.x += 100.0 * scale - image.get_topmost_node_x() * scale;
							rect.min.y += rect.height() - 5.0;

							let clip = ui.clip_rect();
							ui.set_clip_rect(Rect::NOTHING);
							let mut next_rect = image.show_scaled(ui, scale).rect;
							next_rect.min.x -= 50.0 * scale;
							ui.set_clip_rect(clip);

							ui.allocate_ui_at_rect(rect, |ui| image.show_scaled(ui, scale));
							ui.allocate_ui_at_rect(next_rect, |_| {});
						}

						current_opt = next;
					}
				})
			});
		});
	}
}
