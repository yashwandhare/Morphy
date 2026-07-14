# Morphy – Workflow

Morphy follows a **guided, step-by-step CLI workflow** prioritizing clarity and predictability.

## Global Workflow

```
start
  │
  ├── show splash screen
  │
  └── main menu loop
        │
        ├── [1] image conversion
        │     ├── pick file
        │     ├── show image info (format, size, dimensions)
        │     ├── choose target format (png/jpg/webp)
        │     └── convert and save
        │
        ├── [2] video → gif
        │     ├── pick file
        │     ├── check ffmpeg installed
        │     ├── choose frame rate (10/24/30)
        │     ├── choose width (320/480/720/1080)
        │     └── two-pass ffmpeg conversion
        │
        ├── [3] pdf tools
        │     ├── pick file
        │     ├── choose direction:
        │     │     ├── image → pdf (a4 centered or original fit)
        │     │     └── pdf → images (3x scale, per-page png)
        │     └── save output
        │
        ├── [4] compression
        │     ├── sub-menu: image or pdf
        │     │
        │     ├── image compression
        │     │     ├── pick file
        │     │     ├── show image info
        │     │     ├── choose preset or custom target
        │     │     ├── compress (binary search for custom)
        │     │     └── show before/after comparison
        │     │
        │     └── pdf compression
        │           ├── pick file
        │           ├── show pdf info
        │           ├── choose preset or custom target
        │           ├── compress with mupdf
        │           └── show before/after comparison
        │
        ├── [5] markdown → pdf
        │     ├── pick file
        │     └── render styled pdf (via weasyprint)
        │
        └── [6] exit
```

## File Naming

all output files follow this pattern:
- conversion: `{name}_converted.{ext}`
- compression: `{name}_compressed.{ext}`
- pdf pages: `{name}_page_{n}.png`
- markdown: `{name}.pdf`

## Error Handling

- invalid file path → re-prompt
- unsupported format → show error, return to menu
- conversion failure → show error message
- ffmpeg missing → show install prompt

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
