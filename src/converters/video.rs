/// video.rs - converts video to gif using ffmpeg

use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;

use console::style;
use dialoguer::Select;

use crate::ui::theme::Theme;

// --- check if ffmpeg is installed ---

fn check_ffmpeg() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

// --- entry point: validate, ask settings, convert ---

pub fn conv_video(path: &str, args: Option<&[String]>) {
    println!();
    println!("{}", style("VIDEO TO GIF").fg(Theme::HEADER).bold());

    // check dependencies
    println!("{}", style("Checking FFmpeg...").fg(Theme::INFO));
    if !check_ffmpeg() {
        println!(
            "{} FFmpeg is not installed or not in PATH.",
            style("ERROR:").fg(Theme::ERROR).bold(),
        );
        return;
    }
    println!("{}", style("✓ FFmpeg detected.").fg(Theme::SUCCESS));

    // check supported format
    let ext = Path::new(path)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase();

    let supported = ["mp4", "avi", "mov", "mkv", "webm", "flv"];
    if !supported.contains(&ext.as_str()) {
        println!(
            "{}",
            style(format!("Unsupported video format: .{}", ext)).fg(Theme::ERROR),
        );
        return;
    }

    // get conversion settings
    let (fps, width) = if let Some(args) = args {
        let f = args.get(0).map(|s| s.as_str()).unwrap_or("10");
        let w = args.get(1).map(|s| s.as_str()).unwrap_or("480");
        (f, w)
    } else {
        let fps_options = &["10", "24", "30"];
        let fps_idx = Select::new()
            .with_prompt(format!("{}", style("Select FPS").fg(Theme::HEADER)))
            .items(fps_options)
            .default(0)
            .interact()
            .unwrap_or(0);
        let f = fps_options[fps_idx];

        let width_options = &["320", "480", "720", "1080"];
        let width_idx = Select::new()
            .with_prompt(format!("{}", style("Select Width").fg(Theme::HEADER)))
            .items(width_options)
            .default(1)
            .interact()
            .unwrap_or(1);
        let w = width_options[width_idx];
        
        (f, w)
    };

    convert_to_gif(path, fps, width);
}

// --- two-pass ffmpeg conversion: palette → gif ---

fn convert_to_gif(path: &str, fps: &str, width: &str) {
    if !Path::new(path).is_file() {
        println!("{}", style("File not found").fg(Theme::ERROR));
        return;
    }

    // define paths
    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();

    let output_gif = parent.join(format!("{}.gif", name));
    let palette_png = parent.join(format!("{}_palette.png", name));

    let filters = format!("fps={},scale={}:-1:flags=lanczos", fps, width);

    println!(
        "{}",
        style("Processing (this might take a moment)...").fg(Theme::INFO).bold(),
    );

    // generate palette
    let palette_result = Command::new("ffmpeg")
        .args([
            "-y", "-i", path,
            "-vf", &format!("{},palettegen", filters),
            palette_png.to_str().unwrap_or("palette.png"),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if palette_result.map(|s| s.success()).unwrap_or(false) == false {
        println!(
            "{}",
            style("Error: Palette generation failed.").fg(Theme::ERROR).bold(),
        );
        cleanup_file(&palette_png);
        return;
    }

    // generate final gif
    let gif_result = Command::new("ffmpeg")
        .args([
            "-y", "-i", path,
            "-i", palette_png.to_str().unwrap_or("palette.png"),
            "-lavfi", &format!("{} [x]; [x][1:v] paletteuse", filters),
            output_gif.to_str().unwrap_or("output.gif"),
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    // cleanup palette
    cleanup_file(&palette_png);

    match gif_result {
        Ok(status) if status.success() => {
            let size_kb = std::fs::metadata(&output_gif)
                .map(|m| m.len() as f64 / 1024.0)
                .unwrap_or(0.0);
            println!();
            println!(
                "{} GIF saved at: {} ({:.1} KB)",
                style("Success!").fg(Theme::SUCCESS).bold(),
                output_gif.display(),
                size_kb,
            );
        }
        _ => {
            println!(
                "{}",
                style("Error: Conversion failed.").fg(Theme::ERROR).bold(),
            );
        }
    }
}

// --- helper: remove temp file if it exists ---

fn cleanup_file(path: &Path) {
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
}
