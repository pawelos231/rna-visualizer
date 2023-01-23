use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

use egui::*;
use native_dialog::FileDialog;
use stringreader::StringReader;

use super::fast_text_edit::FastTextEdit;

#[derive(Default)]
pub struct ImportWindow {
	pub visible: bool,
	separator: String,
	delete_wrong_chars: bool,
	delete_header: bool,
	header_len: u32,
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

	#[allow(clippy::needless_range_loop)]
	pub fn generate_output(&self) -> String {
		enum Readable<'a> {
			Fs(File),
			Str(StringReader<'a>),
		}

		impl<'a> Read for Readable<'a> {
			fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
				match self {
					Readable::Fs(reader) => reader.read(buf),
					Readable::Str(reader) => reader.read(buf),
				}
			}
		}

		let readable = match self.from_file {
			true => Readable::Fs(File::open(&self.path).unwrap()),
			false => Readable::Str(StringReader::new(&self.input_rna)),
		};

		let length = match &readable {
			Readable::Fs(reader) => reader.metadata().unwrap().len() as usize,
			Readable::Str(_) => self.input_rna.len(),
		};

		let mut result_buffer = String::with_capacity(length);
		let mut reader = BufReader::new(readable);

		if self.delete_header {
			for _ in 0..self.header_len {
				reader.read_until(b'\n', &mut Vec::new()).ok();
			}
		}

		let separator_len = self.separator.len();
		let rem_separator = separator_len != 0;

		let mut byte_buff = [0u8; 4096];
		while let Ok(len) = reader.read(&mut byte_buff) {
			if len == 0 {
				break;
			}
			for i in 0..len {
				let ch = byte_buff[i] as char;
				if matches!(
					ch,
					'A' | 'G' | 'C' | 'U' | 'T' | 'a' | 'g' | 'c' | 'u' | 't'
				) {
					result_buffer.push(ch);
					if rem_separator && result_buffer.ends_with(&self.separator) {
						for _ in 0..separator_len {
							result_buffer.pop();
						}
					}
				}
			}
		}

		result_buffer.shrink_to_fit();
		result_buffer
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
			ui.label("Separator:");
			ui.centered_and_justified(|ui| {
				ui.text_edit_singleline(&mut self.separator);
			});
		});
		ui.checkbox(&mut self.delete_wrong_chars, "Usuń niepoprawne znaki");
		ui.checkbox(&mut self.delete_header, "Usuń nagłówek");
		ui.horizontal(|ui| {
			ui.add_enabled_ui(self.delete_header, |ui| {
				ui.label("Ilość linii do usunięcia:");
				ui.add(DragValue::new(&mut self.header_len));
			});
		});
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
