//! WASM entry points (stub).

use glyph_core::Size;
use glyph_render::{SvgRenderer, Renderer};
use glyph_scene::Scene;

#[no_mangle]
pub extern "C" fn glyph_render_empty_svg(width: f32, height: f32) -> u32 {
    let mut r = SvgRenderer::new();
    let scene = Scene::new();
    let _ = r.render(&scene, Size::new(width, height));
    0
}

#[no_mangle]
pub extern "C" fn glyph_version() -> *const std::os::raw::c_char {
    b"0.1.0\0".as_ptr() as *const _
}
