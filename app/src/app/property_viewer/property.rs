use egui::*;
use rnalib::Protein;

pub trait Property {
	fn evaluate(protein: &Protein, x: f32) -> f32;

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

	fn show(protein: &Protein, ui: &mut Ui, start: f32, end: f32) {
		let rect = ui.available_rect_before_wrap().shrink(10.0);
		if rect.width() <= 0.0 || rect.height() <= 0.0 {
			return;
		}

		Self::show_bg(ui, rect.expand(3.0));

		let mut values = Vec::new();
		let mut sample = start;
		let mut min = f32::MAX;
		let mut max = f32::MIN;
		while sample < end {
			let value = Self::evaluate(protein, sample / end);
			values.push(value);
			max = max.max(value);
			min = min.min(value);
			sample += (10.0 / rect.width()).abs().max(0.01);
		}
		let scale = 1.0 / (min - max).abs();

		let painter = ui.painter();
		painter.line_segment(
			[rect.left_center(), rect.right_center()],
			Stroke::new(1.0, ui.style().visuals.faint_bg_color),
		);

		let mut previous = None;
		let stroke = Stroke::new(1.0, Self::get_color());

		let length = values.len();
		for value in values.iter().enumerate() {
			let eval_x = value.0 as f32 / length as f32;
			let eval_y = 1.0 - (*value.1 * scale).clamp(0.0, 1.0);
			let eval_p = Pos2::new(
				(1.0 - eval_x) * rect.left() + eval_x * rect.right(),
				rect.top() + eval_y * rect.height(),
			);
			painter.line_segment([previous.unwrap_or(eval_p), eval_p], stroke);
			previous = Some(eval_p);
		}

		let m_val = max.abs().max(min.abs());
		ui.allocate_ui_at_rect(Rect::from_points(&[rect.left_top()]), |ui| {
			ui.label(RichText::new(format!("{m_val:.2}")).weak())
		});
		ui.allocate_ui_at_rect(
			Rect::from_points(&[rect.left_bottom() - Vec2::Y * 18.0]),
			|ui| ui.label(RichText::new(format!("-{m_val:.2}")).weak()),
		);
	}
}

pub struct Mass;
impl Property for Mass {
	fn evaluate(protein: &Protein, x: f32) -> f32 {
		protein.get_mass()
	}
}
pub struct Pi;
impl Property for Pi {
	fn evaluate(protein: &Protein, x: f32) -> f32 {
		protein.get_isoletric_point()
	}
}

pub struct Sanity;
impl Property for Sanity {
	fn evaluate(_: &Protein, x: f32) -> f32 {
		1.0 - x.powi(3)
	}
}

pub struct Hydro;
impl Property for Hydro {
	fn get_color() -> Color32 {
		Color32::from_rgb(0, 116, 217)
	}

	fn evaluate(protein: &Protein, x: f32) -> f32 {
		let n = (x * protein.len() as f32) as usize;
		protein.get_phob(n)
	}
}
