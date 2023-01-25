use std::cmp::Ordering;
use usvg::*;

pub struct SvgBounds {
	left: [f32; 2],
	right: [f32; 2],
	top: [f32; 2],
	bottom: [f32; 2],
}

impl SvgBounds {
	pub fn new(source: &Node) -> Self {
		let top = Self::find_bounding_vertical(source, &Ordering::Less);
		let bottom = Self::find_bounding_vertical(source, &Ordering::Greater);
		let left = Self::find_bounding_horizontal(source, &Ordering::Less);
		let right = Self::find_bounding_horizontal(source, &Ordering::Greater);
		Self {
			top: [top[0] as f32, top[1] as f32],
			bottom: [bottom[0] as f32, bottom[1] as f32],
			left: [left[0] as f32, left[1] as f32],
			right: [right[0] as f32, right[1] as f32],
		}
	}

	pub fn get_top(&self) -> [f32; 2] {
		self.top
	}

	pub fn get_bottom(&self) -> [f32; 2] {
		self.bottom
	}

	pub fn get_left(&self) -> [f32; 2] {
		self.left
	}

	pub fn get_right(&self) -> [f32; 2] {
		self.right
	}

	fn find_bounding_vertical(node: &Node, ord: &Ordering) -> [f64; 2] {
		use PathSegment::*;
		let mut bound_x = 0f64;
		let mut bound_y = match ord {
			Ordering::Less => f64::MAX,
			Ordering::Greater => f64::MIN,
			Ordering::Equal => 0.5,
		};
		for node in node.children() {
			let readable = node.borrow().clone();
			if let NodeKind::Path(path) = readable {
				for segment in &path.data.0 {
					match segment {
						MoveTo { x, y } | LineTo { x, y } => {
							if (*y).total_cmp(&bound_y) == *ord {
								bound_x = *x;
								bound_y = *y;
							}
						}
						_ => {}
					};
				}
			}
			let next = Self::find_bounding_vertical(&node, ord);
			if (next[1]).total_cmp(&bound_y) == *ord {
				bound_x = next[0];
				bound_y = next[1];
			}
		}
		[bound_x, bound_y]
	}

	fn find_bounding_horizontal(node: &Node, ord: &Ordering) -> [f64; 2] {
		use PathSegment::*;
		let mut top_x = match ord {
			Ordering::Less => f64::MAX,
			Ordering::Greater => f64::MIN,
			Ordering::Equal => 0.5,
		};
		let mut top_y = 0f64;
		for node in node.children() {
			let readable = node.borrow().clone();
			if let NodeKind::Path(path) = readable {
				for segment in &path.data.0 {
					match segment {
						MoveTo { x, y } | LineTo { x, y } => {
							if (*x).total_cmp(&top_x) == *ord {
								top_x = *x;
								top_y = *y;
							}
						}
						_ => {}
					};
				}
			}
			let next = Self::find_bounding_vertical(&node, ord);
			if next[0].total_cmp(&top_x) == *ord {
				top_x = next[0];
				top_y = next[1];
			}
		}
		[top_x, top_y]
	}
}
