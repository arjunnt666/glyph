//! Render backends: SVG, canvas command list, software raster (stubs).

use glyph_core::{Result, Size};
use glyph_scene::Scene;

pub mod svg;
pub mod canvas;

pub use svg::SvgRenderer;
pub use canvas::CanvasRenderer;

pub trait Renderer {
    fn render(&mut self, scene: &Scene, size: Size<f32>) -> Result<Vec<u8>>;
}
