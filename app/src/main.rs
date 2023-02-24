// #![windows_subsystem = "windows"]

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const ICON: &[u8] = include_bytes!("../icon/icon.bin");
use egui::*;

mod app;

use app::*;

fn main() {
	let native_options = eframe::NativeOptions {
		icon_data: Some(eframe::IconData {
			rgba: ICON.to_vec(),
			width: 32,
			height: 32,
		}),
		initial_window_size: Some(Vec2::new(1280.0, 720.0)),
		vsync: true,
		..Default::default()
	};

	eframe::run_native(
		"Motorola Science Cup 2022/23 - Bioinformatyka",
		native_options,
		Box::new(|cc| Box::new(App::new(cc))),
	);
}
