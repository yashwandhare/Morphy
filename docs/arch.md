# Morphy Architecture

## Overview

Morphy follows a layered architecture where each layer has a single responsibility and avoids leaking logic into adjacent layers.

## Architecture Diagram

```mermaid
graph TB
    subgraph Entry[Entry Point]
        App[app.py]
    end
    
    subgraph UI[UI Layer]
        Splash[splash.py<br/>Splash Screen]
        Menu[menus.py<br/>Main Menu]
        FilePicker[filepicker.py<br/>File Selection]
    end
    
    subgraph Routing[Routing]
        Dispatcher[dispatcher.py<br/>Route Intent]
    end
    
    subgraph Converters[Converter Layer]
        ImgConv[image.py<br/>PNG/JPG/WEBP]
        PdfConv[pdf.py<br/>PDF ↔ Image]
        VidConv[video.py<br/>MP4 → GIF]
    end
    
    subgraph Support[Support Systems]
        Utils[utils/<br/>System Helpers]
        Theme[theme.py<br/>Colors]
    end
    
    subgraph External[External Dependencies]
        Pillow[(Pillow)]
        PyMuPDF[(PyMuPDF)]
        FFmpeg[(FFmpeg)]
    end
    
    App --> Splash
    Splash --> Menu
    Menu --> Dispatcher
    
    Dispatcher --> ImgConv
    Dispatcher --> PdfConv
    Dispatcher --> VidConv
    
    ImgConv --> FilePicker
    PdfConv --> FilePicker
    VidConv --> FilePicker
    
    FilePicker --> Utils
    
    ImgConv -.-> Pillow
    PdfConv -.-> PyMuPDF
    VidConv -.-> FFmpeg
    
    Theme -.-> Menu
    Theme -.-> FilePicker
    
    ImgConv --> Menu
    PdfConv --> Menu
    VidConv --> Menu
    
    style App fill:#4A90E2,stroke:#2E5C8A,color:#fff,stroke-width:3px
    style Dispatcher fill:#50C878,stroke:#2E7D4E,color:#fff,stroke-width:3px
    style ImgConv fill:#F39C12,stroke:#C87F0A,color:#fff
    style PdfConv fill:#F39C12,stroke:#C87F0A,color:#fff
    style VidConv fill:#F39C12,stroke:#C87F0A,color:#fff
    style Menu fill:#9B59B6,stroke:#6C3A80,color:#fff
    style FilePicker fill:#9B59B6,stroke:#6C3A80,color:#fff
    style Splash fill:#9B59B6,stroke:#6C3A80,color:#fff
    style Utils fill:#95A5A6,stroke:#5F6A6B,color:#fff
    style Theme fill:#E74C3C,stroke:#A93226,color:#fff
```

*(charts made by claude)*

## Core Design Principles

- **Separation of UI and core logic**
- **Explicit data flow**
- **No hidden global state**
- **Minimal coupling between modules**
- **Feature scope intentionally limited**

## Layer Responsibilities

### Entry Point (`app.py`)
- Application lifecycle
- Startup sequence
- High-level control flow

### UI Layer (`ui/`)
- Display menus and prompts
- Collect user input
- Validate basic input correctness
- Present results and errors

### Dispatcher (`core/dispatcher.py`)
- Route user intent to the correct converter
- Act as a bridge between UI and conversion logic

### Converter Layer (`converters/`)
Each converter follows the same pattern:
- **Image Converter** - PNG, JPG, WEBP conversion (Pillow)
- **PDF Converter** - Image ↔ PDF conversion (PyMuPDF)
- **Video Converter** - MP4 → GIF conversion (FFmpeg)

### Support Systems
- **Utils** - Shared helper logic and system operations
- **Theme** - Centralized color definitions and styling

## Error Handling Strategy

- Fail early on invalid input
- Validate before conversion
- Surface errors clearly to the user
- Avoid silent failures
- Never crash on user input

## Summary

Morphy's architecture is intentionally simple and explicit:
- **UI handles interaction**
- **Dispatcher handles routing**
- **Converters handle work**
- **Utilities handle system interaction**
