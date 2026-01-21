# Morphy – Workflow

Morphy follows a **guided, step-by-step CLI workflow** prioritizing clarity and predictability.

## Global Workflow

```mermaid
graph TB
    Start([Start Morphy]) --> Splash[Splash Screen]
    Splash --> Menu[Main Menu]
    Menu --> Choice{Select Operation}
    
    Choice -->|Image| ImgFlow[Image Conversion]
    Choice -->|PDF| PdfFlow[PDF Tools]
    Choice -->|Video| VidFlow[Video to GIF]
    Choice -->|Exit| End([Exit])
    
    ImgFlow --> File[File Selection]
    PdfFlow --> File
    VidFlow --> File
    
    File --> Validate{Valid?}
    Validate -->|No| Error[Show Error]
    Error --> File
    Validate -->|Yes| Process[Execute]
    
    Process --> Output[Generate Output]
    Output --> Success[Show Success]
    Success --> Menu
    
    style Start fill:#4A90E2,stroke:#2E5C8A,color:#fff
    style Menu fill:#9B59B6,stroke:#6C3A80,color:#fff
    style Choice fill:#50C878,stroke:#2E7D4E,color:#fff
    style Process fill:#F39C12,stroke:#C87F0A,color:#fff
    style Success fill:#27AE60,stroke:#1E8449,color:#fff
    style Error fill:#E74C3C,stroke:#A93226,color:#fff
```

## Operations

### Image Conversion
**Formats:** PNG, JPG, WEBP

1. Select image file
2. View metadata (format, dimensions)
3. Choose target format
4. Convert and save with `_converted` suffix

### PDF Tools

**Image → PDF**
- Select image file
- Choose page mode (A4 or image-sized)
- Generate PDF

**PDF → Image**
- Select PDF file
- Render each page as PNG
- Save all pages

### Video to GIF
**Format:** MP4 → GIF

1. Check FFmpeg availability
2. Select video file
3. Configure FPS and width
4. Generate GIF with palette optimization
5. Cleanup temp files

## Key Features

- **File Validation:** All paths verified before processing
- **Safe Output:** Files saved with suffixes, never overwrite
- **Clear Errors:** Immediate feedback on failures
- **Loop to Menu:** Return after each operation

## Design Principles

- Interactive by default
- No destructive actions without confirmation
- Beginner-friendly with clear prompts
- Predictable output locations
