# Morphy User Manual

Welcome to Morphy! Morphy is a terminal-based file conversion tool that walks you through every step of file conversion and compression with an easy-to-use, guided interface.

No need to memorize complex CLI arguments—just run the app and follow the prompts!

## Table of Contents

- [Installation & Setup](#installation--setup)
- [Running Morphy](#running-morphy)
- [Core Features](#core-features)
  - [Image Conversion](#image-conversion)
  - [Video to GIF](#video-to-gif)
  - [PDF Tools](#pdf-tools)
  - [Compression](#compression)
  - [Markdown to PDF](#markdown-to-pdf)
- [Troubleshooting](#troubleshooting)

---

## Installation & Setup

### Prerequisites

Morphy requires a few system dependencies for all features to work perfectly:

1. **Rust Toolchain:** To build the app.
2. **FFmpeg:** Needed for Video-to-GIF conversion.
   ```bash
   # on Ubuntu/Debian
   sudo apt install ffmpeg
   # on Fedora
   sudo dnf install ffmpeg
   ```
3. **Clang Devel:** Needed for PDF processing (`mupdf` crate).
   ```bash
   # on Fedora
   sudo dnf install clang-devel
   # on Ubuntu/Debian
   sudo apt install libclang-dev
   ```
4. **WeasyPrint:** (Optional) For Markdown to PDF conversion. 
   ```bash
   pip install weasyprint
   ```

### Building the Project

If you have `clang-devel` installed, you can build with full PDF support:
```bash
cargo build --release
```

If you do NOT want to install `clang-devel`, you can still build and run the app by disabling PDF support:
```bash
cargo build --release --no-default-features
```

---

## Running Morphy

Once built, simply execute:
```bash
cargo run --release
```
You will be greeted with an interactive main menu. Use your arrow keys to navigate and `Enter` to select an option.

---

## Core Features

### Image Conversion
Convert your images between popular formats easily.
1. Select **Image Conversion** from the main menu.
2. Provide the absolute or relative path to your image (e.g., `image.png`).
3. You will see file stats (Size, Dimensions).
4. Choose your target format: **PNG**, **JPG**, or **WEBP**.
5. The output will be saved as `[filename]_converted.[format]`.

### Video to GIF
Turn short video clips into GIFs.
1. Select **Video to GIF**.
2. Provide the path to a supported video (e.g., MP4, MKV, AVI, MOV).
3. Select an **FPS** (10, 24, or 30). Lower FPS results in a smaller file.
4. Select a **Width** (320px, 480px, 720px, 1080px). The aspect ratio will be preserved.
5. The process might take a moment as FFmpeg handles it under the hood. Output is saved as `[filename].gif`.

### PDF Tools
*(Requires full installation with `clang-devel`)*

Convert a single image into a PDF, or extract every page of a PDF into images.
1. Select **PDF Tools**.
2. Provide an Image or a PDF.
3. If an Image, you'll be prompted to scale it (A4 Centered vs. Original Fit). The image becomes a single-page PDF.
4. If a PDF, Morphy extracts each page as a high-quality PNG image (`[filename]_page_1.png`, etc.).

### Compression
Optimize and shrink the size of Images or PDFs.

1. Select **Compression**, then choose Image or PDF.
2. Provide the path to the file.
3. Choose a preset or "Custom":
   - **Image Presets:** Light (~30% reduction), Medium (~50%), Aggressive (~70%). 
   - **PDF Presets:** Standard or Maximum.
   - **Custom:** Type in a target file size in KB.
4. If "Custom" is selected for Images, Morphy uses a binary search to find the perfect quality and dimension combination to meet your target.
5. You'll see a Before/After comparison when the compression finishes.

### Markdown to PDF
Render styled PDFs from Markdown files.
1. Select **Markdown to PDF**.
2. Provide the path to a `.md` file.
3. If `weasyprint` is installed, it outputs a ready-to-read PDF. Otherwise, it generates an HTML fallback file.

---

## Troubleshooting

- **"File not found" when providing a path:** Double-check your path. Relative paths (like `./my_pic.jpg`) or absolute paths (`/home/user/my_pic.jpg`) work best. Ensure you avoid trailing spaces or unmatched quotes.
- **"FFmpeg is not installed" error:** You must install FFmpeg on your system to use the Video-to-GIF feature.
- **"PDF tools require the 'pdf' feature" error:** Re-compile the app with `cargo build --release` after installing `clang-devel`.
- **"HTML saved at... Install weasyprint..." message:** Run `pip install weasyprint` so Morphy can generate actual PDF files from your Markdown.
