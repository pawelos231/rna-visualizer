use egui::*;

mod colors;
mod fast_text_edit;

use colors::*;
pub use fast_text_edit::*;

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
		let mut rect = Rect::from_min_size(
			ui.next_widget_position(),
			Vec2::new(ui.available_size().x, 20.0),
		);

		rect.min.x -= 1.0;
		rect.max.x = rect.min.x + ui.available_width();
		let rect = rect.expand2(Vec2::new(8.0, 3.0));

		ui.painter()
			.rect_filled(rect, Rounding::none(), Colors::TITLEBAR_BG);

		ui.painter().rect_stroke(
			rect,
			Rounding::none(),
			Stroke::new(1.0, ui.style().visuals.faint_bg_color),
		);

		ui.allocate_ui_at_rect(rect.shrink(10.0), |ui| {
			ui.add_space(-5.0);
			ui.add(Label::new(title).wrap(false));
		});

		ui.add_space(5.0);
	}
}
