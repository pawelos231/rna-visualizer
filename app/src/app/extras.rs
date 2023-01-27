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

	pub fn center_vert_with_margins<T: FnMut(&mut Ui)>(
		ui: &mut Ui,
		draw: &mut T,
	) -> InnerResponse<()> {
		ui.vertical(|ui| {
			let available_height = ui.available_height();
			let measured_height = Self::measure(ui, draw).height();
			ui.add_space((available_height - measured_height) / 2.0);
			draw(ui);
			ui.add_space(ui.available_height());
		})
	}

	pub fn title_bar(ui: &mut Ui, title: &str) {
		ui.label(title);
		ui.separator();
	}
}
