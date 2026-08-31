//! Path construction.

use glyph_core::{Point, Result};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathCmd {
    MoveTo(Point<f32>),
    LineTo(Point<f32>),
    QuadTo(Point<f32>, Point<f32>),
    CubicTo(Point<f32>, Point<f32>, Point<f32>),
    ArcTo {
        radii: Point<f32>,
        x_rotation: f32,
        large_arc: bool,
        sweep: bool,
        to: Point<f32>,
    },
    Close,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Path {
    pub cmds: SmallVec<[PathCmd; 8]>,
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn move_to(&mut self, p: Point<f32>) -> &mut Self {
        self.cmds.push(PathCmd::MoveTo(p));
        self
    }
    pub fn line_to(&mut self, p: Point<f32>) -> &mut Self {
        self.cmds.push(PathCmd::LineTo(p));
        self
    }
    pub fn close(&mut self) -> &mut Self {
        self.cmds.push(PathCmd::Close);
        self
    }
    pub fn rect(x: f32, y: f32, w: f32, h: f32) -> Self {
        let mut p = Path::new();
        p.move_to(Point::new(x, y))
            .line_to(Point::new(x + w, y))
            .line_to(Point::new(x + w, y + h))
            .line_to(Point::new(x, y + h))
            .close();
        p
    }
    pub fn to_svg_d(&self) -> String {
        let mut d = String::new();
        for cmd in &self.cmds {
            match cmd {
                PathCmd::MoveTo(p) => d.push_str(&format!("M {} {} ", p.x, p.y)),
                PathCmd::LineTo(p) => d.push_str(&format!("L {} {} ", p.x, p.y)),
                PathCmd::QuadTo(c, p) => d.push_str(&format!("Q {} {} {} {} ", c.x, c.y, p.x, p.y)),
                PathCmd::CubicTo(c1, c2, p) => d.push_str(&format!(
                    "C {} {} {} {} {} {} ",
                    c1.x, c1.y, c2.x, c2.y, p.x, p.y
                )),
                PathCmd::ArcTo { to, .. } => d.push_str(&format!("L {} {} ", to.x, to.y)),
                PathCmd::Close => d.push('Z'),
            }
        }
        d
    }
    pub fn tessellate(&self) -> Result<Vec<Point<f32>>> {
        Ok(self
            .cmds
            .iter()
            .filter_map(|c| match c {
                PathCmd::MoveTo(p) | PathCmd::LineTo(p) => Some(*p),
                _ => None,
            })
            .collect())
    }

    /// Axis-aligned bounds of move/line points. None if the path is empty.
    pub fn bounds(&self) -> Option<(f32, f32, f32, f32)> {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut any = false;
        for cmd in &self.cmds {
            let p = match cmd {
                PathCmd::MoveTo(p) | PathCmd::LineTo(p) => *p,
                PathCmd::QuadTo(_, p) | PathCmd::CubicTo(_, _, p) => *p,
                PathCmd::ArcTo { to, .. } => *to,
                PathCmd::Close => continue,
            };
            any = true;
            min_x = min_x.min(p.x);
            min_y = min_y.min(p.y);
            max_x = max_x.max(p.x);
            max_y = max_y.max(p.y);
        }
        if any {
            Some((min_x, min_y, max_x, max_y))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_has_close() {
        let p = Path::rect(0.0, 0.0, 10.0, 20.0);
        assert!(p.to_svg_d().contains("Z"));
        assert!(p.to_svg_d().contains("M 0 0"));
    }

    #[test]
    fn rect_bounds() {
        let p = Path::rect(10.0, 20.0, 40.0, 15.0);
        let (x0, y0, x1, y1) = p.bounds().unwrap();
        assert_eq!((x0, y0, x1, y1), (10.0, 20.0, 50.0, 35.0));
    }
}

