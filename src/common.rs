

#[must_use]
pub fn lerp(from: f32, to: f32, p: f32) -> f32 {
    from.mul_add(1.0 - p, to * p)
}

#[must_use]
pub fn map(value: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((value - start1) / (stop1 - start1)).mul_add(stop2 - start2, start2)
}

#[must_use]
pub fn norm(value: f32, start: f32, stop: f32) -> f32 {
    map(value, start, stop, 0.0, 1.0)
}
