use super::{assets::*, ProteinSvg};
use crate::app::svg_image::SvgImage;

mod processor;
use processor::*;

pub struct Loader;

impl Loader {
	pub fn load(shorthand: char) -> Option<ProteinSvg> {
		let body = get_body(shorthand)?;

		let regular_src = body.get_regular();
		let regular_tree = Processor::process_svg(regular_src);
		let regular = SvgImage::from_svg_tree(&regular_tree);

		let flipped = match body.get_flipped() {
			Some(flipped_src) => {
				let flipped_tree = Processor::process_svg(flipped_src);
				Some(SvgImage::from_svg_tree(&flipped_tree))
			}
			None => None,
		};

		Some(ProteinSvg::new(regular, flipped))
	}

	pub fn load_base(base_type: BaseType) -> Option<SvgImage> {
		let svg = get_base_svg(base_type)?;
		let tree = Processor::process_svg(svg);
		Some(SvgImage::from_svg_tree(&tree))
	}
}
