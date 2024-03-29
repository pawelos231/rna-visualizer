//! The module that implements [`ImportView`]

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

/// A window view that allows the user to peek
/// into the importer's progress.
#[derive(Default)]
pub struct ImportView {
	pub settings: ImportSettings,
	job: ImportJob,
}

impl ImportView {
	/// Creates a new instance of [`ImportView`],
	/// with the default settings provided
	pub fn new(settings: ImportSettings) -> Self {
		Self {
			settings,
			..Default::default()
		}
	}

	/// Draws self to the ui.
	pub fn show(&mut self, ui: &mut Ui) -> Option<Result<ProteinMap, String>> {
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
			return self.job.take();
		}

		None
	}
}

/// A multithreaded import job, that keeps track
/// of current progress.
#[derive(Default)]
struct ImportJob {
	result: Arc<Mutex<Option<ProteinMap>>>,
	progress: Arc<AtomicU32>,
	started: Arc<AtomicBool>,
	finished: Arc<AtomicBool>,
	error: Arc<Mutex<Option<String>>>,
}

impl ImportJob {
	/// Starts the job, resetting the internal
	/// state as necessary.
	pub fn run(&mut self, settings: ImportSettings) {
		self.started.store(true, Ordering::Relaxed);
		self.finished.store(false, Ordering::Relaxed);

		let progess = self.progress.clone();
		let result = self.result.clone();
		let finished = self.finished.clone();
		let error = self.error.clone();

		spawn(move || {
			let source = Self::generate_output(&settings);

			let mut importer = ThreadedProteinLoader::default();
			importer.start(source);

			while !importer.is_ready() {
				let percentage = importer.get_progress() * 100.0;
				let percentage = percentage.min(100.0) as u32;
				progess.store(percentage, Ordering::Relaxed);
			}

			match importer.take() {
				Some(map) => {
					let mut guard = result.lock().unwrap();
					*guard = Some(map);
				}
				None => {
					if let Ok(mut lock) = error.lock() {
						*lock = Some(ProteinMap::ERR_MESSAGE.into());
					}
				}
			}

			finished.store(true, Ordering::Relaxed);
		});
	}

	/// Returns true if this job had finished.
	pub fn finished(&self) -> bool {
		self.finished.load(Ordering::Relaxed)
	}

	/// Returns true if this job had started.
	pub fn started(&self) -> bool {
		self.started.load(Ordering::Relaxed)
	}

	/// Returns the current import progress as a
	/// percentage (natural number from 0 to 100).
	pub fn progress(&self) -> u32 {
		self.progress.load(Ordering::Relaxed)
	}

	/// Takes ownership of the imported [`ProteinMap`].
	///
	/// Returns [`None`] if nothing has been imported yet.
	///
	/// Returns [`Err`] if there was an error parsing the document.
	pub fn take(&mut self) -> Option<Result<ProteinMap, String>> {
		let Ok(mut guard) = self.result.lock() else { return Some(Err(ProteinMap::ERR_MESSAGE.into())) };
		let Ok(mut lock) = self.error.lock() else { return Some(Err(ProteinMap::ERR_MESSAGE.into()))};
		match lock.as_ref() {
			Some(err) => {
				let err = err.clone();
				*lock = None;
				Some(Err(err))
			}
			None => guard.take().map(Ok),
		}
	}

	/// A helper function that filters through the source
	/// string and pre-processes it according to user settings.
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
				) || !settings.delete_wrong_chars
				{
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
