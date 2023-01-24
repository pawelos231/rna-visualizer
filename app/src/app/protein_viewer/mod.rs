use std::{
	cell::RefMut,
	collections::{hash_map::Entry, HashMap},
};

use egui::*;
use rnalib::Protein;
use usvg::*;

mod svg_image;
use svg_image::SvgImage;

mod assets;
use assets::*;

#[derive(Default)]
pub struct ProteinViewer {
	pub protein: Option<Protein>,
	protein_svgs: HashMap<char, SvgImage>,
}

#[allow(clippy::or_fun_call)]
impl ProteinViewer {
	pub fn show(&mut self, ui: &mut Ui) {
		self.smear_load_svg();

		let Some(protein) = &self.protein else {
			ui.centered_and_justified(|ui| ui.label("Brak wykresu"));
			return;
		};

		ScrollArea::horizontal().show(ui, |ui| {
			for acid in protein.get_codons() {
				let image = &self.protein_svgs[&acid.get_acid_shorthand()];
				image.show_scaled(ui, 0.3);
			}
		});
	}

	fn smear_load_svg(&mut self) {
		for acid_shorthand in SUPPORTED_ACIDS {
			if let Entry::Vacant(entry) = self.protein_svgs.entry(acid_shorthand) {
				let svg = Self::process_svg(get_acid_svg_by_shorthand(acid_shorthand).unwrap());
				entry.insert(SvgImage::from_svg_tree(&svg));
				break;
			}
		}
	}

	#[allow(clippy::all)]
	fn process_svg(data: &str) -> Tree {
		let mut options = Options::default();
		options.keep_named_groups = true;
		options.image_rendering = ImageRendering::OptimizeQuality;

		fn process_node(mut node: Node) {
			let data = node.borrow_mut();

			RefMut::filter_map(data, |x: &mut NodeKind| {
				if let NodeKind::Path(path) = x {
					if let Some(stroke) = &mut path.stroke {
						stroke.paint = Paint::Color(Color::white())
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
		for node in document.root().children() {
			process_node(node);
		}

		document
	}
}
