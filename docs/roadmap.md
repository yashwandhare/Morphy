# High level project roadmap for Morphy 

---

## Global Status

- [x] Project scaffolding and structure
- [x] File picker and path validation
- [x] Image format conversion
- [x] PDF tools (image ↔ PDF)
- [x] Video → GIF conversion (FFmpeg)
- [x] UI theming and visual polish
- [ ] Image compression
- [ ] Non-interactive CLI flags
- [ ] pip packaging
- [ ] Versioned releases

---

## Phase 1 – Initial Project Pipeline

**Goal:** Establish a clean, extensible structure.

- App entry point (`app.py`)
- Clear flow: UI → dispatcher → converters
- Modular directory layout
- No production features, structure-first approach

**Status:** ✅ Complete

---

## Phase 2 – File Input Handling

**Goal:** Make user input safe and predictable.

- Interactive file picker
- Path and existence validation
- Early error handling
- Removal of hardcoded paths

**Status:** ✅ Complete

---

## Phase 3 – Core Conversions

**Goal:** Implement functional, reliable converters.

### 3.1 Image Conversion
- PNG / JPG / WEBP support
- Metadata display (format, size, mode)
- RGB handling where required
- UI wrapper separated from core logic

**Status:** ✅ Complete

---

### 3.2 PDF Tools
- Image → PDF
  - A4 page option (centered image)
  - Original image-sized page option
- PDF → Image (per-page PNG output)
- Higher DPI rendering
- Input validation and predictable output paths

**Status:** ✅ Complete

---

### 3.3 Video Conversion
- MP4 → GIF only (intentionally limited)
- FFmpeg integration via subprocess
- Palette-based pipeline (palettegen → paletteuse)
- User-selectable FPS and width with safe defaults
- Temporary file cleanup

**Status:** ✅ Complete

---

## Phase 4 – UI Polish

**Goal:** Improve usability and visual clarity without overengineering.

- ASCII splash screen
- Centralized theme configuration
- Consistent color semantics (success, error, info)
- Clear menus and explicit exit handling
- Readable, calm CLI output

**Status:** ✅ Complete

---

## Phase 5 – Optimization Features (Planned)

**Goal:** Add practical value while keeping scope controlled.

- Image compression
- Sensible defaults
- Avoid format or option explosion

**Status:** ⏳ Planned

---

## Phase 6 – CLI Evolution (Optional)

**Goal:** Support advanced and scripted usage.

- CLI flags for non-interactive mode
- Retain guided menu-based workflow as default
- No breaking changes to existing UX

**Status:** ⏳ Optional / Future

---

## Current State

Morphy is a **feature-complete, guided CLI tool** with a clean and explainable architecture.  
Active development focus has shifted from adding features to **packaging, distribution, and release readiness**.

Upcoming work will prioritize:
- pip packaging
- entry-point CLI command
- versioned releases

---
