/// pdf.rs - handles pdf and image conversions
///
/// requires the "pdf" feature (mupdf crate + clang-devel installed).
/// if the feature is disabled, shows a helpful message instead.

use std::path::Path;
use std::ffi::OsStr;

use console::style;
use dialoguer::Select;

use crate::ui::theme::Theme;

// --- entry point: detect direction (img→pdf or pdf→img) ---

pub fn conv_doc(path: &str) {
    // check if pdf feature is available
    if !cfg!(feature = "pdf") {
        println!(
            "{}",
            style("PDF tools require the 'pdf' feature. Build with: cargo build --features pdf").fg(Theme::ERROR),
        );
        println!(
            "{}",
            style("You also need clang-devel installed: sudo dnf install -y clang-devel").fg(Theme::DIM),
        );
        return;
    }

    #[cfg(feature = "pdf")]
    conv_doc_inner(path);
}

#[cfg(feature = "pdf")]
fn conv_doc_inner(path: &str) {
    let ext = Path::new(path)
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase();

    // show menu options
    println!();
    println!("{}", style("PDF TOOLS").fg(Theme::HEADER).bold());

    let options = &["Image -> PDF", "PDF -> Images"];
    let selection = Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(0);

    // route logic based on file type
    if selection == 0 {
        let image_exts = ["png", "jpg", "jpeg", "webp", "bmp"];
        if !image_exts.contains(&ext.as_str()) {
            println!("{}", style("Invalid file. Select an image.").fg(Theme::ERROR));
            return;
        }
        img_to_pdf(path);
    } else {
        if ext != "pdf" {
            println!("{}", style("Invalid file. Select a PDF.").fg(Theme::ERROR));
            return;
        }
        pdf_to_img(path);
    }
}

// --- convert an image file to a single-page pdf ---

#[cfg(feature = "pdf")]
fn img_to_pdf(path: &str) {
    // display file stats
    let file_size = std::fs::metadata(path)
        .map(|m| m.len() as f64 / 1024.0)
        .unwrap_or(0.0);
    let filename = Path::new(path)
        .file_name()
        .unwrap_or(OsStr::new("unknown"))
        .to_string_lossy();

    let mut table = comfy_table::Table::new();
    table.load_preset(comfy_table::presets::NOTHING);
    table.add_row(vec!["File", &filename.to_string()]);
    table.add_row(vec!["Size", &format!("{:.2} KB", file_size)]);
    println!("{}", table);

    // get layout choice
    println!();
    println!("{}", style("PAGE SIZING").fg(Theme::HEADER).bold());
    let size_options = &["A4 Centered", "Original Fit"];
    let size_opt = Select::new()
        .items(size_options)
        .default(0)
        .interact()
        .unwrap_or(0);

    println!("{}", style("Building PDF...").fg(Theme::INFO).bold());

    // build output path
    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();
    let output_pdf = parent.join(format!("{}.pdf", name));

    // create pdf using mupdf
    let result = build_pdf_from_image(path, output_pdf.to_str().unwrap_or("output.pdf"), size_opt == 0);

    match result {
        Ok(_) => {
            println!(
                "{} {}",
                style("✓ Saved:").fg(Theme::SUCCESS).bold(),
                output_pdf.display(),
            );
        }
        Err(e) => {
            println!("{} {}", style("Failed:").fg(Theme::ERROR), e);
        }
    }
}

// --- build a pdf from an image using mupdf ---

#[cfg(feature = "pdf")]
fn build_pdf_from_image(image_path: &str, output_path: &str, centered_a4: bool) -> Result<(), String> {
    // open the image to get dimensions
    let img_doc = mupdf::Document::open(image_path)
        .map_err(|e| format!("Can't open image: {}", e))?;

    let img_page = img_doc.load_page(0)
        .map_err(|e| format!("Can't read image: {}", e))?;
    let img_rect = img_page.bounds()
        .map_err(|e| format!("Can't get bounds: {}", e))?;

    // create new pdf document
    let mut doc = mupdf::pdf::PdfDocument::new();

    // a4 dimensions in points
    let (page_w, page_h) = if centered_a4 {
        (595.0_f32, 842.0_f32)
    } else {
        (img_rect.x1 - img_rect.x0, img_rect.y1 - img_rect.y0)
    };

    let mut pdf_page = doc.new_page((page_w, page_h))
        .map_err(|e| format!("Can't create page: {}", e))?;

    // read image data
    let img_data = std::fs::read(image_path)
        .map_err(|e| format!("Can't read file: {}", e))?;

    // calculate position
    let img_w = img_rect.x1 - img_rect.x0;
    let img_h = img_rect.y1 - img_rect.y0;

    let (x, y, w, h) = if centered_a4 {
        let x = (page_w - img_w) / 2.0;
        let y = (page_h - img_h) / 2.0;
        (x, y, img_w, img_h)
    } else {
        (0.0, 0.0, page_w, page_h)
    };

    pdf_page.insert_image(
        &mut doc,
        mupdf::Rect::new(x, y, x + w, y + h),
        mupdf::pdf::PageImageSource::Bytes { data: &img_data, format_hint: None },
        mupdf::pdf::InsertImageOptions::default(),
    ).map_err(|e| format!("Can't insert image: {}", e))?;

    // save
    doc.save(output_path)
        .map_err(|e| format!("Can't save PDF: {}", e))?;

    Ok(())
}

// --- convert each pdf page to a png image ---

#[cfg(feature = "pdf")]
fn pdf_to_img(path: &str) {
    let parent = Path::new(path).parent().unwrap_or(Path::new("."));
    let name = Path::new(path)
        .file_stem()
        .unwrap_or(OsStr::new("output"))
        .to_string_lossy();

    // open pdf
    let doc = match mupdf::Document::open(path) {
        Ok(doc) => doc,
        Err(e) => {
            println!("{}", style(format!("Failed to open PDF: {}", e)).fg(Theme::ERROR));
            return;
        }
    };

    let page_count = doc.page_count().unwrap_or(0);

    // set up progress bar
    let progress = indicatif::ProgressBar::new(page_count as u64);
    progress.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("{msg} [{bar:30}] {pos}/{len}")
            .unwrap_or_else(|_| indicatif::ProgressStyle::default_bar())
    );
    progress.set_message("Extracting pages...");

    let mut saved = 0;

    // loop pages with progress bar
    for i in 0..page_count {
        let page = match doc.load_page(i) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", style(format!("Failed to load page {}: {}", i + 1, e)).fg(Theme::ERROR));
                continue;
            }
        };

        // render at 3x scale (same as python: fitz.Matrix(3, 3))
        let matrix = mupdf::Matrix::new_scale(3.0, 3.0);
        let pixmap = match page.to_pixmap(&matrix, &mupdf::Colorspace::device_rgb(), false, true) {
            Ok(p) => p,
            Err(e) => {
                println!("{}", style(format!("Failed to render page {}: {}", i + 1, e)).fg(Theme::ERROR));
                continue;
            }
        };

        let img_path = parent.join(format!("{}_page_{}.png", name, i + 1));

        match pixmap.save_as(img_path.to_str().unwrap_or("output.png"), mupdf::ImageFormat::PNG) {
            Ok(_) => saved += 1,
            Err(e) => {
                println!("{}", style(format!("Failed to save page {}: {}", i + 1, e)).fg(Theme::ERROR));
            }
        }

        progress.inc(1);
    }

    progress.finish_and_clear();

    println!(
        "{}",
        style(format!("✓ Extracted {} pages.", saved)).fg(Theme::SUCCESS).bold(),
    );
}
