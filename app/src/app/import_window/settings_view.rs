use egui::*;
use native_dialog::FileDialog;

use super::ImportSettings;
use crate::app::extras::FastTextEdit;

#[derive(Default)]
pub struct SettingsView {
	pub settings: ImportSettings,
}

impl SettingsView {
	pub fn show(&mut self, ui: &mut Ui) -> bool {
		self.show_source_select(ui);
		match self.settings.from_file {
			true => self.show_file_select(ui),
			false => self.show_rna_input(ui),
		};
		self.show_preprocessing_opts(ui);
		self.show_import(ui)
	}

	fn show_source_select(&mut self, ui: &mut Ui) {
		ui.label(RichText::new("Ustawienia źródła:").strong());
		ui.horizontal(|ui| {
			ui.radio_value(&mut self.settings.from_file, true, "Plik");
			ui.radio_value(&mut self.settings.from_file, false, "Z tekstu");
		});
	}

	fn show_file_select(&mut self, ui: &mut Ui) {
		ui.label("Ścieżka pliku: ");
		ui.horizontal(|ui| {
			ui.centered_and_justified(|ui| {
				FastTextEdit::singleline(ui, &mut self.settings.path);
				if ui.button("Wybierz plik...").clicked() {
					let path = FileDialog::new()
						.set_location("~/Desktop")
						.add_filter("Text file", &["txt"])
						.add_filter("all files", &["*"])
						.show_open_single_file()
						.unwrap();
					self.settings.path = match path {
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
				ui.text_edit_multiline(&mut self.settings.input_rna);
			});
		});
	}

	fn show_preprocessing_opts(&mut self, ui: &mut Ui) {
		ui.separator();
		ui.label(RichText::new("Ustawienia preprocesora:").strong());
		ui.horizontal(|ui| {
			ui.label("Separator:");
			ui.centered_and_justified(|ui| {
				ui.text_edit_singleline(&mut self.settings.separator);
			});
		});
		ui.checkbox(
			&mut self.settings.delete_wrong_chars,
			"Usuń niepoprawne znaki",
		);
		ui.checkbox(&mut self.settings.delete_header, "Usuń nagłówek");
		ui.horizontal(|ui| {
			ui.add_enabled_ui(self.settings.delete_header, |ui| {
				ui.label("Ilość linii do usunięcia:");
				ui.add(DragValue::new(&mut self.settings.header_len));
			});
		});
	}

	fn show_import(&mut self, ui: &mut Ui) -> bool {
		ui.separator();
		ui.button("Importuj").clicked()
	}
}
