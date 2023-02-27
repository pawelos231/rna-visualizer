use egui::Vec2;
use std::cmp::Ordering;
use usvg::*;

/// The bounds of an svg image.
#[derive(Clone)]
pub struct Bounds {
	/// Top of the bounds.
	top: Vec2,
}

impl Bounds {
	/// Creates [`Bounds`] from an svg [`Node`].
	pub fn new(source: &Node) -> Self {
		let top = Self::find_bounding_vertical(source, Ordering::Less);
		Self { top }
	}

	/// Gets the top-most point of the bounds.
	pub fn get_top(&self) -> Vec2 {
		self.top
	}

	/// Searches for the bottom/top-most point in an svg [`Node`].
	fn find_bounding_vertical(node: &Node, ord: Ordering) -> Vec2 {
		use PathSegment::*;

		let mut bound = Vec2::new(
			0f32,
			match ord {
				Ordering::Less => f32::MAX,
				Ordering::Greater => f32::MIN,
				Ordering::Equal => 0.5,
			},
		);

		let readable = node.borrow().clone();
		if let NodeKind::Path(path) = readable {
			let mut prev = None;
			for current in &path.data.0 {
				if let LineTo { x, y } = current {
					if let Some(&MoveTo { x, y }) = prev {
						if y.total_cmp(&(bound.y as f64)) == ord {
							bound.x = x as f32;
							bound.y = y as f32;
						}
					}
					if y.total_cmp(&(bound.y as f64)) == ord {
						bound.x = *x as f32;
						bound.y = *y as f32;
					}
				}
				prev = Some(current);
			}
		}

		for node in node.children() {
			let res = Self::find_bounding_vertical(&node, ord);
			if res.y.total_cmp(&bound.y) == ord {
				bound = res;
			}
		}

		bound
	}
}
