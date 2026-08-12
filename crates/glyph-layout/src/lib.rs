//! Flex-inspired layout engine.

pub mod style;
pub mod node;
pub mod engine;

pub use style::{Style, FlexDirection, Justify, Align, FlexWrap};
pub use node::{LayoutNode, NodeId};
pub use engine::LayoutEngine;

use glyph_core::{Result, Rect};

#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub root: NodeId,
    pub bounds: Rect<f32>,
}
