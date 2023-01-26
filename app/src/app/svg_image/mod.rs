use egui::*;
use mutex::*;
use resvg::render;
use tiny_skia::Pixmap;
use usvg::*;

mod svg_bounds;
use svg_bounds::SvgBounds;

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

	pub fn show(&self, ui: &mut Ui, scale: f32) -> Response {
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
