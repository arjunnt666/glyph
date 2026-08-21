use clap::{Parser, Subcommand};
use glyph_core::{Color, Rgba, Size};
use glyph_paint::{Fill, FillRule, Paint};
use glyph_path::Path;
use glyph_render::{Renderer, SvgRenderer};
use glyph_scene::{Scene, SceneNode};

#[derive(Parser)]
#[command(name = "glyph", about = "glyph 2d graphics tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Draw a couple of rectangles as SVG
    Svg {
        #[arg(long, default_value = "400")]
        width: f32,
        #[arg(long, default_value = "300")]
        height: f32,
    },
    Version,
}

fn fill(r: f32, g: f32, b: f32) -> Paint {
    Paint::Fill(Fill {
        color: Color::Solid(Rgba::rgb(r, g, b)),
        rule: FillRule::NonZero,
    })
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Svg { width, height } => {
            let mut r = SvgRenderer::new();
            let mut scene = Scene::new();
            scene.set_root(SceneNode::Group {
                transform: Default::default(),
                children: vec![
                    SceneNode::Path {
                        path: Path::rect(24.0, 24.0, width - 48.0, 64.0),
                        paint: fill(0.15, 0.45, 0.85),
                        transform: Default::default(),
                    },
                    SceneNode::Path {
                        path: Path::rect(24.0, 104.0, width / 2.0, 48.0),
                        paint: fill(0.9, 0.35, 0.2),
                        transform: Default::default(),
                    },
                ],
            });
            let bytes = r.render(&scene, Size::new(width, height))?;
            println!("{}", String::from_utf8_lossy(&bytes));
        }
        Commands::Version => {
            println!("glyph 0.1.0");
        }
    }
    Ok(())
}
