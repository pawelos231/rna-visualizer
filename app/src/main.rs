mod app;
mod protein_map;
use app::*;
use protein_map::*;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Motorola Białka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}
