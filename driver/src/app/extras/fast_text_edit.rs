//! The module that implements [`FastTextEdit`]

use egui::*;

pub struct FastTextEdit;

impl FastTextEdit {
	pub fn singleline(ui: &mut Ui, text: &mut String) {
		let id = text as *const String as u32;
		let id = id.to_string().into();

		let has_focus = ui.ctx().memory().focus() == Some(id);
		if text.len() > 50 && !has_focus {
			let mut target_text = String::from(&text[0..50]);
			ui.add(TextEdit::singleline(&mut target_text).id(id));
		} else {
			ui.add(TextEdit::singleline(text).id(id));
		}
	}
}
