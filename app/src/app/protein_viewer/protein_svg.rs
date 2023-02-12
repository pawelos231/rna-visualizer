use crate::app::svg_image::SvgImage;

#[derive(Clone)]
pub struct ProteinSvg {
	regular: SvgImage,
	flipped: Option<SvgImage>,
}

impl ProteinSvg {
	pub const fn new(regular: SvgImage, flipped: Option<SvgImage>) -> Self {
		Self { regular, flipped }
	}

	pub const fn get(&self) -> &SvgImage {
		&self.regular
	}

	pub const fn get_flipped(&self) -> &SvgImage {
		match &self.flipped {
			Some(flipped) => flipped,
			None => &self.regular,
		}
	}
}
