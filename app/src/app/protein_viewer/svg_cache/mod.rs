mod assets;

use super::svg_image::SvgImage;
use assets::*;
use std::{
	cell::RefMut,
	collections::{hash_map::Entry, HashMap},
};
use usvg::*;

#[derive(Default)]
pub struct SvgCache {
	protein_svgs: HashMap<char, SvgImage>,
	base: Option<SvgImage>,
	base_link: Option<SvgImage>,
	base_p: Option<SvgImage>,
	base_p_link: Option<SvgImage>,
	base_no_left: Option<SvgImage>,
	base_no_right: Option<SvgImage>,
	base_no_sides: Option<SvgImage>,
}

pub enum BaseType {
	Default,
	NoLeft,
	NoRight,
	NoSide,
	_Link,
	_P,
	_PLink,
}

impl SvgCache {
	pub fn smear_load_svg(&mut self) {
		for acid_shorthand in SUPPORTED_ACIDS {
			if let Entry::Vacant(entry) = self.protein_svgs.entry(acid_shorthand) {
				let svg_src = get_acid_svg_by_shorthand(acid_shorthand).unwrap();
				let svg = Self::process_svg(svg_src);
				entry.insert(SvgImage::from_svg_tree(&svg));
				return;
			}
		}

		if self.base.is_none() {
			let svg = Self::process_svg(BASE);
			self.base = Some(SvgImage::from_svg_tree(&svg));
			return;
		}

		if self.base_link.is_none() {
			let svg = Self::process_svg(BASE_LINK);
			self.base_link = Some(SvgImage::from_svg_tree(&svg));
			return;
		}

		if self.base_p.is_none() {
			let svg = Self::process_svg(BASE_P);
			self.base_p = Some(SvgImage::from_svg_tree(&svg));
			return;
		}

		if self.base_p_link.is_none() {
			let svg = Self::process_svg(BASE_P_LINK);
			self.base_p_link = Some(SvgImage::from_svg_tree(&svg));
		}

		if self.base_no_left.is_none() {
			let svg = Self::process_svg(BASE_NO_LEFT);
			self.base_no_left = Some(SvgImage::from_svg_tree(&svg));
		}

		if self.base_no_right.is_none() {
			let svg = Self::process_svg(BASE_NO_RIGHT);
			self.base_no_right = Some(SvgImage::from_svg_tree(&svg));
		}

		if self.base_no_sides.is_none() {
			let svg = Self::process_svg(BASE_NO_SIDES);
			self.base_no_sides = Some(SvgImage::from_svg_tree(&svg));
		}
	}

	pub fn get_acid(&self, shorthand: char) -> Option<&SvgImage> {
		self.protein_svgs.get(&shorthand)
	}

	pub fn get_base(&self, base_type: BaseType) -> Option<&SvgImage> {
		use BaseType::*;
		match base_type {
			Default => &self.base,
			NoLeft => &self.base_no_left,
			NoSide => &self.base_no_sides,
			NoRight => &self.base_no_right,
			_Link => &self.base_link,
			_P => &self.base_p,
			_PLink => &self.base_p_link,
		}
		.as_ref()
	}

	fn process_svg(data: &str) -> Tree {
		let options = Options {
			keep_named_groups: true,
			image_rendering: ImageRendering::OptimizeQuality,
			..Options::default()
		};

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
				process_node(node);
			}
		}

		let document = Tree::from_str(data, &options.to_ref()).unwrap();
		process_node(document.root());

		document
	}
}
