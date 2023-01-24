use egui::*;
use mutex::*;
use resvg::render;
use tiny_skia::Pixmap;
use usvg::*;

pub struct SvgImage {
	size: [usize; 2],
	image: Mutex<ColorImage>,
	texture: Mutex<Option<TextureHandle>>,
	options: TextureOptions,
}

impl SvgImage {
	pub fn from_color_image(image: ColorImage) -> Self {
		Self {
			size: image.size,
			image: Mutex::new(image),
			texture: Default::default(),
			options: Default::default(),
		}
	}

	pub fn from_svg_tree(tree: &Tree) -> Self {
		Self::from_color_image(load_svg_tree(tree, FitTo::Original).unwrap())
	}

	pub fn get_size(&self) -> [usize; 2] {
		self.size
	}

	pub fn get_width(&self) -> usize {
		self.size[0]
	}

	pub fn get_height(&self) -> usize {
		self.size[1]
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
				ctx.load_texture("", image, self.options)
			})
			.id()
	}

	pub fn show(&self, ui: &mut Ui) -> Response {
		self.show_size(ui, self.get_size_vec2())
	}

	pub fn show_scaled(&self, ui: &mut Ui, scale: f32) -> Response {
		self.show_size(ui, self.get_size_vec2() * scale)
	}

	pub fn show_size(&self, ui: &mut Ui, desired_size: Vec2) -> Response {
		ui.image(self.texture_id(ui.ctx()), desired_size)
	}
}

pub fn load_svg_tree(tree: &Tree, fit_to: FitTo) -> Result<ColorImage, String> {
	let mut opt = Options::default();
	opt.fontdb.load_system_fonts();

	let pixmap_size = tree.svg_node().size.to_screen_size();
	let width = pixmap_size.width() as f32;
	let height = pixmap_size.height() as f32;
	let [w, h] = match fit_to {
		FitTo::Original => [width as u32, height as u32],
		FitTo::Size(w, h) => [w, h],
		FitTo::Height(h) => [(width * (h as f32 / height)) as u32, h],
		FitTo::Width(w) => [w, (height * (w as f32 / width)) as u32],
		FitTo::Zoom(z) => [(width * z) as u32, (height * z) as u32],
	};

	let mut pixmap = Pixmap::new(w, h).unwrap();
	render(tree, fit_to, Default::default(), pixmap.as_mut()).unwrap();

	let image = ColorImage::from_rgba_unmultiplied([w as _, h as _], pixmap.data());

	Ok(image)
}
