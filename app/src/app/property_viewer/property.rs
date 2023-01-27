use egui::*;
use rnalib::Protein;

pub trait Property {
	fn evaluate<T: AsRef<Protein>>(&self, protein: T, normalized_t: f32) -> f32;

	fn get_color() -> Color32 {
		Color32::from_rgb(255, 65, 54)
	}

	fn show_bg(ui: &mut Ui, rect: Rect) {
		let painter = ui.painter();

		painter.rect(
			rect,
			Rounding::default(),
			Color32::from_gray(32),
			Stroke::new(2.0, ui.style().visuals.faint_bg_color),
		);
	}

	fn show<T: AsRef<Protein>>(
		&self,
		protein: T,
		ui: &mut Ui,
		normalized_start: f32,
		normalized_end: f32,
	) {
		let rect = ui.available_rect_before_wrap().shrink(10.0);
		Self::show_bg(ui, rect.expand(3.0));

		let painter = ui.painter();
		let mut sample = normalized_start;

		let mut values = Vec::new();
		while sample < normalized_end {
			values.push(self.evaluate(&protein, sample / normalized_end));
			sample += 10.0 / rect.width();
		}

		let mut max_val = 0.0;
		for value in &values {
			if *value > max_val {
				max_val = *value;
			}
		}

		let mut previous = None;
		let stroke = Stroke::new(1.0, Self::get_color());

		let length = values.len();
		for value in values.iter().enumerate() {
			let eval_x = value.0 as f32 / length as f32;
			let eval_y = *value.1 / max_val;
			let eval_p = Pos2::new(
				(1.0 - eval_x) * rect.min.x + eval_x * rect.max.x,
				eval_y * rect.min.y + (1.0 - eval_y) * rect.max.y,
			);
			painter.line_segment([previous.unwrap_or(eval_p), eval_p], stroke);
			previous = Some(eval_p);
		}
	}
}

pub struct Sanity;
impl Property for Sanity {
	fn evaluate<T: AsRef<Protein>>(&self, _: T, normalized_t: f32) -> f32 {
		1.0 - normalized_t.powi(3)
	}
}
