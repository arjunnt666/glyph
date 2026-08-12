use glyph_core::{Result, Size};
use glyph_scene::Scene;
use crate::Renderer;

#[derive(Debug, Clone)]
pub enum CanvasCmd {
    Save, Restore, Transform([f32; 6]), BeginPath, MoveTo(f32, f32), LineTo(f32, f32),
    ClosePath, Fill, Stroke, FillText(String, f32, f32),
}

pub struct CanvasRenderer {
    pub commands: Vec<CanvasCmd>,
}

impl CanvasRenderer {
    pub fn new() -> Self { Self { commands: Vec::new() } }
}

impl Default for CanvasRenderer {
    fn default() -> Self { Self::new() }
}

impl Renderer for CanvasRenderer {
    fn render(&mut self, _scene: &Scene, _size: Size<f32>) -> Result<Vec<u8>> {
        self.commands.clear();
        self.commands.push(CanvasCmd::Save);
        self.commands.push(CanvasCmd::Restore);
        Ok(Vec::new())
    }
}
