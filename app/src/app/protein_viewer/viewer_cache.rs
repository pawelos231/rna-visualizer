//! The module that implements [`ViewerCache`]

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

/// Holds cached protein texture data.
#[derive(Default)]
pub struct ViewerCache {
	/// Svgs of protein bodies
	svgs: HashMap<char, ProteinSvg>,
	/// Svgs of protein bases
	base_svgs: HashMap<usize, SvgImage>,
	/// A multithreaded image loader
	loader: ThreadedLoader,
}

impl ViewerCache {
	/// Starts lazily loading a given protein's asset
	/// in the background.
	pub fn lazy_load(&mut self, shorthand: char) {
		if let Vacant(entry) = self.svgs.entry(shorthand) {
			if let Some(svg) = Loader::load(shorthand) {
				entry.insert(svg);
			}
		}
	}

	/// Starts lazily loading a given base's asset
	/// in the background.
	pub fn lazy_load_base(&mut self, base_type: BaseType) {
		if let Vacant(entry) = self.base_svgs.entry(base_type as usize) {
			if let Some(svg) = Loader::load_base(base_type) {
				entry.insert(svg);
			}
		}
	}

	/// Starts lazily building the cache for all
	/// assets in the background.
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

	/// Starts lazily building the cache for all
	/// assets in the background.
	pub fn get(&self, shorthand: char) -> Option<&ProteinSvg> {
		self.svgs.get(&shorthand)
	}

	pub fn get_base(&self, base_type: BaseType) -> Option<&SvgImage> {
		self.base_svgs.get(&(base_type as usize))
	}
}

/// An image loader that works on a separate thread.
#[derive(Default)]
struct ThreadedLoader {
	svgs: Arc<Mutex<HashMap<char, ProteinSvg>>>,
	base_svgs: Arc<Mutex<HashMap<usize, SvgImage>>>,
	loaded: Arc<AtomicBool>,
	busy: bool,
}

impl ThreadedLoader {
	/// Starts loading all available textures.
	fn load(&mut self) {
		self.busy = true;

		let svgs = self.svgs.clone();
		let base_svgs = self.base_svgs.clone();
		let loaded = self.loaded.clone();

		spawn(move || {
			for i in 65u8..91u8 {
				if let Some(svg) = Loader::load(i as char) {
					let mut guard = svgs.lock().unwrap();
					guard.insert(i as char, svg);
				}
			}

			for base in BASES {
				if let Some(svg) = Loader::load_base(base) {
					let mut guard = base_svgs.lock().unwrap();
					guard.insert(base as usize, svg);
				}
			}

			loaded.store(true, Ordering::Relaxed);
		});
	}
}
