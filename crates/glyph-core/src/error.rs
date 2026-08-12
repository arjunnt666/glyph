use thiserror::Error;

pub type Result<T> = std::result::Result<T, GlyphError>;

#[derive(Debug, Error)]
pub enum GlyphError {
    #[error("layout error: {0}")] Layout(String),
    #[error("path error: {0}")] Path(String),
    #[error("text error: {0}")] Text(String),
    #[error("render error: {0}")] Render(String),
    #[error("invalid argument: {0}")] Invalid(String),
    #[error("internal: {0}")] Internal(String),
}
