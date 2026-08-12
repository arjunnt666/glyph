//! Core geometry, colors, and error types for Glyph.

pub mod geom;
pub mod color;
pub mod error;
pub mod size;

pub use geom::{Point, Rect, Size, Transform, Affine};
pub use color::{Color, Rgba};
pub use error::{GlyphError, Result};
pub use size::Length;

pub type Pt = Point<f32>;
pub type Sz = Size<f32>;
pub type Rc = Rect<f32>;
