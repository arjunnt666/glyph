# Text

the current shaper is intentionally naive.

what works in the skeleton:
- one glyph per unicode scalar
- fixed advance based on font size
- simple width measure

what does not:
- bidirectional reordering
- complex script shaping
- font fallback chains
- kerning tables

if you need real text, plug in a harfbuzz-class shaper behind the same TextRun API.
