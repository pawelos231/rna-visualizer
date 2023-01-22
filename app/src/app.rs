use crate::ProteinMap;
use eframe::Frame;
use egui::{Button, CentralPanel, Context, ScrollArea, SidePanel, TopBottomPanel, Window};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use rnalib::AminoString;

#[derive(Default)]
pub struct App {
	rna: String,
	proteins: ProteinMap,
	flag: bool,
	separator: String,
	delete_wrong_chars: bool,
	import_type: bool,
	path: String,
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

		Window::new("Import danych")
			.resizable(true)
			.show(ctx, |ui| {
				ui.label("Źródło");
				ui.horizontal(|ui| {
					ui.radio_value(&mut self.import_type, true, "Plik");
					ui.radio_value(&mut self.import_type, false, "Z tekstu");
				});
				if self.import_type {
					ui.horizontal(|ui| {
						ui.label("wybierz plik: ");
						ui.text_edit_singleline::<String>(&mut self.path);
						if ui.button("Wczytaj plik").clicked() {
							let path = FileDialog::new()
								.set_location("~/Desktop")
								.add_filter("Text file", &["txt"])
								.add_filter("all files", &["*"])
								.show_open_single_file()
								.unwrap();
							self.path = match path {
								Some(path) => path.to_str().unwrap().to_owned(),
								None => return,
							};
						};
					});
				} else {
					ui.horizontal(|ui| {
						ui.label("Wpisz: ");
					});
					ui.text_edit_multiline::<String>(&mut self.rna);
				};
				ui.separator();
				ui.horizontal(|ui| {
					ui.label("Separator: ");
					ui.text_edit_singleline::<String>(&mut self.separator);
				});
				ui.checkbox(&mut self.delete_wrong_chars, "Usuń niepoprawne znaki");
			});

		CentralPanel::default().show(ctx, |ui| {});
	}
}
