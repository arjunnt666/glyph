use crate::Renderer;
use glyph_core::{Color, Result, Size};
use glyph_paint::Paint;
use glyph_path::Path;
use glyph_scene::{Scene, SceneNode};

pub struct SvgRenderer;

impl SvgRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn fill_css(paint: &Paint) -> String {
    let color = match paint {
        Paint::Fill(f) | Paint::FillAndStroke { fill: f, .. } => &f.color,
        Paint::Stroke(_) => return "none".into(),
    };
    match color {
        Color::Solid(c) => format!(
            "rgb({},{},{})",
            (c.r * 255.0) as u8,
            (c.g * 255.0) as u8,
            (c.b * 255.0) as u8
        ),
        _ => "#888".into(),
    }
}

fn emit_path(out: &mut String, path: &Path, paint: &Paint) {
    out.push_str(&format!(
        "  <path d=\"{}\" fill=\"{}\" />\n",
        path.to_svg_d(),
        fill_css(paint)
    ));
}

fn walk(node: &SceneNode, out: &mut String) {
    match node {
        SceneNode::Group { children, .. } => {
            for c in children {
                walk(c, out);
            }
        }
        SceneNode::Path { path, paint, .. } => emit_path(out, path, paint),
        SceneNode::Text { run, .. } => {
            out.push_str(&format!(
                "  <text x=\"8\" y=\"20\" font-size=\"14\">{}</text>\n",
                escape(&run.text)
            ));
        }
        SceneNode::LayoutRef { .. } => {}
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl Renderer for SvgRenderer {
    fn render(&mut self, scene: &Scene, size: Size<f32>) -> Result<Vec<u8>> {
        let mut body = String::new();
        if let Some(root) = &scene.root {
            walk(root, &mut body);
        }
        let svg = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}">
{body}</svg>"#,
            w = size.width,
            h = size.height,
            body = body
        );
        Ok(svg.into_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glyph_core::Rgba;
    use glyph_paint::{Fill, FillRule, Paint};
    use glyph_path::Path;
    use glyph_scene::SceneNode;

    #[test]
    fn renders_rect() {
        let mut scene = Scene::new();
        scene.set_root(SceneNode::Path {
            path: Path::rect(10.0, 10.0, 40.0, 20.0),
            paint: Paint::Fill(Fill {
                color: Color::Solid(Rgba::rgb(1.0, 0.0, 0.0)),
                rule: FillRule::NonZero,
            }),
            transform: Default::default(),
        });
        let bytes = SvgRenderer::new()
            .render(&scene, Size::new(100.0, 80.0))
            .unwrap();
        let s = String::from_utf8(bytes).unwrap();
        assert!(s.contains("<path"));
        assert!(s.contains("rgb(255,0,0)"));
    }
}
