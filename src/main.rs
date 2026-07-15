/**
 * main.rs -
 * entry point of the app
 * starts the program
 * controls the flow of app
 * calls the ui functions in order when told
 */

mod ui;
mod core;
mod converters;
mod compression;

use std::env;

use ui::splash::show_splash;
use ui::menus::show_menu;
use ui::filepicker::pick_file;
use crate::core::registry::Registry;

fn main() {
    let args: Vec<String> = env::args().collect();
    let registry = Registry::new();

    // Natural Language CLI Path
    if args.len() > 1 {
        handle_cli_args(&registry, &args);
        return;
    }

    // Interactive Path
    show_splash();

    // main app loop
    loop {
        let choice_id = match show_menu(&registry) {
            Some(id) => id,
            None => {
                println!("Exiting Morphy.");
                break;
            }
        };

        let path = pick_file();
        
        // Dispatch
        if let Some(op) = registry.operations.iter().find(|o| o.id == choice_id) {
            (op.handler)(&path, None);
        } else {
            println!("ERROR: Operation handler not found.");
        }
    }

    println!("Application finished.");
}

// --- Natural Language CLI logic ---
fn handle_cli_args(registry: &Registry, args: &[String]) {
    let verb = args[1].to_lowercase();
    
    if args.len() < 3 {
        println!("Usage: morphy <action> <file> [options...]");
        println!("Example: morphy convert image.png webp");
        return;
    }

    let path = &args[2];
    let ext = std::path::Path::new(path).extension().unwrap_or_default().to_string_lossy().to_lowercase();
    let handler_args = if args.len() > 3 { Some(&args[3..]) } else { None };

    let op_id = match verb.as_str() {
        "make" | "convert" => {
            match ext.as_str() {
                "md" | "markdown" => "markdown_pdf",
                "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" => "video_gif",
                _ => "image_conv",
            }
        },
        "compress" | "shrink" => {
            match ext.as_str() {
                "pdf" => "pdf_compress",
                _ => "image_compress",
            }
        },
        "pdf" | "extract" | "document" => "pdf_tools",
        _ => {
            println!("Unknown action: {}", verb);
            return;
        }
    };

    if let Some(op) = registry.operations.iter().find(|o| o.id == op_id) {
        (op.handler)(path, handler_args);
    } else {
        println!("Feature not available for '{}'", verb);
    }
}
