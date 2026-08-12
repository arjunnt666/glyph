//! Retained-mode scene graph.

use glyph_core::{Affine, Result};
use glyph_path::Path;
use glyph_text::TextRun;
use glyph_paint::Paint;
use glyph_layout::NodeId;

#[derive(Debug, Clone)]
pub enum SceneNode {
    Group { transform: Affine, children: Vec<SceneNode> },
    Path { path: Path, paint: Paint, transform: Affine },
    Text { run: TextRun, paint: Paint, transform: Affine },
    LayoutRef { node_id: NodeId },
}

#[derive(Debug, Default)]
pub struct Scene {
    pub root: Option<SceneNode>,
}

impl Scene {
    pub fn new() -> Self { Self::default() }
    pub fn set_root(&mut self, node: SceneNode) { self.root = Some(node); }
    pub fn visit<F>(&self, mut f: F) -> Result<()>
    where F: FnMut(&SceneNode) {
        if let Some(root) = &self.root { visit_node(root, &mut f); }
        Ok(())
    }
}

fn visit_node<F>(node: &SceneNode, f: &mut F) where F: FnMut(&SceneNode) {
    f(node);
    if let SceneNode::Group { children, .. } = node {
        for c in children { visit_node(c, f); }
    }
}
