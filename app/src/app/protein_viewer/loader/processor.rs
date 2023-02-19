use std::cell::RefMut;

use usvg::*;

pub struct Processor;

impl Processor {
	pub fn process_svg(data: &str) -> Tree {
		let options = Options {
			keep_named_groups: true,
			image_rendering: ImageRendering::OptimizeQuality,
			..Options::default()
		};

		let document = Tree::from_str(data, &options.to_ref()).unwrap();
		Self::process_node(document.root());

		document
	}

	pub fn process_node(mut node: Node) {
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
}
