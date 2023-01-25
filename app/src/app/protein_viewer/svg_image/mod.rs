use egui::*;
use mutex::*;
use resvg::render;
use tiny_skia::Pixmap;
use usvg::*;

pub struct SvgImage {
	size: [usize; 2],
	image: Mutex<ColorImage>,
	texture: Mutex<Option<TextureHandle>>,
	topmost_node_x: f32,
}

impl SvgImage {
	pub fn from_color_image(image: ColorImage) -> Self {
		Self {
			size: image.size,
			image: Mutex::new(image),
			texture: Default::default(),
			topmost_node_x: 0.0,
		}
	}

	pub fn from_svg_tree(tree: &Tree) -> Self {
		let tree_data = load_svg_tree(tree).unwrap();
		Self {
			topmost_node_x: tree_data.1,
			..Self::from_color_image(tree_data.0)
		}
	}

	pub fn get_size(&self) -> [usize; 2] {
		self.size
	}

	pub fn _get_width(&self) -> usize {
		self.size[0]
	}

	pub fn _get_height(&self) -> usize {
		self.size[1]
	}

	pub fn get_topmost_node_x(&self) -> f32 {
		self.topmost_node_x
	}

	pub fn get_size_vec2(&self) -> Vec2 {
		let [w, h] = self.get_size();
		vec2(w as f32, h as f32)
	}

	pub fn texture_id(&self, ctx: &Context) -> TextureId {
		self.texture
			.lock()
			.get_or_insert_with(|| {
				let image: &mut ColorImage = &mut self.image.lock();
				let image = std::mem::take(image);
				ctx.load_texture("", image, TextureOptions::default())
			})
			.id()
	}

	pub fn _show(&self, ui: &mut Ui) -> Response {
		self.show_size(ui, self.get_size_vec2())
	}

	pub fn show_scaled(&self, ui: &mut Ui, scale: f32) -> Response {
		self.show_size(ui, self.get_size_vec2() * scale)
	}

	pub fn show_size(&self, ui: &mut Ui, desired_size: Vec2) -> Response {
		ui.image(self.texture_id(ui.ctx()), desired_size)
	}
}

fn load_svg_tree(tree: &Tree) -> Result<(ColorImage, f32), String> {
	let mut opt = Options::default();
	opt.fontdb.load_system_fonts();

	let pixmap_size = tree.svg_node().size.to_screen_size();
	let width = pixmap_size.width();
	let height = pixmap_size.height();

	let mut pixmap = Pixmap::new(width, height).unwrap();
	render(tree, FitTo::Original, Default::default(), pixmap.as_mut()).unwrap();

	let image = ColorImage::from_rgba_unmultiplied([width as _, height as _], pixmap.data());

	Ok((image, find_topmost_node(&tree.root()).0 as f32))
}

fn find_topmost_node(node: &Node) -> (f64, f64) {
	use PathSegment::*;
	let mut top_x = 0f64;
	let mut top_y = f64::MAX;
	for node in node.children() {
		let readable = node.borrow().clone();
		if let NodeKind::Path(path) = readable {
			for segment in &path.data.0 {
				match segment {
					MoveTo { x, y } | LineTo { x, y } => {
						if *y < top_y {
							top_x = *x;
							top_y = *y;
						}
					}
					_ => {}
				};
			}
		}
		let next = find_topmost_node(&node);
		if next.1 < top_y {
			(top_x, top_y) = next;
		}
	}
	(top_x, top_y)
}
