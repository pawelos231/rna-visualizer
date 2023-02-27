use std::{
	cell::RefMut,
	collections::HashMap,
	sync::{
		atomic::{AtomicBool, Ordering},
		Arc, Mutex,
	},
	thread::spawn,
};

use super::{assets::*, ProteinSvg};
use crate::app::svg_image::SvgImage;

mod processor;
use processor::*;
use usvg::*;

/// An image loader that works on a separate thread.
#[derive(Default)]
pub struct ThreadedLoader {
	pub svgs: Arc<Mutex<HashMap<char, ProteinSvg>>>,
	pub base_svgs: Arc<Mutex<HashMap<usize, SvgImage>>>,
	loaded: Arc<AtomicBool>,
	busy: bool,
}

impl ThreadedLoader {
	/// Starts loading all available textures.
	pub fn load(&mut self) {
		self.busy = true;

		let svgs = self.svgs.clone();
		let base_svgs = self.base_svgs.clone();
		let loaded = self.loaded.clone();

		spawn(move || {
			for i in 65u8..91u8 {
				if let Some(svg) = Self::load_body(i as char) {
					let mut guard = svgs.lock().unwrap();
					guard.insert(i as char, svg);
				}
			}

			for base in BASES {
				if let Some(svg) = Self::load_base(base) {
					let mut guard = base_svgs.lock().unwrap();
					guard.insert(base as usize, svg);
				}
			}

			loaded.store(true, Ordering::Relaxed);
		});
	}

	/// Returns true if this loader finished loading
	/// the images.
	pub fn is_ready(&self) -> bool {
		self.loaded.load(Ordering::Relaxed)
	}

	/// Returns true if this loader is busy loading
	/// the images.
	pub fn is_busy(&self) -> bool {
		self.busy
	}

	/// Overrides this loader's busy state
	pub fn set_busy(&mut self, busy: bool) {
		self.busy = busy;
	}

	/// A helper method for loading an acid body svg
	/// by its shorthand
	///
	/// Returns [`None`] if no such acid exists.
	pub fn load_body(shorthand: char) -> Option<ProteinSvg> {
		let body = get_body(shorthand)?;

		let regular_src = body.get_regular();
		let regular_tree = Self::process_svg(regular_src);
		let regular = SvgImage::from_svg_tree(&regular_tree);

		let flipped = match body.get_flipped() {
			Some(flipped_src) => {
				let flipped_tree = Self::process_svg(flipped_src);
				Some(SvgImage::from_svg_tree(&flipped_tree))
			}
			None => None,
		};

		Some(ProteinSvg::new(regular, flipped))
	}

	/// A helper method for loading an acid base svg
	/// by its type
	pub fn load_base(base_type: BaseType) -> Option<SvgImage> {
		let svg = get_base_svg(base_type)?;
		let tree = Processor::process_svg(svg);
		Some(SvgImage::from_svg_tree(&tree))
	}

	/// A helper method for pre-processing an svg node
	fn process_node(mut node: Node) {
		let data = node.borrow_mut();

		RefMut::filter_map(data, |x: &mut NodeKind| {
			if let NodeKind::Path(path) = x {
				if let Some(fill) = &mut path.fill {
					fill.paint = Paint::Color(Color::white());
					fill.opacity = NormalizedValue::new(0.8);
				}
				if let Some(stroke) = &mut path.stroke {
					stroke.paint = Paint::Color(Color::white());
					stroke.opacity = NormalizedValue::new(0.8);
					stroke.width = StrokeWidth::new(3.0);
				}
			}
			Some(x)
		})
		.ok();

		for node in node.children() {
			Self::process_node(node);
		}
	}

	/// A helper method for pre-processing an svg source string
	fn process_svg(data: &str) -> Tree {
		let options = Options {
			keep_named_groups: true,
			image_rendering: ImageRendering::OptimizeQuality,
			..Options::default()
		};

		let document = Tree::from_str(data, &options.to_ref()).unwrap();
		Self::process_node(document.root());

		document
	}
}
