pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
	(1.0 - t) * a + t * b
}

pub fn inv_lerp(a: f32, b: f32, v: f32) -> f32 {
	(v - a) / (b - a)
}

pub fn qerp(a: f32, b: f32, t: f32) -> f32 {
	let t = lerp(t * t, 1.0 - (f32::powi(1.0 - t, 2)), t);
	lerp(a, b, t)
}
