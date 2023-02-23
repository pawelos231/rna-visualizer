use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
	sync::atomic::{AtomicBool, AtomicU32, Ordering},
	thread::spawn,
};

use egui::*;
use rnalib::{ProteinMap, ThreadedProteinLoader};
use stringreader::StringReader;

use std::sync::{Arc, Mutex};

use super::ImportSettings;

#[derive(Default)]
pub struct ImportView {
	pub settings: ImportSettings,
	job: ImportJob,
}

impl ImportView {
	pub fn new(settings: ImportSettings) -> Self {
		Self {
			settings,
			..Default::default()
		}
	}

	pub fn show(&mut self, ui: &mut Ui) -> Option<ProteinMap> {
		match self.job.finished() {
			true => {
				ui.label("Zaimportowano dane.");
			}
			false => {
				ui.label("Importowanie w toku...");
				let progress = self.job.progress();
				match progress {
					0 => {
						ui.label("Filtrowanie pliku wejściowego...");
						ui.ctx().request_repaint();
					}
					100 => {
						ui.label("Indeksowanie wyników...");
						ui.ctx().request_repaint();
					}
					_ => {
						let bar = ProgressBar::new(progress as f32 / 100.0)
							.animate(true)
							.show_percentage()
							.text("Szukanie białek");
						ui.ctx().request_repaint();
						ui.add(bar);
					}
				}
			}
		};

		if !self.job.started() {
			self.job.run(self.settings.clone());
		}

		if self.job.finished() {
			return self.job.pop();
		}

		None
	}
}

#[derive(Default)]
struct ImportJob {
	result: Arc<Mutex<Option<ProteinMap>>>,
	progress: Arc<AtomicU32>,
	started: Arc<AtomicBool>,
	finished: Arc<AtomicBool>,
}

impl ImportJob {
	pub fn run(&mut self, settings: ImportSettings) {
		self.started.store(true, Ordering::Relaxed);
		self.finished.store(false, Ordering::Relaxed);

		let progess = self.progress.clone();
		let result = self.result.clone();
		let finished = self.finished.clone();

		spawn(move || {
			let source = Self::generate_output(&settings);

			let mut importer = ThreadedProteinLoader::default();
			importer.start(source);

			while !importer.is_ready() {
				let percentage = importer.get_progress() * 100.0;
				let percentage = percentage.min(100.0) as u32;
				progess.store(percentage, Ordering::Relaxed);
			}

			let map = importer.take().unwrap();

			let mut guard = result.lock().unwrap();
			*guard = Some(map);

			finished.store(true, Ordering::Relaxed);
		});
	}

	pub fn finished(&self) -> bool {
		self.finished.load(Ordering::Relaxed)
	}

	pub fn started(&self) -> bool {
		self.started.load(Ordering::Relaxed)
	}

	pub fn progress(&self) -> u32 {
		self.progress.load(Ordering::Relaxed)
	}

	pub fn pop(&mut self) -> Option<ProteinMap> {
		let Ok(mut guard) = self.result.lock() else { return None };
		guard.take()
	}

	fn generate_output(settings: &ImportSettings) -> String {
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

		let readable = match settings.from_file {
			true => Readable::Fs(File::open(&settings.path).unwrap()),
			false => Readable::Str(StringReader::new(&settings.input_rna)),
		};

		let length = match &readable {
			Readable::Fs(reader) => reader.metadata().unwrap().len() as usize,
			Readable::Str(_) => settings.input_rna.len(),
		};

		let mut result_buffer = String::with_capacity(length);
		let mut reader = BufReader::new(readable);

		if settings.delete_header {
			for _ in 0..settings.header_len {
				reader.read_until(b'\n', &mut Vec::new()).ok();
			}
		}

		let separator_len = settings.separator.len();
		let rem_separator = separator_len != 0;

		let mut byte_buff = [0u8; 4096];
		while let Ok(len) = reader.read(&mut byte_buff) {
			if len == 0 {
				break;
			}
			for &ch in &byte_buff[0..len] {
				let ch = ch as char;
				if matches!(
					ch,
					'A' | 'G' | 'C' | 'U' | 'T' | 'a' | 'g' | 'c' | 'u' | 't'
				) {
					result_buffer.push(ch);
					if rem_separator && result_buffer.ends_with(&settings.separator) {
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
