use egui::*;
use rnalib::AminoString;
use rnalib::Protein;

pub trait Property {
	fn evaluate(protein: &AminoString, x: f32) -> f32;

	fn get_name(&self) -> String;

	fn get_unit(&self) -> String;

	fn get_color() -> Color32 {
		Color32::from_rgb(255, 65, 54)
	}

	fn generate_cache(protein: &Protein) -> [f32; 100] {
		let mut cache = [0.0; 100];
		let unit = protein.len() as f32 / 100.0;
		(0..100).for_each(|i| {
			let x = unit * i as f32;
			cache[i] = Self::evaluate(&protein.slice(0, 1 + x as usize), 1.0);
		});
		cache
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

	fn show_samples(&self, ui: &mut Ui, samples: [f32; 100]) {
		//TODO: add interpolation, start y values from 0

		let rect = ui.available_rect_before_wrap().shrink(10.0);
		if rect.width() <= 0.0 || rect.height() <= 0.0 {
			return;
		}

		Self::show_bg(ui, rect.expand(3.0));

		let mut min = f32::MAX;
		let mut max = f32::MIN;
		for value in samples {
			max = max.max(value);
			min = min.min(value);
		}

		let scale = 1.0 / (min - max).abs();

		let painter = ui.painter();
		painter.line_segment(
			[rect.left_center(), rect.right_center()],
			Stroke::new(1.0, ui.style().visuals.faint_bg_color),
		);

		let mut previous = None;
		let stroke = Stroke::new(1.0, Self::get_color());

		let length = samples.len();
		for value in samples.iter().enumerate() {
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

	fn show(&self, protein: &Protein, ui: &mut Ui, start: f32, end: f32) {
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
	fn get_name(&self) -> String {
		String::from("Masa")
	}

	fn get_unit(&self) -> String {
		String::from("Dalton")
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_mass()
	}
}
pub struct Pi;
impl Property for Pi {
	fn get_name(&self) -> String {
		String::from("Punkt izoelektryczny")
	}

	fn get_unit(&self) -> String {
		String::from("is")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(221, 221, 221)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_isoletric_point()
	}
}
pub struct NetCharge;
impl Property for NetCharge {
	fn get_name(&self) -> String {
		String::from("Suma ładunków")
	}

	fn get_unit(&self) -> String {
		String::from("zakłada ph = 7")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(52, 186, 186)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_neutral_charge()
	}
}
pub struct Extinction;
impl Property for Extinction {
	fn get_name(&self) -> String {
		String::from("Współczynnik absorbcji")
	}

	fn get_unit(&self) -> String {
		String::from("M⁻¹ * cm⁻¹")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(255, 220, 0)
	}

	fn evaluate(protein: &AminoString, _x: f32) -> f32 {
		protein.get_ext() as f32
	}
}

pub struct Hydro;
impl Property for Hydro {
	fn get_name(&self) -> String {
		String::from("Indeks hydrofobowy")
	}

	fn get_unit(&self) -> String {
		String::from("Kcal * mol⁻¹")
	}

	fn get_color() -> Color32 {
		Color32::from_rgb(0, 116, 217)
	}

	fn evaluate(protein: &AminoString, x: f32) -> f32 {
		let n = (x * protein.len() as f32) as usize;
		protein.get_phob(n)
	}
}
