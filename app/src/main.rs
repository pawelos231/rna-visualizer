// #![windows_subsystem = "windows"]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
const ICON: &[u8] = include_bytes!("../icon/icon.bin");

mod app;
mod fonts;
mod protein_map;

use app::*;

fn main() {
	let native_options = eframe::NativeOptions {
		icon_data: Some(eframe::IconData {
			rgba: ICON.to_vec(),
			width: 32,
			height: 32,
		}),
		vsync: true,
		..Default::default()
	};
	eframe::run_native(
		"Motorola Science Cup 2022/23 - Bioinformatyka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}
