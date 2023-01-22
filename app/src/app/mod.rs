use crate::ProteinMap;
use eframe::Frame;
use egui::{Button, CentralPanel, Context, ScrollArea, SidePanel, TopBottomPanel};
use rnalib::AminoString;

mod import_window;
use import_window::ImportWindow;

#[derive(Default)]
pub struct App {
	rna: String,
	proteins: ProteinMap,
	flag: bool,
	import_window: ImportWindow,
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
				ui.text_edit_singleline::<String>(&mut self.rna);
				if ui.button("Wczytaj").clicked() {
					self.flag = true;
					let mut proteins = Vec::new();
					for amino in AminoString::parse(&self.rna) {
						for protein in amino.get_proteins() {
							proteins.push(protein);
						}
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
			.resizable(true)
			.default_width(150.0)
			.width_range(80.0..=200.0)
			.show(ctx, |ui| {
				ScrollArea::vertical().show(ui, |ui| {
					if self.proteins.sorted_keys.len() as u32 == 0 && self.flag {
						ui.add_sized([300., 30.], Button::new("Nie znaleziono zadnych białek"));
					};
					for protein in &self.proteins.sorted_keys {
						ui.add_sized([300., 30.], Button::new(protein));
					}
				});
			});

		CentralPanel::default().show(ctx, |_| {});
	}
}
