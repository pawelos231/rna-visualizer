use resvg::*;
use rnalib::Protein;
use tiny_skia::*;
use usvg::{FitTo, Options, Tree};

mod assets;

pub fn make_vis(protein: &Protein) -> Result<(), ()> {
	let codons = protein.get_codons();
	let acids = codons.iter().map(|x| x.get_acid_shorthand());

	let mut map = Pixmap::new(1000, 1000).ok_or(())?;

	let mut start_x = 0f32;
	let start_y = map.height() as f32 / 2.0;

	let options = Options::default();

	let base = Tree::from_str(assets::BASE, &options).unwrap();
	let base_height = base.size.height() as f32;
	let base_width = base.size.width() as f32;

	for acid in acids {
		let svg_src = match assets::get_acid_svg_by_shorthand(acid) {
			Some(src) => src,
			None => panic!("Unsupported acid!"),
		};

		let characteristic = Tree::from_str(svg_src, &options)
			.unwrap_or_else(|_| panic!("Invalid acid svg: {acid}"));

		let root = Transform::from_translate(start_x, start_y);
		resvg::render(&base, FitTo::Original, root, map.as_mut());

		let root = Transform::from_translate(start_x + 76.0, start_y + base_height - 10.0);
		resvg::render(&characteristic, FitTo::Original, root, map.as_mut());

		start_x += base_width;
	}

	map.save_png("test.png").expect("zapis sie wyjebał");

	Ok(())
}
