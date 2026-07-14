/// menus.rs - shows main menu and gets user choice

use console::style;
use dialoguer::Select;
use crate::ui::theme::Theme;

/// the choices returned by the main menu
#[derive(Debug, Clone, PartialEq)]
pub enum MenuChoice {
    Image,
    Video,
    Doc,
    Compression,
    MarkdownToPdf,
    Exit,
}

/// the choices returned by the compression sub-menu
#[derive(Debug, Clone, PartialEq)]
pub enum CompressionChoice {
    ImageCompress,
    PdfCompress,
    Back,
}

// --- main menu ---

pub fn show_menu() -> MenuChoice {
    println!();
    println!("{}", style("SELECT OPERATION").fg(Theme::INFO).bold());

    let options = &[
        "Image Conversion",
        "Video to GIF",
        "PDF Tools",
        "Compression",
        "Markdown to PDF",
        "Exit",
    ];

    let selection = Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(5); // default to exit on error

    match selection {
        0 => MenuChoice::Image,
        1 => MenuChoice::Video,
        2 => MenuChoice::Doc,
        3 => MenuChoice::Compression,
        4 => MenuChoice::MarkdownToPdf,
        _ => MenuChoice::Exit,
    }
}

// --- compression sub-menu ---

pub fn show_compression_menu() -> CompressionChoice {
    println!();
    println!("{}", style("COMPRESSION").fg(Theme::INFO).bold());

    let options = &[
        "Image Compression",
        "PDF Compression",
        "Back",
    ];

    let selection = Select::new()
        .items(options)
        .default(0)
        .interact()
        .unwrap_or(2); // default to back on error

    match selection {
        0 => CompressionChoice::ImageCompress,
        1 => CompressionChoice::PdfCompress,
        _ => CompressionChoice::Back,
    }
}
