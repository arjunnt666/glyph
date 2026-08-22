Want two colored rectangles as SVG? that is the current demo.

```bash
cargo build -p glyph-cli
./target/debug/glyph svg --width 80 --height 60
```

Behind that command: points, rects, path command lists, `Path::rect` turned into an SVG `d`, and a scene walker that emits `<path>` with fill. `rect` can tell you if a point is inside it. tests live under `cargo test --workspace`.

Text shaping is the thing I am not going to fake. no bidi, no gpu tessellation you would ship. layout is flex-ish, not a browser.

MIT
