//! Text runs, metrics, and simple wrapping stubs.

use glyph_core::{Result, Size};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub line_height: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self { font_family: "sans-serif".into(), font_size: 16.0, font_weight: 400, line_height: 1.2 }
    }
}

#[derive(Debug, Clone)]
pub struct TextRun {
    pub text: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone)]
pub struct ShapedGlyph {
    pub cluster: u32,
    pub x_advance: f32,
    pub y_advance: f32,
    pub x_offset: f32,
    pub y_offset: f32,
}

#[derive(Debug, Clone)]
pub struct ShapedText {
    pub glyphs: Vec<ShapedGlyph>,
    pub width: f32,
    pub height: f32,
}

pub fn shape(run: &TextRun) -> Result<ShapedText> {
    let mut glyphs = Vec::new();
    let advance = run.style.font_size * 0.6;
    for (i, _) in run.text.chars().enumerate() {
        glyphs.push(ShapedGlyph {
            cluster: i as u32, x_advance: advance, y_advance: 0.0, x_offset: 0.0, y_offset: 0.0,
        });
    }
    let width = glyphs.len() as f32 * advance;
    let height = run.style.font_size * run.style.line_height;
    Ok(ShapedText { glyphs, width, height })
}

pub fn measure(run: &TextRun, max_width: Option<f32>) -> Result<Size<f32>> {
    let shaped = shape(run)?;
    let w = max_width.unwrap_or(shaped.width).min(shaped.width);
    Ok(Size::new(w, shaped.height))
}
