use anyhow::Result;
use interstellar_triangulum::{AssetLoader, ScriptParser};
use std::path::Path;

fn main() -> Result<()> {
    println!("🎬 Video Engine - Digital Artisan PoC\n");

    // Parse script file from args or use default
    let args: Vec<String> = std::env::args().collect();
    println!("DEBUG: Args: {:?}", args);
    let default_path = "examples/simple.json".to_string();
    let script_path = args.get(1).unwrap_or(&default_path);
    let example_script = Path::new(script_path);

    if example_script.exists() {
        println!("Parsing script: {}", example_script.display());
        let script = ScriptParser::parse_json(example_script)?;

        println!("\n📋 Script Summary:");
        println!("{}", ScriptParser::summarize(&script));

        // Load assets
        let base_path = example_script.parent().unwrap_or_else(|| Path::new("."));
        let mut loader = AssetLoader::new(base_path);

        // Pillar 2: Narrative (Engaging)
        interstellar_triangulum::context::narrative::NarrativeContext::run(&script);

        // Pillar 3: Credibility (Trustworthy)
        interstellar_triangulum::context::credibility::CredibilityContext::run(&script);

        // Pillar 1: Performance (Fast) - Asset Loading & Rendering
        println!("\n🎨 Loading assets...");
        // Pre-load assets for statistics and validation
        for scene in &script.scenes {
            for layer in &scene.layers {
                match layer {
                    interstellar_triangulum::script::Layer::Image { source, .. } => {
                        if let Err(e) = loader.load_image(source) {
                            println!("  ✗ Failed to load image {}: {}", source.display(), e);
                        } else {
                            println!("  ✓ Loaded image: {}", source.display());
                        }
                    }
                    interstellar_triangulum::script::Layer::Video { source, .. } => {
                        if let Err(e) = loader.load_video(source) {
                            println!("  ✗ Failed to load video {}: {}", source.display(), e);
                        } else {
                            println!("  ✓ Loaded video: {}", source.display());
                        }
                    }
                    interstellar_triangulum::script::Layer::Text { font, .. } => {
                        if let Err(e) = loader.load_font(font) {
                            println!("  ✗ Failed to load font {}: {}", font.display(), e);
                        } else {
                            println!("  ✓ Loaded font: {}", font.display());
                        }
                    }
                }
            }
        }

        let output_dir = Path::new("output");
        let use_blender = args
            .windows(2)
            .any(|w| w[0] == "--renderer" && w[1] == "blender");

        interstellar_triangulum::context::performance::PerformanceContext::run(
            &script,
            &mut loader,
            output_dir,
            use_blender,
        )?;

        println!("\n📊 Asset Statistics:");
        println!("  {}", loader.stats());
    } else {
        println!(
            "ℹ️  No example script found at {}",
            example_script.display()
        );
        println!("   Create an example script to test the engine.");
        println!("\n💡 See examples/simple.json for reference format");
    }

    Ok(())
}
