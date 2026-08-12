use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Length { Px(f32), Percent(f32), Auto }

impl Length {
    pub fn px(v: f32) -> Self { Length::Px(v) }
    pub fn percent(v: f32) -> Self { Length::Percent(v) }
    pub fn resolve(&self, parent: f32) -> f32 {
        match self {
            Length::Px(v) => *v,
            Length::Percent(p) => parent * (*p / 100.0),
            Length::Auto => 0.0,
        }
    }
}

impl Default for Length {
    fn default() -> Self { Length::Auto }
}
