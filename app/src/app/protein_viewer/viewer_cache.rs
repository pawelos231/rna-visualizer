//! The module that implements [`ViewerCache`]

use std::collections::{hash_map::Entry::*, HashMap};

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
			if let Some(svg) = ThreadedLoader::load_body(shorthand) {
				entry.insert(svg);
			}
		}
	}

	/// Starts lazily loading a given base's asset
	/// in the background.
	pub fn lazy_load_base(&mut self, base_type: BaseType) {
		if let Vacant(entry) = self.base_svgs.entry(base_type as usize) {
			if let Some(svg) = ThreadedLoader::load_base(base_type) {
				entry.insert(svg);
			}
		}
	}

	/// Starts lazily building the cache for all
	/// assets in the background.
	pub fn load_threaded(&mut self) {
		if self.loader.is_ready() && !self.loader.is_busy() {
			return;
		}

		let loader = &mut self.loader;

		if !loader.is_busy() {
			loader.load();
		}

		if loader.is_ready() {
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

		self.loader.set_busy(false);
	}

	/// Returns the cached protein body svg image, indexed
	/// by its shorthand.
	///
	/// Returns [`None`] if no such body is present in the cache.
	pub fn get(&self, shorthand: char) -> Option<&ProteinSvg> {
		self.svgs.get(&shorthand)
	}

	/// Returns the cached protein base svg image, indexed
	/// by its shorthand.
	///
	/// Returns [`None`] if no such base is present in the cache.
	pub fn get_base(&self, base_type: BaseType) -> Option<&SvgImage> {
		self.base_svgs.get(&(base_type as usize))
	}
}
