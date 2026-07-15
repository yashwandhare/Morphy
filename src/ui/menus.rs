/// menus.rs - shows main menu and gets user choice

use console::style;
use dialoguer::Select;
use crate::ui::theme::Theme;
use crate::core::registry::Registry;

// --- main menu ---

pub fn show_menu(registry: &Registry) -> Option<String> {
    println!();
    println!("{}", style("SELECT OPERATION").fg(Theme::INFO).bold());

    let mut options: Vec<String> = registry
        .operations
        .iter()
        .map(|op| op.display_name.to_string())
        .collect();
    
    options.push("Exit".to_string());

    let selection = Select::new()
        .items(&options)
        .default(0)
        .interact()
        .unwrap_or(options.len() - 1); // default to exit on error

    if selection == options.len() - 1 {
        return None;
    }

    Some(registry.operations[selection].id.to_string())
}
