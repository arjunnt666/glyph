use glyph_core::{Rect, Size};
use crate::style::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: NodeId,
    pub style: Style,
    pub children: Vec<NodeId>,
    pub layout: Option<ComputedLayout>,
}

#[derive(Debug, Clone)]
pub struct ComputedLayout {
    pub bounds: Rect<f32>,
    pub content_size: Size<f32>,
}

impl LayoutNode {
    pub fn new(id: NodeId, style: Style) -> Self {
        Self { id, style, children: Vec::new(), layout: None }
    }
}
