/// splash.rs - displays the startup ascii splash screen

use console::style;
use crate::ui::theme::Theme;

/// show the styled ascii splash screen with correct alignment
pub fn show_splash() {
    // geometric ascii art
    let logo = [
        r"███╗   ███╗  ██████╗  ██████╗  ██████╗  ██╗  ██╗ ██╗   ██╗",
        r"████╗ ████║ ██╔═══██╗ ██╔══██╗ ██╔══██╗ ██║  ██║ ╚██╗ ██╔╝",
        r"██╔████╔██║ ██║   ██║ ██████╔╝ ██████╔╝ ███████║  ╚████╔╝ ",
        r"██║╚██╔╝██║ ██║   ██║ ██╔══██╗ ██╔═══╝  ██╔══██║   ╚██╔╝  ",
        r"██║ ╚═╝ ██║ ╚██████╔╝ ██║  ██║ ██║      ██║  ██║    ██║   ",
        r"╚═╝     ╚═╝  ╚═════╝  ╚═╝  ╚═╝ ╚═╝      ╚═╝  ╚═╝    ╚═╝   ",
    ];

    // --- dimensions ---
    let content_width = logo[0].chars().count();
    let padding = 4;
    let inner_width = content_width + (padding * 2);

    // --- text content ---
    let left_text = "simple file converter";
    let right_text = "made by kazuto";
    let spacer_len = inner_width.saturating_sub(8 + left_text.len() + right_text.len());

    // --- render ---

    // top border
    println!(
        "{}",
        style(format!("╔{}╗", "═".repeat(inner_width))).fg(Theme::BORDER)
    );

    // top spacer
    println!(
        "{}",
        style(format!("║{}║", " ".repeat(inner_width))).fg(Theme::BORDER)
    );

    // logo section
    for line in &logo {
        let left_border = style("║").fg(Theme::BORDER);
        let right_border = style("║").fg(Theme::BORDER);
        let logo_text = style(*line).fg(Theme::HEADER);
        let pad = " ".repeat(padding);

        println!("{left_border}{pad}{logo_text}{pad}{right_border}");
    }

    // middle spacer
    println!(
        "{}",
        style(format!("║{}║", " ".repeat(inner_width))).fg(Theme::BORDER)
    );

    // subtext section
    let left_b = style("║").fg(Theme::BORDER);
    let right_b = style("║").fg(Theme::BORDER);
    let sub = style(format!(
        "{left_text}{}{right_text}",
        " ".repeat(spacer_len)
    ))
    .fg(Theme::TEXT);

    println!("{left_b}    {sub}    {right_b}");

    // bottom spacer
    println!(
        "{}",
        style(format!("║{}║", " ".repeat(inner_width))).fg(Theme::BORDER)
    );

    // bottom border
    println!(
        "{}",
        style(format!("╚{}╝", "═".repeat(inner_width))).fg(Theme::BORDER)
    );
}
