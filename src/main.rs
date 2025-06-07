use clap::Parser;
use std::process::Command;
use std::path::PathBuf;

/// 🌸 Aether Renderer Core
#[derive(Parser, Debug)]
#[command(name = "aether-renderer")]
#[command(about = "Convert PNG frame sequences to WebM/MP4 with alpha", long_about = None)]
struct Args {
    /// Folder containing input PNG files
    #[arg(short, long)]
    input: PathBuf,

    /// Output file name
    #[arg(short, long, default_value = "output.webm")]
    output: String,

    /// Frames per second
    #[arg(short, long, default_value_t = 30)]
    fps: u32,

    /// Output format: webm or mp4
    #[arg(short = 't', long, default_value = "webm")]
    format: String,
}

fn main() {
    let args = Args::parse();

    // Build input pattern path
    let input_pattern = args.input.join("frame_%04d.png");
    let input_str = input_pattern.to_str().unwrap();

    println!("🌿 Rendering {} → {} at {} FPS...", input_str, args.output, args.fps);

    // Build ffmpeg command
    let output_format = args.format.as_str();
    let codec = match output_format {
        "webm" => "libvpx",
        "mp4" => "libx264",
        _ => {
            eprintln!("❌ Unsupported format: {}", output_format);
            return;
        }
    };

    let pix_fmt = if output_format == "webm" {
        "yuva420p" // supports alpha
    } else {
        "yuv420p" // no alpha in mp4
    };

    let status = Command::new("ffmpeg")
        .args([
            "-framerate", &args.fps.to_string(),
            "-i", input_str,
            "-c:v", codec,
            "-pix_fmt", pix_fmt,
            "-auto-alt-ref", "0", // ← required for alpha with libvpx
            "-y",
            &args.output,
        ])

        .status()
        .expect("⚠️ Failed to execute ffmpeg");

    if status.success() {
        println!("✅ Video exported: {}", args.output);
    } else {
        eprintln!("❌ ffmpeg failed. Check your frame pattern or input path.");
    }
}
