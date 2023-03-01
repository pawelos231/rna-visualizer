//! The module that defines [`Property`]

use egui::*;
use rnalib::AminoString;

mod charge;
mod extinction;
mod hydro;
mod mass;
mod pi;

pub use charge::*;
pub use extinction::*;
pub use hydro::*;
pub use mass::*;
pub use pi::*;

use super::math::inv_lerp;
use super::math::lerp;
use super::math::qerp;

pub type PointsCache = [f32; 100];

pub trait Property {
	/// Evaluate the property value of [`AminoString`]
	/// at x value.
	fn evaluate(protein: &AminoString, x: f32) -> f32;

	/// Returns the name of the property.
	fn get_name(&self) -> String;

	/// Returns the unit of the property.
	fn get_unit(&self) -> String;

	/// Returns whether the property can evaluate to negative numbers.
	fn get_show_negative(&self) -> bool {
		true
	}

	/// Returns the color in which the property should be plotted.
	fn get_color() -> Color32 {
		Color32::from_rgb(255, 65, 54)
	}

	/// Generates a [`PointsCache`] cache by sampling an [`AminoString`]
	/// along its entire length.
	fn sample(protein: &AminoString) -> PointsCache {
		let mut cache = [0.0; 100];
		let unit = protein.len() as f32 / 100.0;
		(0..100).for_each(|i| {
			let x = unit * i as f32;
			cache[i] = Self::evaluate(&protein.slice(0, 1 + x as usize), 1.0);
		});
		cache[cache.len() - 1] = Self::evaluate(protein, 1.0);
		cache
	}

	/// A helper function that draws a chart depicting
	/// sampled values to the ui.
	fn show(&self, ui: &mut Ui, samples: PointsCache) {
		let rect = ui.available_rect_before_wrap().shrink(10.0);
		if rect.width() <= 0.0 || rect.height() <= 0.0 {
			return;
		}

		let cursor = ui.input().pointer.hover_pos();

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
			Stroke::new(1.0, ui.style().visuals.code_bg_color),
		);

		let mut previous = None;
		let color = Self::get_color();
		let stroke = Stroke::new(1.1, Self::get_color());
		let fill = Color32::from_rgb(
			lerp(color.r() as f32, 32.0, 0.85) as u8,
			lerp(color.g() as f32, 32.0, 0.85) as u8,
			lerp(color.b() as f32, 32.0, 0.85) as u8,
		);
		let fill = Stroke::new(1.0, fill);

		let zero_y = match self.get_show_negative() {
			true => rect.center().y,
			false => rect.bottom(),
		};

		let cx = inv_lerp(
			rect.left(),
			rect.right(),
			cursor.map(|x| x.x).unwrap_or_default(),
		);
		let hover_index_x = cx * samples.len() as f32;
		let mut hover_p = Pos2::ZERO;
		let mut hover_val = 0.0;

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

			painter.line_segment([p, Pos2::new(p.x, zero_y)], fill);
			painter.line_segment([previous.unwrap_or(p), p], stroke);
			previous = Some(p);

			if hover_index_x as u32 == index_x as u32 {
				hover_p = p;
				hover_val = next;
			}
		}

		if let Some(cursor) = cursor {
			let hovered = rect.contains(cursor.to_vec2().to_pos2());
			let enabled = ui.is_enabled();
			if hovered && enabled {
				painter.circle_filled(hover_p, 3.0, stroke.color);
				egui::containers::show_tooltip_at_pointer(
					ui.ctx(),
					ui.id().with("_HOVER_TOOLTIP"),
					|ui| {
						ui.label(format!("{0:.2}", hover_val));
					},
				);
			}
		}

		ui.allocate_ui_at_rect(Rect::from_points(&[rect.left_top()]), |ui| {
			ui.label(RichText::new(format!("{max_val:.2}")).weak())
		});

		ui.allocate_ui_at_rect(
			Rect::from_points(&[rect.left_bottom() - Vec2::Y * 18.0]),
			|ui| ui.label(RichText::new(format!("{min_val:.2}")).weak()),
		);
	}

	/// A helper function that draws a chart's background.
	fn show_bg(ui: &mut Ui, rect: Rect) {
		let painter = ui.painter();

		painter.rect(
			rect,
			Rounding::default(),
			Color32::from_gray(28),
			Stroke::new(2.0, Color32::from_gray(22)),
		);
	}
}
