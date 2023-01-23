use eframe::{epaint::Shadow, Frame};
use egui::{CentralPanel, Context, Rounding, SidePanel, TopBottomPanel};
use rnalib::AminoString;

mod import_window;
use import_window::ImportWindow;

mod protein_selector;
use protein_selector::ProteinSelector;

mod fast_text_edit;
use fast_text_edit::FastTextEdit;

use crate::{fonts, protein_map::ProteinMap};
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

		let mut style = egui::Style::default();
		style.visuals.window_shadow = Shadow::small_dark();
		style.visuals.window_rounding = Rounding::same(3.0);
		style.animation_time = 0.0;

		cc.egui_ctx.set_style(style);

		let mut fonts = egui::FontDefinitions::default();
		fonts.font_data.insert(
			"Regular".to_owned(),
			egui::FontData::from_static(fonts::REGULAR),
		);
		fonts
			.font_data
			.insert("Bold".to_owned(), egui::FontData::from_static(fonts::BOLD));
		fonts
			.families
			.entry(egui::FontFamily::Proportional)
			.or_default()
			.insert(0, "Regular".to_owned());
		cc.egui_ctx.set_fonts(fonts);

		Self::default()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		if self.import_window.visible && self.import_window.show(ctx) {
			self.rna = self.import_window.generate_output();
		}

		TopBottomPanel::top("TOP").show(ctx, |ui| {
			ui.add_space(2.0);
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
			ui.add_space(2.0);
		});

		SidePanel::left("left_panel")
			.min_width(300.0)
			.show(ctx, |ui| {
				self.protein_selector.show(ui, &self.proteins);
			});

		CentralPanel::default().show(ctx, |_| {});
	}
}
