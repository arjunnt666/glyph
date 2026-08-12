use std::collections::HashMap;
use glyph_core::{Result, Rect, Size, Point};
use crate::node::{LayoutNode, NodeId, ComputedLayout};
use crate::style::Style;
use crate::LayoutResult;

pub struct LayoutEngine {
    nodes: HashMap<NodeId, LayoutNode>,
    next_id: u32,
}

impl LayoutEngine {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), next_id: 1 }
    }

    pub fn create_node(&mut self, style: Style) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        self.nodes.insert(id, LayoutNode::new(id, style));
        id
    }

    pub fn add_child(&mut self, parent: NodeId, child: NodeId) {
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(child);
        }
    }

    pub fn layout(&mut self, root: NodeId, available: Size<f32>) -> Result<LayoutResult> {
        self.layout_node(root, Point::zero(), available)?;
        let bounds = self.nodes.get(&root).and_then(|n| n.layout.as_ref())
            .map(|l| l.bounds)
            .unwrap_or_else(|| Rect::new(0.0, 0.0, available.width, available.height));
        Ok(LayoutResult { root, bounds })
    }

    fn layout_node(&mut self, id: NodeId, origin: Point<f32>, available: Size<f32>) -> Result<()> {
        let children = self.nodes.get(&id).map(|n| n.children.clone()).unwrap_or_default();
        let mut y = origin.y;
        for child in &children {
            let child_size = Size::new(available.width, 40.0);
            self.layout_node(*child, Point::new(origin.x, y), child_size)?;
            y += 44.0;
        }
        if let Some(node) = self.nodes.get_mut(&id) {
            node.layout = Some(ComputedLayout {
                bounds: Rect { origin, size: available },
                content_size: available,
            });
        }
        Ok(())
    }
}

impl Default for LayoutEngine {
    fn default() -> Self { Self::new() }
}
