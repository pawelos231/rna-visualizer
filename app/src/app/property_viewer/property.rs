use egui::*;
use rnalib::AminoString;
use rnalib::Protein;

use super::math::inv_lerp;
use super::math::lerp;
use super::math::qerp;

pub trait Property {
	fn evaluate(protein: &AminoString, x: f32) -> f32;

	fn get_name(&self) -> String;

	fn get_unit(&self) -> String;

	fn get_show_negative(&self) -> bool {
		true
	}

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
		let rect = ui.available_rect_before_wrap().shrink(10.0);
		if rect.width() <= 0.0 || rect.height() <= 0.0 {
			return;
		}

		ui.vertical_centered_justified(|ui| ui.label(""));

		Self::show_bg(ui, rect.expand(3.0));

		let mut min = f32::MAX;
		let mut max = f32::MIN;
		for value in samples {
			max = max.max(value);
			min = min.min(value);
		}

		let max_val = max.abs().max(min.abs());
		let min_val = match self.get_show_negative() {
			true => -max_val,
			false => 0.0,
		};

		let painter = ui.painter();
		painter.line_segment(
			[rect.left_center(), rect.right_center()],
			Stroke::new(1.0, ui.style().visuals.faint_bg_color),
		);

		let mut previous = None;
		let stroke = Stroke::new(1.0, Self::get_color());

		let end = rect.width() as u32;
		for i in 0..end {
			let x = i as f32 / end as f32;

			let index_x = x * samples.len() as f32;
			let local_t = index_x - index_x.floor();
			let prev = samples[index_x as usize];
			let next = samples[(index_x as usize + 1).min(samples.len() - 1)];

			let value = qerp(prev, next, local_t);
			let t = inv_lerp(min_val, max_val, value);
			let p = Pos2::new(
				lerp(rect.left(), rect.right(), x),
				lerp(rect.bottom(), rect.top(), t),
			);

			painter.line_segment([previous.unwrap_or(p), p], stroke);
			previous = Some(p);
		}

		ui.allocate_ui_at_rect(Rect::from_points(&[rect.left_top()]), |ui| {
			ui.label(RichText::new(format!("{max_val:.2}")).weak())
		});

		ui.allocate_ui_at_rect(
			Rect::from_points(&[rect.left_bottom() - Vec2::Y * 18.0]),
			|ui| ui.label(RichText::new(format!("{min_val:.2}")).weak()),
		);
	}

	fn show(&self, protein: &Protein, ui: &mut Ui) {
		let mut samples = [0.0; 100];
		(0..100).for_each(|i| {
			let value = Self::evaluate(protein, i as f32 / 100.0);
			samples[i] = value;
		});
		self.show_samples(ui, samples);
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

	fn get_show_negative(&self) -> bool {
		false
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

	fn get_show_negative(&self) -> bool {
		false
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
		String::from("zakłada ph 7")
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

	fn get_show_negative(&self) -> bool {
		false
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
