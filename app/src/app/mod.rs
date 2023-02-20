use std::rc::Rc;

use eframe::{epaint::Shadow, Frame};
use egui::*;
use rnalib::ProteinMap;

mod import_window;
use import_window::ImportWindow;

mod protein_selector;
use protein_selector::ProteinSelector;

mod protein_viewer;
use protein_viewer::ProteinViewer;

mod extras;
use extras::FastTextEdit;

mod property_viewer;
use property_viewer::PropertyViewer;

mod svg_image;

mod fonts;

pub type ProteinCollection = ProteinMap;

#[derive(Default)]
pub struct App {
	rna: String,
	proteins: ProteinCollection,
	import_window: ImportWindow,
	protein_selector: ProteinSelector,
	protein_viewer: ProteinViewer,
	property_viewer: PropertyViewer,
}

impl App {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		cc.egui_ctx.set_pixels_per_point(1.3);

		let mut fonts = egui::FontDefinitions::default();
		fonts.font_data.insert(
			"Regular".to_owned(),
			egui::FontData::from_static(fonts::REGULAR),
		);
		fonts
			.font_data
			.insert("Bold".to_owned(), egui::FontData::from_static(fonts::BOLD));
		fonts
			.families
			.entry(egui::FontFamily::Proportional)
			.or_default()
			.insert(0, "Regular".to_owned());
		cc.egui_ctx.set_fonts(fonts);

		let font_id = FontId {
			size: 14.0,
			family: FontFamily::Proportional,
		};

		let mut style = egui::Style::default();
		style.visuals.window_shadow = Shadow::small_dark();
		style.visuals.window_rounding = Rounding::same(3.0);
		style.animation_time = 0.0;

		style.text_styles.insert(TextStyle::Heading, font_id);

		cc.egui_ctx.set_style(style);

		Self::default()
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		if let Some(map) = self.import_window.show(ctx) {
			self.proteins = map;
		}

		TopBottomPanel::top("TOP").show(ctx, |ui| {
			ui.add_space(2.0);
			ui.horizontal(|ui| {
				ui.label("Ciąg RNA:");
				FastTextEdit::singleline(ui, &mut self.rna);
				if ui.button("Wczytaj").clicked() {
					self.proteins = ProteinMap::parse(self.rna.to_string());
					self.protein_selector.clear_cache();
				};
				if ui.button("Wytnij niepoprawne znaki").clicked() {
					self.rna
						.retain(|x| "AGCUT ".contains(x.to_ascii_uppercase()));
				}
				if ui.button("Zaawansowany import...").clicked() {
					self.import_window.visible = true;
				}
			});
			ui.add_space(2.0);
		});

		SidePanel::left("left_panel")
			.min_width(300.0)
			.show(ctx, |ui| {
				if let Some(selection) = self.protein_selector.show(ui, &self.proteins) {
					self.protein_viewer.protein = Some(Rc::clone(&selection));
					self.property_viewer.set(selection);
				}
			});

		CentralPanel::default().show(ctx, |ui| {
			let available = ui.available_height();
			let height = TopBottomPanel::top("DISPLAY_TOP")
				.resizable(true)
				.show(ctx, |ui| {
					self.protein_viewer.show(ui);
				})
				.response
				.rect
				.height();
			TopBottomPanel::bottom("DISPLAY_BOTTOM")
				.resizable(false)
				.exact_height(available - height + 25.0)
				.show(ctx, |ui| {
					self.property_viewer.show(ui);
				});
		});
	}
}
