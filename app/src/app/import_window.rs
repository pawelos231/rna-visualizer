use egui::*;
use native_dialog::FileDialog;

use super::fast_text_edit::FastTextEdit;

#[derive(Default)]
pub struct ImportWindow {
	pub visible: bool,
	separator: String,
	delete_wrong_chars: bool,
	from_file: bool,
	input_rna: String,
	path: String,
	preview: String,
}

impl ImportWindow {
	pub fn show(&mut self, ctx: &Context) -> bool {
		let mut open = self.visible;
		let mut import = false;
		Window::new("Ustawienia importu")
			.open(&mut open)
			.resizable(true)
			.collapsible(false)
			.show(ctx, |ui| {
				self.show_source_select(ui);
				match self.from_file {
					true => self.show_file_select(ui),
					false => self.show_rna_input(ui),
				};
				self.show_preprocessing_opts(ui);
				self.show_preview(ui);
				import = self.show_import(ui);
			});
		self.visible = open;
		import
	}

	pub fn generate_output(&self) -> String {
		let output = match self.from_file {
			true => std::fs::read_to_string(&self.path),
			false => Ok(self.input_rna.to_owned()),
		};

		let Ok(mut output) = output else {
			return String::from("Invalid file path")
		};

		if !self.separator.is_empty() {
			output = output.as_str().replace(&self.separator, "");
		}

		if self.delete_wrong_chars {
			output.retain(|x| {
				matches!(
					x,
					'A' | 'G' | 'C' | 'U' | 'T' | 'a' | 'g' | 'c' | 'u' | 't' | ' '
				)
			});
		}

		output
	}

	fn show_source_select(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Ustawienia źródła:").strong());
		ui.horizontal(|ui| {
			ui.radio_value(&mut self.from_file, true, "Plik");
			ui.radio_value(&mut self.from_file, false, "Z tekstu");
		});
	}

	fn show_file_select(&mut self, ui: &mut Ui) {
		ui.label("Ścieżka pliku: ");
		ui.horizontal(|ui| {
			ui.centered_and_justified(|ui| {
				FastTextEdit::singleline(ui, &mut self.path);
				if ui.button("Wybierz plik...").clicked() {
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
		});
	}

	fn show_rna_input(&mut self, ui: &mut Ui) {
		ui.label("Wpisz:");
		ui.horizontal(|ui| {
			ui.centered_and_justified(|ui| {
				ui.text_edit_multiline(&mut self.input_rna);
			});
		});
	}

	fn show_preprocessing_opts(&mut self, ui: &mut Ui) {
		ui.separator();
		ui.label(RichText::new("Ustawienia preprocesora:").strong());
		ui.horizontal(|ui| {
			ui.label("Separator: ");
			ui.centered_and_justified(|ui| {
				ui.text_edit_singleline(&mut self.separator);
			});
		});
		ui.checkbox(&mut self.delete_wrong_chars, "Usuń niepoprawne znaki");
	}

	fn show_preview(&mut self, ui: &mut Ui) {
		ui.separator();
		ui.label(RichText::new("Podgląd:").strong());
		ui.horizontal(|ui| {
			ui.centered_and_justified(|ui| {
				ScrollArea::vertical().show(ui, |ui| {
					let max_w = ui.available_width() as usize;
					Grid::new("PREVIEW_GRID").show(ui, |ui| {
						self.preview
							.chars()
							.collect::<Vec<_>>()
							.chunks(max_w / 7)
							.for_each(|x| {
								ui.label(RichText::new(x.iter().collect::<String>()).monospace());
								ui.end_row();
							});
					});
				});
				if ui.button("Odśwież").clicked() {
					self.preview = self.generate_output();
				}
			});
		});
	}

	fn show_import(&mut self, ui: &mut Ui) -> bool {
		ui.separator();
		ui.button("Importuj").clicked()
	}
}
