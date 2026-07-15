/// pdf.rs - pdf compression with preset and custom size options
///
/// requires the "pdf" feature (mupdf crate + clang-devel installed).

use std::path::Path;
use std::ffi::OsStr;

use console::style;
use dialoguer::Select;

use crate::ui::theme::Theme;

// compression presets
struct Preset {
    name: &'static str,
    garbage: i32,
    reduction: &'static str,
}

const PRESETS: [Preset; 2] = [
    Preset { name: "Standard", garbage: 3, reduction: "~25%" },
    Preset { name: "Maximum",  garbage: 4, reduction: "~40%" },
];

// --- entry point ---

pub fn compress_pdf(path: &str, args: Option<&[String]>) {
    if !validate_pdf(path) {
        return;
    }

    let doc = match load_pdf(path) {
        Some(doc) => doc,
        None => return,
    };

    let original_size = std::fs::metadata(path)
        .map(|m| m.len())
        .unwrap_or(0);

    show_info(path, &doc, original_size);

    let choice = get_compression_choice(args);

    let output_path = match choice {
        2 => {
            // custom target size
            let target_kb = match get_target_size(args) {
                Some(kb) => kb,
                None => return,
            };
            compress_to_target(path, &doc, target_kb, original_size)
        }
        _ => {
            // preset compression
            compress_with_preset(path, &doc, &PRESETS[choice])
        }
    };

    if let Some(output) = output_path {
        show_comparison(original_size, &output);
    }
}

// --- validation ---

fn validate_pdf(path: &str) -> bool {
    let ext = Path::new(path)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase();

    if ext != "pdf" {
        println!(
            "{}",
            style("Invalid file type. Please select a PDF.").fg(Theme::ERROR),
        );
        return false;
    }
    true
}

// --- load pdf ---

fn load_pdf(path: &str) -> Option<mupdf::pdf::PdfDocument> {
    match mupdf::pdf::PdfDocument::open(path) {
        Ok(doc) => Some(doc),
        Err(e) => {
            println!(
                "{}",
                style(format!("Failed to open PDF: {}", e)).fg(Theme::ERROR),
            );
            None
        }
    }
}

// --- display file info ---

fn show_info(path: &str, doc: &mupdf::pdf::PdfDocument, size_bytes: u64) {
    let size_kb = size_bytes as f64 / 1024.0;
    let size_mb = size_kb / 1024.0;

    let size_str = if size_mb >= 1.0 {
        format!("{:.2} MB", size_mb)
    } else {
        format!("{:.2} KB", size_kb)
    };

    let page_count = doc.page_count().unwrap_or(0);

    let filename = Path::new(path)
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_string_lossy();

    println!();
    println!("{}", style("PDF COMPRESSION").fg(Theme::HEADER).bold());

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.add_row(vec!["File", &filename.to_string()]);
    table.add_row(vec!["Size", &size_str]);
    table.add_row(vec!["Pages", &page_count.to_string()]);
    println!("{}", table);
}

// --- ask how to compress ---

fn get_compression_choice(args: Option<&[String]>) -> usize {
    if let Some(args) = args {
        if let Some(arg) = args.get(0) {
            return match arg.to_lowercase().as_str() {
                "standard" => 0,
                "maximum" => 1,
                _ => 2, // custom size
            };
        }
    }

    println!();
    println!("{}", style("Select compression level:").fg(Theme::HEADER).bold());

    let options = &[
        format!("Standard    garbage: {}  |  deflate: Yes  |  {}", PRESETS[0].garbage, PRESETS[0].reduction),
        format!("Maximum     garbage: {}  |  deflate: Yes  |  {}", PRESETS[1].garbage, PRESETS[1].reduction),
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

fn compress_with_preset(path: &str, doc: &mupdf::pdf::PdfDocument, preset: &Preset) -> Option<String> {
    let output_path = get_output_path(path);

    println!(
        "{}",
        style(format!("Compressing with {} settings...", preset.name)).fg(Theme::INFO),
    );

    let mut save_opts = mupdf::pdf::PdfWriteOptions::default();
    save_opts.set_garbage_level(preset.garbage)
             .set_compress(true)
             .set_clean(true);

    match doc.save_with_options(&output_path, save_opts) {
        Ok(_) => Some(output_path),
        Err(e) => {
            println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
            None
        }
    }
}

// --- compress to a target file size ---

fn compress_to_target(path: &str, doc: &mupdf::pdf::PdfDocument, target_kb: u64, original_size: u64) -> Option<String> {
    let target_bytes = target_kb * 1024;
    let output_path = get_output_path(path);

    println!(
        "{}",
        style("Compressing to target size...").fg(Theme::INFO),
    );

    // try max compression
    let mut save_opts = mupdf::pdf::PdfWriteOptions::default();
    save_opts.set_garbage_level(4)
             .set_compress(true)
             .set_clean(true);

    if let Err(e) = doc.save_with_options(&output_path, save_opts) {
        println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
        return None;
    }

    let compressed_size = std::fs::metadata(&output_path)
        .map(|m| m.len())
        .unwrap_or(0);

    if compressed_size > target_bytes {
        // target not achievable — ask user what to do
        let _ = std::fs::remove_file(&output_path);

        let best_kb = compressed_size as f64 / 1024.0;
        let best_str = if best_kb >= 1024.0 {
            format!("{:.2} MB", best_kb / 1024.0)
        } else {
            format!("{:.0} KB", best_kb)
        };

        println!();
        println!(
            "{}",
            style(format!("⚠ Target {} KB not achievable (structural compression only)", target_kb)).fg(Theme::ERROR),
        );
        println!(
            "{}",
            style(format!("Best possible: {} (with Maximum settings)", best_str)).fg(Theme::INFO),
        );

        let proceed_options = &["Proceed with best possible", "Cancel"];
        let proceed = Select::new()
            .items(proceed_options)
            .default(1)
            .interact()
            .unwrap_or(1);

        if proceed == 0 {
            // re-save with max settings
            let mut save_opts = mupdf::pdf::PdfWriteOptions::default();
            save_opts.set_garbage_level(4)
                     .set_compress(true)
                     .set_clean(true);
            match doc.save_with_options(&output_path, save_opts) {
                Ok(_) => return Some(output_path),
                Err(e) => {
                    println!("{}", style(format!("Failed to save: {}", e)).fg(Theme::ERROR));
                    return None;
                }
            }
        } else {
            println!("{}", style("Cancelled.").fg(Theme::DIM));
            return None;
        }
    }

    Some(output_path)
}

// --- build the output filename ---

fn get_output_path(path: &str) -> String {
    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();

    parent.join(format!("{}_compressed.pdf", name))
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
