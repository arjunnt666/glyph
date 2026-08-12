use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgba { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self { Self { r, g, b, a } }
    pub fn rgb(r: f32, g: f32, b: f32) -> Self { Self::new(r, g, b, 1.0) }
    pub fn black() -> Self { Self::rgb(0.0, 0.0, 0.0) }
    pub fn white() -> Self { Self::rgb(1.0, 1.0, 1.0) }
    pub fn transparent() -> Self { Self::new(0.0, 0.0, 0.0, 0.0) }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Color {
    Solid(Rgba),
    LinearGradient { start: crate::Point<f32>, end: crate::Point<f32>, stops: Vec<(f32, Rgba)> },
    RadialGradient { center: crate::Point<f32>, radius: f32, stops: Vec<(f32, Rgba)> },
}

impl From<Rgba> for Color {
    fn from(c: Rgba) -> Self { Color::Solid(c) }
}
