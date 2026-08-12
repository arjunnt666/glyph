export interface Size { width: number; height: number; }

export class GlyphCanvas {
  constructor(private canvas: HTMLCanvasElement) {}
  clear() {
    const ctx = this.canvas.getContext("2d");
    if (ctx) ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }
  render(_scene: unknown) {}
}

export function createLayoutEngine() {
  return {
    layout(_root: unknown, _available: Size) {
      return { bounds: { x: 0, y: 0, width: 0, height: 0 } };
    },
  };
}

export default { GlyphCanvas, createLayoutEngine };
