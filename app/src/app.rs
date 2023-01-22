use std::collections::HashMap;

use eframe::Frame;
use egui::{Button, CentralPanel, Context, ScrollArea, SidePanel, TopBottomPanel};
use rnalib::{AminoString, Protein};

#[derive(Default)]
pub struct App {
	rna: String,
	proteins: ProteinMap,
}

#[derive(Default)]
struct ProteinMap {
	proteins: HashMap<String, Protein>,
	sorted_keys: Vec<String>,
}

impl ProteinMap {
	fn new(source: Vec<Protein>) -> Self {
		let mut proteins = HashMap::new();
		let mut sorted_keys = Vec::new();

		for protein in source {
			sorted_keys.push(protein.to_string());
			proteins.insert(protein.to_string(), protein);
		}

		sorted_keys.sort_by(|a, b| a.to_string().len().cmp(&b.to_string().len()));

		Self {
			proteins,
			sorted_keys,
		}
	}
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_pixels_per_point(1.3);
		Self::default()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		TopBottomPanel::top("TOP").show(ctx, |ui| {
			ui.horizontal(|ui| {
				ui.label("Ciąg RNA");
				ui.text_edit_singleline::<String>(&mut self.rna);
				if ui.button("Wczytaj").clicked() {
					let mut proteins = Vec::new();
					for amino in AminoString::parse(&self.rna) {
						for protein in amino.get_proteins() {
							proteins.push(protein);
						}
					}
					self.proteins = ProteinMap::new(proteins);
				}
			});
		});

		SidePanel::left("left_panel")
			.resizable(true)
			.default_width(150.0)
			.width_range(80.0..=200.0)
			.show(ctx, |ui| {
				ScrollArea::vertical().show(ui, |ui| {
					for protein in &self.proteins.sorted_keys {
						ui.add_sized([300., 30.], Button::new(protein));
					}
				});
			});

		CentralPanel::default().show(ctx, |ui| {});
	}
}
