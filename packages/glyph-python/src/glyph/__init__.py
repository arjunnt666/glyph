"""Glyph Python bindings (stub)."""

class Size:
    def __init__(self, width: float, height: float):
        self.width = width
        self.height = height

class SvgRenderer:
    def render(self, scene=None, size: Size = None) -> bytes:
        w = size.width if size else 400
        h = size.height if size else 300
        return f'''<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}">
  <!-- glyph python stub -->
</svg>'''.encode()

def version() -> str:
    return "0.1.0"

__all__ = ["Size", "SvgRenderer", "version"]
