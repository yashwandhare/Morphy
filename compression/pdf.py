"""
pdf.py - pdf compression with preset and custom size options
"""

import os
import tempfile

import fitz
from rich.console import Console
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme

console = Console()

PRESETS = {
    "1": {"name": "Standard", "garbage": 3, "reduction": "~25%"},
    "2": {"name": "Maximum", "garbage": 4, "reduction": "~40%"},
}


def compress_pdf(path):
    if not _validate_pdf(path):
        return

    doc = _load_pdf(path)
    if doc is None:
        return

    original_size = os.path.getsize(path)
    _show_info(path, doc, original_size)

    choice = _get_compression_choice()

    if choice == "3":
        target_kb = _get_target_size()
        output_path = _compress_to_target(path, doc, target_kb, original_size)
    else:
        output_path = _compress_with_preset(path, doc, PRESETS[choice])

    doc.close()

    if output_path:
        _show_comparison(original_size, output_path)


def _validate_pdf(path):
    ext = os.path.splitext(path)[1].lower()
    
    if ext != ".pdf":
        console.print(f"[{Theme.ERROR}]Invalid file type. Please select a PDF.[/{Theme.ERROR}]")
        return False
    return True


def _load_pdf(path):
    try:
        return fitz.open(path)
    except Exception as e:
        console.print(f"[{Theme.ERROR}]Failed to open PDF: {e}[/{Theme.ERROR}]")
        return None


def _show_info(path, doc, size_bytes):
    size_kb = size_bytes / 1024
    size_mb = size_kb / 1024
    
    if size_mb >= 1:
        size_str = f"{size_mb:.2f} MB"
    else:
        size_str = f"{size_kb:.2f} KB"

    console.print(f"\n[bold {Theme.HEADER}]PDF COMPRESSION[/bold {Theme.HEADER}]")
    
    table = Table(show_header=False, box=None)
    table.add_row("File", os.path.basename(path), style=Theme.TEXT)
    table.add_row("Size", size_str, style=Theme.TEXT)
    table.add_row("Pages", str(len(doc)), style=Theme.TEXT)
    console.print(table)


def _get_compression_choice():
    console.print(f"\n[bold {Theme.HEADER}]Select compression level:[/bold {Theme.HEADER}]")
    console.print(f"  1  Standard    garbage: 3  |  deflate: Yes  |  ~25% smaller")
    console.print(f"  2  Maximum     garbage: 4  |  deflate: Yes  |  ~40% smaller")
    console.print(f"  3  Custom      Enter target size in KB")
    console.print(f"                 [{Theme.DIM}]Examples: 500kb, 250kb, 100kb[/{Theme.DIM}]")
    console.print(f"                 [{Theme.DIM}]Note: For MB targets, use options 1-2[/{Theme.DIM}]")

    return Prompt.ask(
        f"\n[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
        choices=["1", "2", "3"],
        default="2",
    )


def _get_target_size():
    while True:
        try:
            value = Prompt.ask(
                f"[{Theme.PROMPT}]Enter target size in KB[/{Theme.PROMPT}]",
                default="500",
            )
            value = value.lower().strip().replace("kb", "").replace(" ", "")
            target_kb = int(value)
            if target_kb <= 0:
                console.print(f"[{Theme.ERROR}]Please enter a positive number[/{Theme.ERROR}]")
                continue
            return target_kb
        except ValueError:
            console.print(f"[{Theme.ERROR}]Invalid input. Enter a number like 500, 250, 100[/{Theme.ERROR}]")


def _compress_with_preset(path, doc, preset):
    output_path = _get_output_path(path)
    
    with console.status(f"[{Theme.INFO}]Compressing with {preset['name']} settings...[/{Theme.INFO}]"):
        doc.save(
            output_path,
            garbage=preset["garbage"],
            deflate=True,
            clean=True,
        )
    
    return output_path


def _compress_to_target(path, doc, target_kb, original_size):
    target_bytes = target_kb * 1024
    
    with console.status(f"[{Theme.INFO}]Compressing to target size...[/{Theme.INFO}]"):
        output_path = _get_output_path(path)
        doc.save(
            output_path,
            garbage=4,
            deflate=True,
            clean=True,
        )
        
        compressed_size = os.path.getsize(output_path)
        
        if compressed_size > target_bytes:
            os.remove(output_path)
            console.print(f"\n[{Theme.ERROR}]⚠ Target {target_kb} KB not achievable (structural compression only)[/{Theme.ERROR}]")
            
            best_kb = compressed_size / 1024
            if best_kb >= 1024:
                best_str = f"{best_kb / 1024:.2f} MB"
            else:
                best_str = f"{best_kb:.0f} KB"
            
            console.print(f"[{Theme.INFO}]Best possible: {best_str} (with Maximum settings)[/{Theme.INFO}]")
            
            console.print(f"\n  1  Proceed with best possible")
            console.print(f"  0  Cancel")
            
            proceed = Prompt.ask(
                f"[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
                choices=["1", "0"],
                default="0",
            )
            
            if proceed == "1":
                doc.save(
                    output_path,
                    garbage=4,
                    deflate=True,
                    clean=True,
                )
                return output_path
            else:
                console.print(f"[{Theme.DIM}]Cancelled.[/{Theme.DIM}]")
                return None
        
        return output_path


def _get_output_path(path):
    directory = os.path.dirname(path) or "."
    name = os.path.splitext(os.path.basename(path))[0]
    return os.path.join(directory, f"{name}_compressed.pdf")


def _show_comparison(original_bytes, output_path):
    compressed_bytes = os.path.getsize(output_path)
    
    original_kb = original_bytes / 1024
    compressed_kb = compressed_bytes / 1024
    
    if original_kb >= 1024:
        original_str = f"{original_kb / 1024:.2f} MB"
    else:
        original_str = f"{original_kb:.2f} KB"
    
    if compressed_kb >= 1024:
        compressed_str = f"{compressed_kb / 1024:.2f} MB"
    else:
        compressed_str = f"{compressed_kb:.2f} KB"
    
    reduction = ((original_bytes - compressed_bytes) / original_bytes) * 100
    
    console.print(f"\n[bold {Theme.SUCCESS}]✓ Done![/bold {Theme.SUCCESS}]")
    
    table = Table(show_header=False, box=None)
    table.add_row("Before", original_str, style=Theme.DIM)
    table.add_row("After", compressed_str, style=Theme.SUCCESS)
    table.add_row("Reduced", f"{reduction:.1f}%", style=Theme.INFO)
    console.print(table)
    
    console.print(f"\n[{Theme.TEXT}]Saved: [underline]{output_path}[/underline][/{Theme.TEXT}]")
