use egui::*;

pub struct Extras;

impl Extras {
	pub fn measure<T: FnMut(&mut Ui)>(ui: &mut Ui, draw: &mut T) -> Rect {
		let next = ui.next_widget_position();
		let clip = ui.clip_rect();
		ui.set_clip_rect(Rect::NOTHING);
		let rect = ui
			.scope(|ui| {
				draw(ui);
			})
			.response
			.rect;
		ui.set_clip_rect(clip);
		ui.allocate_rect(Rect::from_min_size(next, Vec2::ZERO), Sense::hover());
		rect
	}

	pub fn title_bar(ui: &mut Ui, title: &str) {
		ui.label(title);
		ui.separator();
	}
}
