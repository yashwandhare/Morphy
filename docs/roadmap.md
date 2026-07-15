# Post v1.0 Feature Roadmap

This document outlines the planned features and architectural improvements for Morphy Post v1.0, making it the default local-first file utility for Linux and macOS.

## Architecture Improvements

- [ ] **File Provider System**: Replace manual path entry with a flexible file acquisition layer (`rfd` for native file picker, CLI path, Drag & Drop, Directory picker, Watch directory).
- [ ] **Complete UI Separation**: UI gathers user input, processing modules execute work (e.g., `compress_image(path, CompressionOptions)`).
- [ ] **Registry-Based Operations**: Replace hardcoded dispatcher `match` statements with a dynamic operation registry (supporting dynamic menus and automatic capability discovery).
- [ ] **Smart File Detection**: Detect file type and show supported operations instead of asking for the operation first.
- [ ] **Natural Language CLI**: Support readable commands like `morphy make image.png webp`.
- [ ] **Directory Mode**: Allow operations on folders (e.g., `morphy make images/ webp`).
- [ ] **Watch Mode**: Watch a directory and process newly added files (e.g., `morphy watch Downloads webp`).
- [ ] **Smart File Suggestions**: Suggest nearby matches if a requested file cannot be found (e.g., "Did you mean resume.docx?").
- [ ] **Rich Progress Feedback**: Display progress bar, current step, output location, and before/after statistics for every operation.

## Supported Operations

### Images
- [x] Format Conversion
- [x] Compression
- [ ] Background Removal
- [ ] Upscaling
- [ ] Metadata Viewer
- [ ] Metadata Removal

### PDF
- [x] PDF → Images
- [x] Image → PDF
- [ ] OCR
- [x] Compression
- [ ] Metadata Viewer
- [ ] Metadata Removal

### Video
- [x] Video → GIF
- [ ] GIF → Video

### Audio
- [ ] MP3, WAV, FLAC, OGG, AAC, Opus conversions

### Documents
- [x] Markdown → PDF
- [ ] HTML → PDF
- [ ] TXT → PDF

### Archives
- [ ] ZIP
- [ ] Unzip

## Developer Utilities
- [ ] **Morphy Doctor**: Check system dependencies (FFmpeg, MuPDF, OCR backend, etc.) and provide actionable fixes.
- [ ] **Morphy Info**: Display detailed file information (format, resolution, codec, dimensions, size, metadata).
