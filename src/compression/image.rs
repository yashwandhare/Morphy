/// image.rs - image compression with preset and custom size options

use std::io::Cursor;
use std::path::Path;
use std::ffi::OsStr;

use console::style;
use dialoguer::Select;
use image::codecs::jpeg::JpegEncoder;
use image::DynamicImage;

use crate::ui::theme::Theme;

// compression presets
struct Preset {
    name: &'static str,
    quality: u8,
    max_dim: u32,
    reduction: &'static str,
}

const PRESETS: [Preset; 3] = [
    Preset { name: "Light",      quality: 90, max_dim: 1920, reduction: "~30%" },
    Preset { name: "Medium",     quality: 85, max_dim: 1280, reduction: "~50%" },
    Preset { name: "Aggressive", quality: 75, max_dim: 720,  reduction: "~70%" },
];

const VALID_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "webp", "bmp", "tiff"];

// --- entry point ---

pub fn compress_image(path: &str, args: Option<&[String]>) {
    if !validate_image(path) {
        return;
    }

    let img = match load_image(path) {
        Some(img) => img,
        None => return,
    };

    let original_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    show_info(path, &img, original_size);

    let choice = get_compression_choice(args);

    let output_path = match choice {
        3 => {
            // custom target size
            let target_kb = match get_target_size(args) {
                Some(kb) => kb,
                None => return,
            };
            compress_to_target(path, &img, target_kb)
        }
        _ => {
            // preset compression
            compress_with_preset(path, &img, &PRESETS[choice])
        }
    };

    if let Some(output) = output_path {
        show_comparison(original_size, &output);
    }
}

// --- validation ---

fn validate_image(path: &str) -> bool {
    let ext = get_extension(path);
    if !VALID_EXTENSIONS.contains(&ext.as_str()) {
        println!(
            "{}",
            style("Invalid file type. Supported: PNG, JPG, WEBP, BMP, TIFF").fg(Theme::ERROR),
        );
        return false;
    }
    true
}

// --- load image ---

fn load_image(path: &str) -> Option<DynamicImage> {
    match image::open(path) {
        Ok(img) => Some(img),
        Err(e) => {
            println!(
                "{}",
                style(format!("Failed to open image: {}", e)).fg(Theme::ERROR),
            );
            None
        }
    }
}

// --- display file info ---

fn show_info(path: &str, img: &DynamicImage, size_bytes: u64) {
    let size_kb = size_bytes as f64 / 1024.0;
    let size_mb = size_kb / 1024.0;

    let size_str = if size_mb >= 1.0 {
        format!("{:.2} MB", size_mb)
    } else {
        format!("{:.2} KB", size_kb)
    };

    let filename = Path::new(path)
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_string_lossy();

    println!();
    println!("{}", style("IMAGE COMPRESSION").fg(Theme::HEADER).bold());

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.add_row(vec!["File", &filename.to_string()]);
    table.add_row(vec!["Size", &size_str]);
    table.add_row(vec!["Dimensions", &format!("{}x{}", img.width(), img.height())]);
    println!("{}", table);
}

// --- ask how to compress ---

fn get_compression_choice(args: Option<&[String]>) -> usize {
    if let Some(args) = args {
        if let Some(arg) = args.get(0) {
            return match arg.to_lowercase().as_str() {
                "light" => 0,
                "medium" => 1,
                "aggressive" => 2,
                _ => 3, // custom size
            };
        }
    }

    println!();
    println!("{}", style("Select compression level:").fg(Theme::HEADER).bold());

    let options = &[
        format!("Light       quality: {}  |  max: {}px  |  {}", PRESETS[0].quality, PRESETS[0].max_dim, PRESETS[0].reduction),
        format!("Medium      quality: {}  |  max: {}px  |  {}", PRESETS[1].quality, PRESETS[1].max_dim, PRESETS[1].reduction),
        format!("Aggressive  quality: {}  |  max: {}px   |  {}", PRESETS[2].quality, PRESETS[2].max_dim, PRESETS[2].reduction),
        "Custom      Enter target size in KB".to_string(),
    ];

    Select::new()
        .items(options)
        .default(1)
        .interact()
        .unwrap_or(1)
}

// --- ask for custom target size ---

