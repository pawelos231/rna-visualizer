mod app;
use app::*;

fn main() {
	let native_options = eframe::NativeOptions::default();
	eframe::run_native(
		"Motorola Białka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}
