# glyph

2d layout + paths without pretending to be a browser.

points, rects, path commands, a scene graph that actually becomes SVG. text shaping is still the thing we refuse to fake.

## works today

- rect contains-point
- path command lists and `Path::rect` to SVG `d`
- SVG renderer walks the scene and emits `<path>` with fill
- `glyph svg` prints two colored rectangles

## does not work yet

- complex text / bidi
- gpu tessellation you would ship

## try it

```bash
cargo test --workspace
cargo build -p glyph-cli
./target/debug/glyph svg --width 80 --height 60
```

## license

mit.
