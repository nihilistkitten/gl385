//! Handles WebGl data.
use crate::{Error, Result};

pub struct Data {
    vertices: Vec<(f32, f32, f32)>,
    colors: Vec<(f32, f32, f32, f32)>,
    matrix: [f32; 12],
}

impl Data {
    fn flatten() {}
}
