use glyph_core::{Result, Size};
use glyph_scene::Scene;
use crate::Renderer;

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn new() -> Self { Self }
}

impl Default for SvgRenderer {
    fn default() -> Self { Self::new() }
}

impl Renderer for SvgRenderer {
    fn render(&mut self, _scene: &Scene, size: Size<f32>) -> Result<Vec<u8>> {
        let svg = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <!-- glyph scene render stub -->
</svg>"#,
            size.width, size.height, size.width, size.height
        );
        Ok(svg.into_bytes())
    }
}
