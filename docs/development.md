# Development Guide

This document outlines how to set up your development environment for Morphy and how to extend the application by adding new features or converters.

## Local Setup

### Prerequisites
Morphy requires Rust and Cargo to be installed.

Additionally, to compile the PDF processing features, you must install the `clang` compiler and development headers, as they are required to build the `mupdf-sys` C bindings.

- **Ubuntu/Debian**: `sudo apt install build-essential libclang-dev`
- **Fedora**: `sudo dnf install clang-devel`
- **macOS**: `xcode-select --install`

### Compiling
To build the complete application (including PDF features):
```bash
cargo build
```

To build the application faster (without the `mupdf` C compilation) and test non-PDF features:
```bash
cargo build --no-default-features
```

## Adding a New Converter

Morphy's architecture makes it easy to add new file converters. Follow these steps to implement a new feature:

### 1. Update the UI (`src/ui/menus.rs`)
Add your new feature to the `MenuChoice` enum and the corresponding text option in the main menu prompt.

```rust
pub enum MenuChoice {
    // ...
    NewConverter,
}

// In show_menu():
let options = &[
    // ...
    "New Feature",
];
```

### 2. Create the Module (`src/converters/new_feature.rs`)
Create a new file in the `converters/` directory.

```rust
use crate::ui::theme::Theme;
use console::style;

pub fn process_file(path: &str) {
    println!("{}", style("Running New Feature...").fg(Theme::INFO));
    // Implementation here...
}
```

Don't forget to export your module in `src/converters/mod.rs`:
```rust
pub mod new_feature;
```

### 3. Route the Request (`src/core/dispatcher.rs`)
Update the `dispatch()` function to route the new `MenuChoice` to your module.

```rust
pub fn dispatch(choice: &MenuChoice, path: &str) {
    match choice {
        // ...
        MenuChoice::NewConverter => {
            converters::new_feature::process_file(path);
        }
    }
}
```

## Adding a New Dependency

When adding a new dependency to `Cargo.toml`, consider the following:
1. **Is it a pure Rust crate?** Prefer pure Rust crates (like `image` or `pulldown-cmark`) over crates that bind to C libraries, to keep cross-compilation simple.
2. **Does it require C bindings?** If a crate requires C dependencies (like `mupdf`), make it an `optional = true` dependency and gate its usage behind a Cargo feature flag. This ensures Morphy can always compile in environments where the system dependency cannot be installed.

## Code Style
- Use `console::style` alongside `Theme` constants (`Theme::INFO`, `Theme::ERROR`, `Theme::SUCCESS`, etc.) for all terminal output to ensure visual consistency.
- Return early on errors using simple print statements and `return;` rather than panicking or bubbling complex Results back up to `main.rs`. Keep the user inside the application loop.
