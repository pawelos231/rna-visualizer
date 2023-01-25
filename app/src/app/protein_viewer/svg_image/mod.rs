use std::cmp::Ordering;

use egui::*;
use mutex::*;
use resvg::render;
use tiny_skia::Pixmap;
use usvg::*;

pub struct SvgImage {
	size: [usize; 2],
	image: Mutex<ColorImage>,
	texture: Mutex<Option<TextureHandle>>,
	bounds: SvgBounds,
}

impl SvgImage {
	pub fn from_svg_tree(tree: &Tree) -> Self {
		let tree_data = load_svg_tree(tree).unwrap();
		Self::from_svg_data(tree_data)
	}

	pub fn get_size(&self) -> [usize; 2] {
		self.size
	}

	pub fn get_width(&self) -> usize {
		self.size[0]
	}

	pub fn _get_height(&self) -> usize {
		self.size[1]
	}

	pub fn get_bounds(&self) -> &SvgBounds {
		&self.bounds
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

	fn from_svg_data(data: SvgData) -> Self {
		Self {
			size: data.image.size,
			image: Mutex::new(data.image),
			texture: Default::default(),
			bounds: data.bounds,
		}
	}
}

pub struct SvgBounds {
	left: [f32; 2],
	right: [f32; 2],
	top: [f32; 2],
	bottom: [f32; 2],
}

impl SvgBounds {
	pub fn new(source: &Node) -> Self {
		let top = Self::find_bounding_vertical(source, &Ordering::Less);
		let bottom = Self::find_bounding_vertical(source, &Ordering::Greater);
		let left = Self::find_bounding_horizontal(source, &Ordering::Less);
		let right = Self::find_bounding_horizontal(source, &Ordering::Greater);
		Self {
			top: [top[0] as f32, top[1] as f32],
			bottom: [bottom[0] as f32, bottom[1] as f32],
			left: [left[0] as f32, left[1] as f32],
			right: [right[0] as f32, right[1] as f32],
		}
	}

	pub fn get_top(&self) -> [f32; 2] {
		self.top
	}

	pub fn get_bottom(&self) -> [f32; 2] {
		self.bottom
	}

	pub fn get_left(&self) -> [f32; 2] {
		self.left
	}

	pub fn get_right(&self) -> [f32; 2] {
		self.right
	}

	fn find_bounding_vertical(node: &Node, ord: &Ordering) -> [f64; 2] {
		use PathSegment::*;
		let mut bound_x = 0f64;
		let mut bound_y = match ord {
			Ordering::Less => f64::MAX,
			Ordering::Greater => f64::MIN,
			Ordering::Equal => 0.5,
		};
		for node in node.children() {
			let readable = node.borrow().clone();
			if let NodeKind::Path(path) = readable {
				for segment in &path.data.0 {
					match segment {
						MoveTo { x, y } | LineTo { x, y } => {
							if (*y).total_cmp(&bound_y) == *ord {
								bound_x = *x;
								bound_y = *y;
							}
						}
						_ => {}
					};
				}
			}
			let next = Self::find_bounding_vertical(&node, ord);
			if (next[1]).total_cmp(&bound_y) == *ord {
				bound_x = next[0];
				bound_y = next[1];
			}
		}
		[bound_x, bound_y]
	}

	fn find_bounding_horizontal(node: &Node, ord: &Ordering) -> [f64; 2] {
		use PathSegment::*;
		let mut top_x = match ord {
			Ordering::Less => f64::MAX,
			Ordering::Greater => f64::MIN,
			Ordering::Equal => 0.5,
		};
		let mut top_y = 0f64;
		for node in node.children() {
			let readable = node.borrow().clone();
			if let NodeKind::Path(path) = readable {
				for segment in &path.data.0 {
					match segment {
						MoveTo { x, y } | LineTo { x, y } => {
							if (*x).total_cmp(&top_x) == *ord {
								top_x = *x;
								top_y = *y;
							}
						}
						_ => {}
					};
				}
			}
			let next = Self::find_bounding_vertical(&node, ord);
			if next[0].total_cmp(&top_x) == *ord {
				top_x = next[0];
				top_y = next[1];
			}
		}
		[top_x, top_y]
	}
}

struct SvgData {
	pub image: ColorImage,
	pub bounds: SvgBounds,
}

fn load_svg_tree(tree: &Tree) -> Result<SvgData, String> {
	let mut opt = Options::default();
	opt.fontdb.load_system_fonts();

	let pixmap_size = tree.svg_node().size.to_screen_size();
	let width = pixmap_size.width();
	let height = pixmap_size.height();

	let mut pixmap = Pixmap::new(width, height).unwrap();
	render(tree, FitTo::Original, Default::default(), pixmap.as_mut()).unwrap();

	let image = ColorImage::from_rgba_unmultiplied([width as _, height as _], pixmap.data());
	let bounds = SvgBounds::new(&tree.root());

	Ok(SvgData { image, bounds })
}
