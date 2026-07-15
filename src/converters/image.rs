/// image.rs - shows image info and converts formats

use std::path::Path;
use std::ffi::OsStr;

use console::style;
use dialoguer::Select;
use image::DynamicImage;

use crate::ui::theme::Theme;

// --- main entry: show info, ask format, convert ---

pub fn conv_image(path: &str, args: Option<&[String]>) {
    let ext = get_extension(path);

    // check if format is supported
    let supported = ["png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp", "ico"];
    if !supported.contains(&ext.as_str()) {
        println!("{}", style(format!("Invalid file type. Supported: PNG, JPG, WEBP, BMP, GIF, TIFF, ICO")).fg(Theme::ERROR));
        return;
    }

    // load the image
    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            println!("{}", style(format!("Failed to open image: {}", e)).fg(Theme::ERROR));
            return;
        }
    };

    // read image details
    let (width, height) = (img.width(), img.height());
    let file_size = match std::fs::metadata(path) {
        Ok(m) => m.len(),
        Err(_) => 0,
    };
    let size_kb = file_size as f64 / 1024.0;
    let filename = Path::new(path)
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_string_lossy();

    // show file info
    println!();
    println!("{}", style("IMAGE CONVERSION").fg(Theme::HEADER).bold());

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.add_row(vec!["Filename", &filename]);
    table.add_row(vec!["Format", &ext.to_uppercase()]);
    table.add_row(vec!["Mode", color_mode_str(&img)]);
    table.add_row(vec!["Size", &format!("{}x{}", width, height)]);
    table.add_row(vec!["File size", &format!("{:.2} KB", size_kb)]);
    println!("{}", table);

    // get target format from CLI args or prompt
    let target_format = if let Some(args) = args {
        if let Some(arg) = args.get(0) {
            match arg.to_lowercase().as_str() {
                "png" => "PNG",
                "jpg" | "jpeg" => "JPEG",
                "webp" => "WEBP",
                _ => {
                    println!("Invalid format {}. Falling back to WEBP.", arg);
                    "WEBP"
                }
            }
        } else {
            "WEBP"
        }
    } else {
        // show options
        println!();
        println!("{}", style("CONVERT TO").fg(Theme::HEADER).bold());

        let options = &[
            "PNG  (Lossless)",
            "JPG  (Smaller size)",
            "WEBP (Modern)",
        ];

        let selection = Select::new()
            .items(options)
            .default(0)
            .interact()
            .unwrap_or(0);

        match selection {
            0 => "PNG",
            1 => "JPEG",
            _ => "WEBP",
        }
    };

    // run conversion
    convert_image(path, target_format, img);
}

// --- does the actual conversion and save ---

fn convert_image(path: &str, target_format: &str, img: DynamicImage) {
    let original_dir = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();

    println!("{}", style(format!("Converting to {}...", target_format)).fg(Theme::INFO).bold());

    // convert to rgb for jpg/webp (can't handle transparency)
    let img = if needs_rgb_conversion(&img, target_format) {
        DynamicImage::ImageRgb8(img.to_rgb8())
    } else {
        img
    };

    // build output path
    let out_ext = match target_format {
        "PNG" => "png",
        "JPEG" => "jpg",
        _ => "webp",
    };
    let output_path = original_dir.join(format!("{}_converted.{}", name, out_ext));

    // save with the right format
    let format = match target_format {
        "PNG" => image::ImageFormat::Png,
        "JPEG" => image::ImageFormat::Jpeg,
        _ => image::ImageFormat::WebP,
    };

    match img.save_with_format(&output_path, format) {
        Ok(_) => {
            println!(
                "{} Saved at: {}",
                style("✓ Done!").fg(Theme::SUCCESS).bold(),
                style(output_path.display()).underlined(),
            );
        }
        Err(e) => {
            println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
        }
    }
}

// --- helper: get file extension lowercase ---

fn get_extension(path: &str) -> String {
    Path::new(path)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase()
}

// --- helper: check if image needs rgb conversion ---

fn needs_rgb_conversion(img: &DynamicImage, target_format: &str) -> bool {
    if target_format == "JPEG" || target_format == "WEBP" {
        matches!(img, DynamicImage::ImageRgba8(_) | DynamicImage::ImageLuma8(_) | DynamicImage::ImageLumaA8(_))
    } else {
        false
    }
}

// --- helper: describe the image color mode ---

fn color_mode_str(img: &DynamicImage) -> &'static str {
    match img {
        DynamicImage::ImageRgb8(_) => "RGB",
        DynamicImage::ImageRgba8(_) => "RGBA",
        DynamicImage::ImageLuma8(_) => "L",
        DynamicImage::ImageLumaA8(_) => "LA",
        DynamicImage::ImageRgb16(_) => "RGB16",
        DynamicImage::ImageRgba16(_) => "RGBA16",
        DynamicImage::ImageRgb32F(_) => "RGB32F",
        DynamicImage::ImageRgba32F(_) => "RGBA32F",
        DynamicImage::ImageLuma16(_) => "L16",
        DynamicImage::ImageLumaA16(_) => "LA16",
        _ => "Unknown",
    }
}