fn get_target_size(args: Option<&[String]>) -> Option<u64> {
    let input = if let Some(args) = args {
        args.get(0).cloned().unwrap_or_else(|| "500".to_string())
    } else {
        dialoguer::Input::new()
            .with_prompt(format!("{}", style("Enter target size in KB").fg(Theme::INFO)))
            .default("500".to_string())
            .interact_text()
            .unwrap_or_default()
    };

    // clean up input (remove "kb" suffix, spaces)
    let cleaned = input.to_lowercase().replace("kb", "").trim().to_string();

    match cleaned.parse::<u64>() {
        Ok(kb) if kb > 0 => Some(kb),
        _ => {
            println!(
                "{}",
                style("Invalid input. Enter a number like 500, 250, 100").fg(Theme::ERROR),
            );
            None
        }
    }
}

// --- compress with a named preset ---

fn compress_with_preset(path: &str, img: &DynamicImage, preset: &Preset) -> Option<String> {
    let output_path = get_output_path(path);

    println!(
        "{}",
        style(format!("Compressing with {} settings...", preset.name)).fg(Theme::INFO),
    );

    let compressed = apply_preset(img, preset);
    save_image(&compressed, &output_path, preset.quality, path);

    Some(output_path)
}

// --- compress to a target file size ---

fn compress_to_target(path: &str, img: &DynamicImage, target_kb: u64) -> Option<String> {
    let target_bytes = target_kb * 1024;
    let tolerance = (target_bytes as f64 * 0.05) as u64;

    println!(
        "{}",
        style("Optimizing to target size...").fg(Theme::INFO),
    );

    let result = binary_search_quality(img, target_bytes, tolerance);

    // show warning if target wasn't achievable
    if let Some(ref warning) = result.warning {
        println!();
        println!("{}", style(format!("⚠ {}", warning)).fg(Theme::ERROR));
    }

    let output_path = get_output_path(path);

    // apply the winning settings
    let final_img = if result.resized {
        result.image.unwrap_or_else(|| img.clone())
    } else {
        img.clone()
    };

    save_image(&final_img, &output_path, result.quality, path);

    println!();
    println!(
        "{}",
        style(format!("Quality: {} | Resized: {}", result.quality, result.resized_dims)).fg(Theme::INFO),
    );

    Some(output_path)
}

// --- result of the binary search ---

struct SearchResult {
    quality: u8,
    warning: Option<String>,
    resized: bool,
    resized_dims: String,
    image: Option<DynamicImage>,
}

// --- binary search for the right quality/scale combo ---

fn binary_search_quality(img: &DynamicImage, target_bytes: u64, tolerance: u64) -> SearchResult {
    let mut min_quality: u8 = 50;
    let mut max_quality: u8 = 95;
    let mut best_quality: u8 = 85;
    let mut best_size: u64 = u64::MAX;

    let original_width = img.width();
    let original_height = img.height();
    let mut scale_factor: f64 = 1.0;
    let mut resized = false;
    let mut resized_dims = format!("{}x{}", original_width, original_height);

    // first try quality alone
    for _ in 0..15 {
        let quality = (min_quality + max_quality) / 2;
        let size = get_compressed_size(img, quality, scale_factor);

        if (size as i64 - target_bytes as i64).unsigned_abs() <= tolerance {
            return SearchResult {
                quality,
                warning: None,
                resized,
                resized_dims,
                image: None,
            };
        }

        if size < best_size {
            best_quality = quality;
            best_size = size;
        }

        if size > target_bytes {
            if quality == 0 { break; }
            max_quality = quality.saturating_sub(1);
        } else {
            min_quality = quality + 1;
        }

        if min_quality > max_quality {
            break;
        }
    }

    // if quality alone isn't enough, try scaling down
    if best_size > target_bytes && scale_factor >= 1.0 {
        let scale_factors = [0.75, 0.50, 0.35, 0.25, 0.15];

        for &factor in &scale_factors {
            let new_width = (original_width as f64 * factor) as u32;
            let new_height = (original_height as f64 * factor) as u32;
            let test_size = get_compressed_size(img, min_quality, factor);

            if test_size <= target_bytes + tolerance {
                scale_factor = factor;
                resized = true;
                resized_dims = format!("{}x{}", new_width, new_height);
                best_quality = min_quality;
                best_size = test_size;
                break;
            }
        }
    }

    // check if target was achievable
    let warning = if best_size > target_bytes + tolerance {
        Some(format!("Target not achievable. Best size: {} KB", best_size / 1024))
    } else {
        None
    };

    // build resized image if needed
    let resized_img = if resized {
        let new_width = (original_width as f64 * scale_factor) as u32;
        let new_height = (original_height as f64 * scale_factor) as u32;
        Some(img.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3))
    } else {
        None
    };

    SearchResult {
        quality: best_quality,
        warning,
        resized,
        resized_dims,
        image: resized_img,
    }
}

