use std::collections::{hash_map::Entry::*, HashMap};

use super::{assets::BaseType, *};
use crate::app::svg_image::SvgImage;

#[derive(Default)]
pub struct ProteinCache {
	svgs: HashMap<char, SvgImage>,
	base_svgs: HashMap<usize, SvgImage>,
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

	pub fn get(&self, shorthand: char) -> Option<&SvgImage> {
		self.svgs.get(&shorthand)
	}

	pub fn get_base(&self, base_type: BaseType) -> Option<&SvgImage> {
		self.base_svgs.get(&(base_type as usize))
	}
}
