// src/keypoint.rs
#[derive(Debug, Clone)]
pub struct KeyPoint {
    pub x: f32,
    pub y: f32,
    pub octave: usize,
    pub scale: usize,
    pub orientation: f32,
    pub descriptor: [f32; 128],
}

pub fn refine_keypoint(
    dog: &DoGPyramid,
    octave: usize,
    scale: usize,
    x: usize,
    y: usize,
) -> Option<KeyPoint> {
    // Taylor expansion for sub-pixel refinement
    unimplemented!()
}