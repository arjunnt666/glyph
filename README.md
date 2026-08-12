# glyph

2d vector graphics + layout engine.

flex-ish layout. paths. text. paint. scene graph.  
wasm friendly. not a full browser. not skia. not cairo. just the bits that keep showing up when you try to draw stuff without crying.

## why

every time you need “draw this box with rounded corners and some text that wraps” you end up either:
- pulling in half a browser
- writing the same geometry code for the 4th time
- accepting that your ui looks like 2003

glyph tries to be the small, boring middle ground.

## what it does (in theory)

- layout: flex + absolute-ish positioning, margins, padding, gaps
- paths: move/line/curve/arc, fill, stroke, even-odd
- text: shaping stubs, font metrics, simple wrapping
- paint: solid, linear/radial gradients, image patterns (stubs)
- scene: retained-mode node tree, dirty regions (stub)
- export: svg, canvas commands, raw vertices

## status

early skeleton. the types and the overall shape are there.  
the actual rasterizer / tessellator / text shaper are still “todo with a capital T”.

don’t put production money on it. do put curiosity on it.

## crates

- `glyph-core` — geometry, colors, errors
- `glyph-layout` — flex-style layout engine
- `glyph-path` — path building + tessellation stubs
- `glyph-text` — text runs + metrics
- `glyph-paint` — brushes + gradients
- `glyph-scene` — retained scene graph
- `glyph-render` — backend traits (cpu, canvas, svg)
- `glyph-wasm` — wasm bindings
- `glyph-cli` — quick svg/png export tool

js and python packages under `packages/`.

## license

mit. steal it. improve it. don’t sue me when the text looks weird.
