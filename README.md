# glyph

2d layout + paths without pretending to be a browser.

points, rects, path commands, a scene graph sketch. text shaping is still the thing we refuse to fake.

## works today

- rect contains-point
- path command lists
- `glyph version`

## does not work yet

- complex text / bidi
- gpu tessellation you would ship

## try it

```bash
cargo test --workspace
cargo build -p glyph-cli
./target/debug/glyph version
```

## license

mit.
