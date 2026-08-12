use serde::{Deserialize, Serialize};
use glyph_core::Length;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexDirection { Row, Column, RowReverse, ColumnReverse }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Justify { FlexStart, FlexEnd, Center, SpaceBetween, SpaceAround, SpaceEvenly }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Align { FlexStart, FlexEnd, Center, Stretch, Baseline }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlexWrap { NoWrap, Wrap, WrapReverse }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Style {
    pub direction: FlexDirection,
    pub justify_content: Justify,
    pub align_items: Align,
    pub align_self: Option<Align>,
    pub flex_wrap: FlexWrap,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Length,
    pub width: Length,
    pub height: Length,
    pub min_width: Length,
    pub min_height: Length,
    pub max_width: Length,
    pub max_height: Length,
    pub margin: Edges,
    pub padding: Edges,
    pub gap: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Edges { pub top: Length, pub right: Length, pub bottom: Length, pub left: Length }

impl Default for Edges {
    fn default() -> Self {
        Self { top: Length::Px(0.0), right: Length::Px(0.0), bottom: Length::Px(0.0), left: Length::Px(0.0) }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Row,
            justify_content: Justify::FlexStart,
            align_items: Align::Stretch,
            align_self: None,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            width: Length::Auto,
            height: Length::Auto,
            min_width: Length::Px(0.0),
            min_height: Length::Px(0.0),
            max_width: Length::Px(f32::INFINITY),
            max_height: Length::Px(f32::INFINITY),
            margin: Edges::default(),
            padding: Edges::default(),
            gap: 0.0,
        }
    }
}
