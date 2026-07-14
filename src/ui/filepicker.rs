/// filepicker.rs - gets and validates file path from user

use std::path::Path;
use console::style;
use dialoguer::Input;
use crate::ui::theme::Theme;

// --- ask the user for a file path, validate it ---

pub fn pick_file() -> String {
    loop {
        let path: String = Input::new()
            .with_prompt(format!("{}", style("ENTER PATH TO FILE").fg(Theme::INFO).bold()))
            .interact_text()
            .unwrap_or_default();

        let path = path.trim().to_string();

        if Path::new(&path).is_file() {
            // success message
            println!(
                "{} {}",
                style("✓ File found:").fg(Theme::SUCCESS),
                style(&path).fg(Theme::DIM),
            );
            println!();
            return path;
        }

        // error message
        println!(
            "{} {}",
            style("✗ File not found:").fg(Theme::ERROR),
            path,
        );
    }
}
