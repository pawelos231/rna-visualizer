use eframe::Frame;
use egui::{CentralPanel, Context, SidePanel, TopBottomPanel};
use rnalib::AminoString;

mod import_window;
use import_window::ImportWindow;

mod protein_selector;
use protein_selector::ProteinSelector;

mod fast_text_edit;
use fast_text_edit::FastTextEdit;

use crate::protein_map::ProteinMap;
pub type ProteinCollection = ProteinMap;

#[derive(Default)]
pub struct App {
	rna: String,
	proteins: ProteinCollection,
	import_window: ImportWindow,
	protein_selector: ProteinSelector,
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_pixels_per_point(1.3);
		Self::default()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		if self.import_window.visible && self.import_window.show(ctx) {
			self.rna = self.import_window.generate_output();
		}

		TopBottomPanel::top("TOP").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Ciąg RNA:");
				FastTextEdit::singleline(ui, &mut self.rna);
				if ui.button("Wczytaj").clicked() {
					let mut proteins = Vec::new();
					for amino in AminoString::parse(&self.rna) {
						proteins.append(&mut amino.get_proteins());
					}
					self.proteins = ProteinMap::new(proteins);
				};
				if ui.button("Wytnij niepoprawne znaki").clicked() {
					self.rna
						.retain(|x| "AGCUT ".contains(x.to_ascii_uppercase()));
				}
				if ui.button("Zaawansowany import...").clicked() {
					self.import_window.visible = true;
				}
			});
		});

		SidePanel::left("left_panel")
			.min_width(300.0)
			.show(ctx, |ui| {
				self.protein_selector.show(ui, &self.proteins);
			});

		CentralPanel::default().show(ctx, |_| {});
	}
}
