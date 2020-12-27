pub fn lerp(from: f32, to: f32, dt: f32) -> f32 {
    from + dt * (to - from)
}

pub fn remap(n: f32, start1: f32, stop1: f32, start2: f32, stop2: f32) -> f32 {
    ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2
}
