use clap::{Parser, Subcommand};
use glyph_core::Size;
use glyph_render::{SvgRenderer, Renderer};
use glyph_scene::Scene;

#[derive(Parser)]
#[command(name = "glyph", about = "glyph 2d graphics tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Emit a minimal empty SVG
    Svg {
        #[arg(long, default_value = "400")]
        width: f32,
        #[arg(long, default_value = "300")]
        height: f32,
    },
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Svg { width, height } => {
            let mut r = SvgRenderer::new();
            let scene = Scene::new();
            let bytes = r.render(&scene, Size::new(width, height))?;
            println!("{}", String::from_utf8_lossy(&bytes));
        }
        Commands::Version => {
            println!("glyph 0.1.0");
        }
    }
    Ok(())
}
