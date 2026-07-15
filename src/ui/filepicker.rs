/// filepicker.rs - gets and validates file path from user

use std::path::Path;
use console::style;
use dialoguer::Input;
use crate::ui::theme::Theme;

// --- ask the user for a file path, validate it ---

pub fn pick_file() -> String {
    loop {
        let path: String = Input::new()
            .with_prompt(format!("{}", style("ENTER PATH TO FILE (or press ENTER to browse)").fg(Theme::INFO).bold()))
            .allow_empty(true)
            .interact_text()
            .unwrap_or_default();

        let path = path.trim().trim_matches('\'').trim_matches('"').to_string();

        let path = if path.is_empty() {
            if let Some(file) = rfd::FileDialog::new().pick_file() {
                file.to_string_lossy().into_owned()
            } else {
                continue;
            }
        } else {
            path
        };

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
