/// dispatcher.rs - routes menu choices to the right handler

use crate::ui::menus::{MenuChoice, CompressionChoice};
use crate::converters;
use crate::compression;

// --- routes the user's menu choice to the right handler ---

pub fn dispatch(choice: &MenuChoice, path: &str) {
    match choice {
        MenuChoice::Image => {
            converters::image::conv_image(path);
        }
        MenuChoice::Video => {
            converters::video::conv_video(path);
        }
        MenuChoice::Doc => {
            converters::pdf::conv_doc(path);
        }
        MenuChoice::MarkdownToPdf => {
            converters::markdown::convert_markdown_to_pdf(path);
        }
        // compression is handled separately via dispatch_compression
        _ => {
            println!("ERROR: invalid choice");
        }
    }
}

// --- routes compression sub-menu choice ---

pub fn dispatch_compression(choice: &CompressionChoice, path: &str) {
    match choice {
        CompressionChoice::ImageCompress => {
            compression::image::compress_image(path);
        }
        CompressionChoice::PdfCompress => {
            #[cfg(feature = "pdf")]
            {
                compression::pdf::compress_pdf(path);
            }
            #[cfg(not(feature = "pdf"))]
            {
                println!("PDF compression requires the 'pdf' feature. Build with: cargo build --features pdf");
                let _ = path; // suppress unused warning
            }
        }
        CompressionChoice::Back => {}
    }
}
