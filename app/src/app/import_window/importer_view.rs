use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
};

use egui::*;
use rnalib::ProteinMap;
use stringreader::StringReader;

use super::ImportSettings;

#[derive(Default)]
pub struct ImportView {
	pub settings: ImportSettings,
	finished: bool,
	frame: usize,
}

impl ImportView {
	pub fn new(settings: ImportSettings) -> Self {
		Self {
			settings,
			..Default::default()
		}
	}

	pub fn show(&mut self, ui: &mut Ui) -> Option<ProteinMap> {
		if self.finished {
			ui.label("Zaimportowano dane");
		} else {
			ui.label("Importowanie w toku...");
		}

		if self.frame > 0 && !self.finished {
			self.finished = true;
			return Some(ProteinMap::parse_multithreaded(&self.generate_output()));
		}

		self.frame += 1;
		None
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

		let readable = match self.settings.from_file {
			true => Readable::Fs(File::open(&self.settings.path).unwrap()),
			false => Readable::Str(StringReader::new(&self.settings.input_rna)),
		};

		let length = match &readable {
			Readable::Fs(reader) => reader.metadata().unwrap().len() as usize,
			Readable::Str(_) => self.settings.input_rna.len(),
		};

		let mut result_buffer = String::with_capacity(length);
		let mut reader = BufReader::new(readable);

		if self.settings.delete_header {
			for _ in 0..self.settings.header_len {
				reader.read_until(b'\n', &mut Vec::new()).ok();
			}
		}

		let separator_len = self.settings.separator.len();
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
					if rem_separator && result_buffer.ends_with(&self.settings.separator) {
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
}