// --- compress in memory and return the size ---

fn get_compressed_size(img: &DynamicImage, quality: u8, scale_factor: f64) -> u64 {
    let mut working = img.clone();

    // resize if needed
    if scale_factor < 1.0 {
        let new_width = (working.width() as f64 * scale_factor) as u32;
        let new_height = (working.height() as f64 * scale_factor) as u32;
        working = working.resize_exact(new_width, new_height, image::imageops::FilterType::Lanczos3);
    }

    // convert to rgb (jpeg can't handle transparency)
    let rgb = working.to_rgb8();

    // compress to memory buffer
    let mut buffer = Cursor::new(Vec::new());
    let encoder = JpegEncoder::new_with_quality(&mut buffer, quality);
    if rgb.write_with_encoder(encoder).is_err() {
        return u64::MAX;
    }

    buffer.into_inner().len() as u64
}

// --- resize to fit within a preset's max dimension ---

fn apply_preset(img: &DynamicImage, preset: &Preset) -> DynamicImage {
    let max_dim = preset.max_dim;
    let (w, h) = (img.width(), img.height());

    if w > max_dim || h > max_dim {
        let (new_w, new_h) = if w > h {
            let new_w = max_dim;
            let new_h = (h as f64 * (max_dim as f64 / w as f64)) as u32;
            (new_w, new_h)
        } else {
            let new_h = max_dim;
            let new_w = (w as f64 * (max_dim as f64 / h as f64)) as u32;
            (new_w, new_h)
        };
        img.resize_exact(new_w, new_h, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    }
}

// --- save with the right format handling ---

fn save_image(img: &DynamicImage, output_path: &str, quality: u8, original_path: &str) {
    let ext = get_extension(original_path);

    // pick save format based on original
    match ext.as_str() {
        "png" => {
            if let Err(e) = img.save_with_format(output_path, image::ImageFormat::Png) {
                println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
            }
        }
        "webp" => {
            if let Err(e) = img.save_with_format(output_path, image::ImageFormat::WebP) {
                println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
            }
        }
        _ => {
            // default to jpeg for jpg, jpeg, bmp, tiff
            let rgb = img.to_rgb8();
            let mut file = match std::fs::File::create(output_path) {
                Ok(f) => f,
                Err(e) => {
                    println!("{}", style(format!("Failed to create file: {}", e)).fg(Theme::ERROR));
                    return;
                }
            };
            let encoder = JpegEncoder::new_with_quality(&mut file, quality);
            if let Err(e) = rgb.write_with_encoder(encoder) {
                println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
            }
        }
    }
}

// --- build the output filename ---

fn get_output_path(path: &str) -> String {
    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();
    let ext = Path::new(path)
        .extension()
        .unwrap_or(OsStr::new("jpg"))
        .to_string_lossy();

    parent.join(format!("{}_compressed.{}", name, ext))
        .to_string_lossy()
        .to_string()
}

// --- show before/after comparison ---

fn show_comparison(original_bytes: u64, output_path: &str) {
    let compressed_bytes = std::fs::metadata(output_path)
        .map(|m| m.len())
        .unwrap_or(0);

    let original_kb = original_bytes as f64 / 1024.0;
    let compressed_kb = compressed_bytes as f64 / 1024.0;

    let original_str = if original_kb >= 1024.0 {
        format!("{:.2} MB", original_kb / 1024.0)
    } else {
        format!("{:.2} KB", original_kb)
    };

    let compressed_str = if compressed_kb >= 1024.0 {
        format!("{:.2} MB", compressed_kb / 1024.0)
    } else {
        format!("{:.2} KB", compressed_kb)
    };

    let reduction = if original_bytes > 0 {
        ((original_bytes - compressed_bytes) as f64 / original_bytes as f64) * 100.0
    } else {
        0.0
    };

    println!();
    println!("{}", style("✓ Done!").fg(Theme::SUCCESS).bold());

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.add_row(vec!["Before", &original_str]);
    table.add_row(vec!["After", &compressed_str]);
    table.add_row(vec!["Reduced", &format!("{:.1}%", reduction)]);
    println!("{}", table);

    println!();
    println!(
        "{}",
        style(format!("Saved: {}", output_path)).fg(Theme::TEXT),
    );
}

// --- helper: get file extension lowercase ---

fn get_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase()
}
