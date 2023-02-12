use std::{
	collections::{hash_map::Entry::*, HashMap},
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Mutex,
	},
	thread::spawn,
};

use super::{assets::BaseType, *};
use crate::app::svg_image::SvgImage;

#[derive(Default)]
pub struct ProteinCache {
	svgs: HashMap<char, ProteinSvg>,
	base_svgs: HashMap<usize, SvgImage>,
	loader: ThreadedLoader,
}

impl ProteinCache {
	pub fn lazy_load(&mut self, shorthand: char) {
		if let Vacant(entry) = self.svgs.entry(shorthand) {
			if let Some(svg) = ProteinLoader::load(shorthand) {
				entry.insert(svg);
			}
		}
	}

	pub fn lazy_load_base(&mut self, base_type: BaseType) {
		if let Vacant(entry) = self.base_svgs.entry(base_type as usize) {
			if let Some(svg) = ProteinLoader::load_base(base_type) {
				entry.insert(svg);
			}
		}
	}

	pub fn load_threaded(&mut self) {
		if self.loader.loaded.load(Ordering::Relaxed) && !self.loader.busy {
			return;
		}

		let loader = &mut self.loader;

		if !loader.busy {
			loader.load();
		}

		if loader.loaded.load(Ordering::Relaxed) {
			let Ok(guard) = loader.svgs.lock() else { return };
			for (k, v) in guard.iter() {
				self.svgs.insert(*k, v.clone());
			}

			let Ok(guard) = loader.base_svgs.lock() else { return };
			for (k, v) in guard.iter() {
				self.base_svgs.insert(*k, v.clone());
			}
		} else {
			return;
		}

		self.loader.busy = false;
	}

	pub fn get(&self, shorthand: char) -> Option<&ProteinSvg> {
		self.svgs.get(&shorthand)
	}

	pub fn get_base(&self, base_type: BaseType) -> Option<&SvgImage> {
		self.base_svgs.get(&(base_type as usize))
	}
}

#[derive(Default)]
struct ThreadedLoader {
	svgs: Arc<Mutex<HashMap<char, ProteinSvg>>>,
	base_svgs: Arc<Mutex<HashMap<usize, SvgImage>>>,
	loaded: Arc<AtomicBool>,
	busy: bool,
}

impl ThreadedLoader {
	fn load(&mut self) {
		self.busy = true;

		let svgs = self.svgs.clone();
		let base_svgs = self.base_svgs.clone();
		let loaded = self.loaded.clone();

		spawn(move || {
			for i in 65u8..91u8 {
				if let Some(svg) = ProteinLoader::load(i as char) {
					let mut guard = svgs.lock().unwrap();
					guard.insert(i as char, svg);
				}
			}

			for base in BASES {
				if let Some(svg) = ProteinLoader::load_base(base) {
					let mut guard = base_svgs.lock().unwrap();
					guard.insert(base as usize, svg);
				}
			}

			loaded.store(true, Ordering::Relaxed);
		});
	}
}
