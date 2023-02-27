//! The module that implements [`ProteinSvg`]

use crate::app::svg_image::SvgImage;

/// Holds the protein body svg image in its
/// variants (regular & flipped).
#[derive(Clone)]
pub struct ProteinSvg {
	regular: SvgImage,
	flipped: Option<SvgImage>,
}

impl ProteinSvg {
	/// Create a new [`ProteinSvg`] from the data provided.
	pub const fn new(regular: SvgImage, flipped: Option<SvgImage>) -> Self {
		Self { regular, flipped }
	}

	/// Returns the standard variant of this protein body.
	pub const fn get(&self) -> &SvgImage {
		&self.regular
	}

	/// Returns the flipped variant of this protein body.
	pub const fn get_flipped(&self) -> &SvgImage {
		match &self.flipped {
			Some(flipped) => flipped,
			None => &self.regular,
		}
	}
}
