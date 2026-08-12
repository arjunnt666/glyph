//! Brushes, strokes, and fill styles.

use glyph_core::{Color, Rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_array: Vec<f32>,
    pub dash_offset: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineCap { Butt, Round, Square }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineJoin { Miter, Round, Bevel }

impl Default for Stroke {
    fn default() -> Self {
        Self {
            color: Color::Solid(Rgba::black()),
            width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
            dash_array: vec![],
            dash_offset: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub color: Color,
    pub rule: FillRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillRule { NonZero, EvenOdd }

impl Default for Fill {
    fn default() -> Self {
        Self { color: Color::Solid(Rgba::black()), rule: FillRule::NonZero }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Paint {
    Fill(Fill),
    Stroke(Stroke),
    FillAndStroke { fill: Fill, stroke: Stroke },
}
