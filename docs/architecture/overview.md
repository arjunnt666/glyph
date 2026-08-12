# Architecture

Glyph splits the classic graphics pipeline into focused crates:

1. **core** — points, rects, transforms, colors, errors
2. **layout** — flex-style node tree → computed boxes
3. **path** — path commands + tessellation stubs
4. **text** — runs, metrics, naive shaping
5. **paint** — fills, strokes, gradients
6. **scene** — retained graph of drawable nodes
7. **render** — backends (SVG, canvas command list, future CPU raster)
8. **wasm** — thin exports for browser use

Everything is still early. Tessellation, real text shaping, and a software rasterizer are the obvious next pieces.
