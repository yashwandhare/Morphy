/// markdown.rs - converts markdown files to styled pdfs
///
/// uses pulldown-cmark for markdown parsing and subprocess to weasyprint
/// for pdf rendering (same engine as the python version).

use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;

use console::style;

use crate::ui::theme::Theme;

// --- entry point: convert markdown to pdf ---

pub fn convert_markdown_to_pdf(path: &str, _args: Option<&[String]>) {
    let md_path = Path::new(path);

    // validate input
    let ext = md_path
        .extension()
        .unwrap_or(OsStr::new(""))
        .to_string_lossy()
        .to_lowercase();

    if ext != "md" && ext != "markdown" {
        println!("{}", style("Invalid file. Select a Markdown file (.md)").fg(Theme::ERROR));
        return;
    }

    if !md_path.is_file() {
        println!(
            "{}",
            style(format!("Markdown file not found at: {}", path)).fg(Theme::ERROR),
        );
        return;
    }

    println!("{}", style(format!("--- Starting conversion for: {} ---", path)).fg(Theme::INFO));

    // read markdown content
    let md_text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            println!("{}", style(format!("Failed to read file: {}", e)).fg(Theme::ERROR));
            return;
        }
    };

    // convert markdown to html using pulldown-cmark
    let parser = pulldown_cmark::Parser::new(&md_text);
    let mut html_body = String::new();
    pulldown_cmark::html::push_html(&mut html_body, parser);

    println!("{}", style("Successfully converted Markdown to HTML.").fg(Theme::SUCCESS));

    // wrap in document template
    let html_content = format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <style>
    body {{
      font-family: sans-serif;
      line-height: 1.6;
      margin: 2rem;
    }}
    img {{
      max-width: 100%;
    }}
  </style>
</head>
<body>
{}
</body>
</html>"#,
        html_body
    );

    // build output path
    let output_path = md_path.with_extension("pdf");

    // try weasyprint first (subprocess)
    if try_weasyprint(&html_content, &output_path, md_path) {
        return;
    }

    // fallback: save html and tell user
    let html_path = md_path.with_extension("html");
    match std::fs::write(&html_path, &html_content) {
        Ok(_) => {
            println!(
                "{}",
                style(format!("HTML saved at: {}", html_path.display())).fg(Theme::SUCCESS),
            );
            println!(
                "{}",
                style("Install weasyprint for direct PDF output: pip install weasyprint").fg(Theme::DIM),
            );
        }
        Err(e) => {
            println!("{}", style(format!("Failed to save HTML: {}", e)).fg(Theme::ERROR));
        }
    }
}

// --- try rendering pdf with weasyprint subprocess ---

fn try_weasyprint(html_content: &str, output_path: &Path, md_path: &Path) -> bool {
    // write temp html file
    let temp_html = md_path.with_extension("_temp.html");
    if std::fs::write(&temp_html, html_content).is_err() {
        return false;
    }

    // try weasyprint command
    let result = Command::new("weasyprint")
        .arg(temp_html.to_str().unwrap_or("input.html"))
        .arg(output_path.to_str().unwrap_or("output.pdf"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    // cleanup temp file
    let _ = std::fs::remove_file(&temp_html);

    match result {
        Ok(status) if status.success() => {
            println!(
                "{}",
                style(format!("Successfully generated PDF at: {}", output_path.display())).fg(Theme::SUCCESS),
            );
            true
        }
        _ => {
            // weasyprint not available
            false
        }
    }
}
