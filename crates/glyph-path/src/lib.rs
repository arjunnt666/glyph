//! Path construction and tessellation stubs.

use glyph_core::{Point, Result};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathCmd {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    QuadTo(Point<f32>, Point<f32>),
    CubicTo(Point<f32>, Point<f32>, Point<f32>),
    ArcTo { radii: Point<f32>, x_rotation: f32, large_arc: bool, sweep: bool, to: Point<f32> },
    Close,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Path {
    pub cmds: SmallVec<[PathCmd; 8]>,
}

impl Path {
    pub fn new() -> Self { Self::default() }
    pub fn move_to(&mut self, p: Point<f32>) -> &mut Self {
        self.cmds.push(PathCmd::MoveTo(p)); self
    }
    pub fn line_to(&mut self, p: Point<f32>) -> &mut Self {
        self.cmds.push(PathCmd::LineTo(p)); self
    }
    pub fn close(&mut self) -> &mut Self {
        self.cmds.push(PathCmd::Close); self
    }
    pub fn tessellate(&self) -> Result<Vec<Point<f32>>> {
        Ok(Vec::new())
    }
}
