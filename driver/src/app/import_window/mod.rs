//! The module that implements [`ImportSettings`]

use egui::*;
use rnalib::ProteinMap;

use importer_view::ImportView;
use settings_view::SettingsView;

mod importer_view;
mod settings_view;

#[derive(Default, Clone)]
pub struct ImportSettings {
	separator: String,
	delete_wrong_chars: bool,
	delete_header: bool,
	header_len: u32,
	from_file: bool,
	input_rna: String,
	path: String,
}

/// Enum representing the states of an import
/// window.
enum View {
	/// A state that allows the user to configure
	/// the import settings.
	Settings(SettingsView),
	/// A state that allows the user to peek
	/// into the importer's progress.
	Import(ImportView),
}

impl Default for View {
	fn default() -> Self {
		Self::Settings(Default::default())
	}
}

/// A ui window that displays an advanced
/// import configuration.
#[derive(Default)]
pub struct ImportWindow {
	pub visible: bool,
	view: View,
}

impl ImportWindow {
	/// Shows self on the ui context.
	pub fn show(&mut self, ctx: &Context) -> Option<Result<ProteinMap, String>> {
		let mut open = self.visible;
		let mut result = None;

		if !open {
			if let View::Import(_) = self.view {
				self.view = View::Settings(Default::default());
			}
		}

		Window::new("Ustawienia importu")
			.open(&mut open)
			.resizable(true)
			.collapsible(false)
			.show(ctx, |ui| match &mut self.view {
				View::Settings(view) => {
					if view.show(ui) {
						self.view = View::Import(ImportView::new(view.settings.clone()));
					}
				}
				View::Import(view) => {
					result = view.show(ui);
				}
			});
		self.visible = open;
		result
	}
}
