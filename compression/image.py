"""
image.py - image compression with preset and custom size options
"""

import os
import tempfile
from io import BytesIO

from PIL import Image
from rich.console import Console
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme

console = Console()

PRESETS = {
    "1": {"name": "Light", "quality": 90, "max_dim": 1920, "reduction": "~30%"},
    "2": {"name": "Medium", "quality": 85, "max_dim": 1280, "reduction": "~50%"},
    "3": {"name": "Aggressive", "quality": 75, "max_dim": 720, "reduction": "~70%"},
}


def compress_image(path):
    if not _validate_image(path):
        return

    img = _load_image(path)
    if img is None:
        return

    original_size = os.path.getsize(path)
    _show_info(path, img, original_size)

    choice = _get_compression_choice()
    
    if choice == "4":
        target_kb = _get_target_size()
        output_path = _compress_to_target(path, img, target_kb)
    else:
        output_path = _compress_with_preset(path, img, PRESETS[choice])

    if output_path:
        _show_comparison(original_size, output_path)


def _validate_image(path):
    valid_extensions = {".png", ".jpg", ".jpeg", ".webp", ".bmp", ".tiff"}
    ext = os.path.splitext(path)[1].lower()
    
    if ext not in valid_extensions:
        console.print(f"[{Theme.ERROR}]Invalid file type. Supported: PNG, JPG, WEBP, BMP, TIFF[/{Theme.ERROR}]")
        return False
    return True


def _load_image(path):
    try:
        return Image.open(path)
    except Exception as e:
        console.print(f"[{Theme.ERROR}]Failed to open image: {e}[/{Theme.ERROR}]")
        return None


def _show_info(path, img, size_bytes):
    size_kb = size_bytes / 1024
    size_mb = size_kb / 1024
    
    if size_mb >= 1:
        size_str = f"{size_mb:.2f} MB"
    else:
        size_str = f"{size_kb:.2f} KB"

    console.print(f"\n[bold {Theme.HEADER}]IMAGE COMPRESSION[/bold {Theme.HEADER}]")
    
    table = Table(show_header=False, box=None)
    table.add_row("File", os.path.basename(path), style=Theme.TEXT)
    table.add_row("Size", size_str, style=Theme.TEXT)
    table.add_row("Dimensions", f"{img.width}x{img.height}", style=Theme.TEXT)
    console.print(table)


def _get_compression_choice():
    console.print(f"\n[bold {Theme.HEADER}]Select compression level:[/bold {Theme.HEADER}]")
    console.print(f"  1  Light       quality: 90  |  max: 1920px  |  ~30% smaller")
    console.print(f"  2  Medium      quality: 85  |  max: 1280px  |  ~50% smaller")
    console.print(f"  3  Aggressive  quality: 75  |  max: 720px   |  ~70% smaller")
    console.print(f"  4  Custom      Enter target size in KB")
    console.print(f"                 [{Theme.DIM}]Examples: 500kb, 250kb, 100kb[/{Theme.DIM}]")
    console.print(f"                 [{Theme.DIM}]Note: For MB targets, use options 1-3[/{Theme.DIM}]")

    return Prompt.ask(
        f"\n[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
        choices=["1", "2", "3", "4"],
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


def _compress_with_preset(path, img, preset):
    output_path = _get_output_path(path)
    
    with console.status(f"[{Theme.INFO}]Compressing with {preset['name']} settings...[/{Theme.INFO}]"):
        compressed = _apply_preset(img.copy(), preset)
        _save_image(compressed, output_path, preset["quality"], img.format)
    
    return output_path


def _compress_to_target(path, img, target_kb):
    target_bytes = target_kb * 1024
    tolerance = target_bytes * 0.05
    
    with console.status(f"[{Theme.INFO}]Optimizing to target size...[/{Theme.INFO}]"):
        result = _binary_search_quality(path, img, target_bytes, tolerance)
    
    if result["warning"]:
        console.print(f"\n[{Theme.ERROR}]⚠ {result['warning']}[/{Theme.ERROR}]")
    
    output_path = _get_output_path(path)
    
    if result["resized"]:
        final_img = result["image"]
    else:
        final_img = img.copy()
    
    _save_image(final_img, output_path, result["quality"], img.format)
    
    console.print(f"\n[{Theme.INFO}]Quality: {result['quality']} | Resized: {result['resized_dims']}[/{Theme.INFO}]")
    
    return output_path


def _binary_search_quality(path, img, target_bytes, tolerance):
    min_quality = 50
    max_quality = 95
    best_quality = 85
    best_size = float("inf")
    
    original_width, original_height = img.size
    scale_factor = 1.0
    resized = False
    resized_dims = f"{original_width}x{original_height}"
    
    for _ in range(15):
        quality = (min_quality + max_quality) // 2
        size = _get_compressed_size(img, quality, scale_factor)
        
        if abs(size - target_bytes) <= tolerance:
            return {
                "quality": quality,
                "warning": None,
                "resized": resized,
                "resized_dims": resized_dims,
                "image": None,
            }
        
        if size < best_size:
            best_quality = quality
            best_size = size
        
        if size > target_bytes:
            max_quality = quality - 1
        else:
            min_quality = quality + 1
        
        if min_quality > max_quality:
            break
    
    if best_size > target_bytes and scale_factor >= 1.0:
        scale_factors = [0.75, 0.5, 0.35, 0.25, 0.15]
        
        for factor in scale_factors:
            new_width = int(original_width * factor)
            new_height = int(original_height * factor)
            test_size = _get_compressed_size(img, min_quality, factor)
            
            if test_size <= target_bytes + tolerance:
                scale_factor = factor
                resized = True
                resized_dims = f"{new_width}x{new_height}"
                best_quality = min_quality
                best_size = test_size
                break
    
    warning = None
    if best_size > target_bytes + tolerance:
        warning = f"Target not achievable. Best size: {best_size / 1024:.0f} KB"
    
    resized_img = None
    if resized:
        new_width = int(original_width * scale_factor)
        new_height = int(original_height * scale_factor)
        resized_img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
    
    return {
        "quality": best_quality,
        "warning": warning,
        "resized": resized,
        "resized_dims": resized_dims,
        "image": resized_img,
    }


def _get_compressed_size(img, quality, scale_factor=1.0):
    buffer = BytesIO()
    
    if scale_factor < 1.0:
        new_width = int(img.width * scale_factor)
        new_height = int(img.height * scale_factor)
        resized = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
        save_img = resized
    else:
        save_img = img
    
    save_format = img.format if img.format in ["JPEG", "PNG", "WEBP"] else "JPEG"
    
    if save_format == "JPEG" and save_img.mode in ["RGBA", "LA", "P"]:
        save_img = save_img.convert("RGB")
    
    if save_format == "JPEG":
        save_img.save(buffer, format=save_format, quality=quality, optimize=True)
    elif save_format == "PNG":
        save_img.save(buffer, format=save_format, optimize=True, compress_level=9)
    else:
        save_img.save(buffer, format=save_format, quality=quality)
    
    return buffer.tell()


def _apply_preset(img, preset):
    max_dim = preset["max_dim"]
    
    if img.width > max_dim or img.height > max_dim:
        if img.width > img.height:
            new_width = max_dim
            new_height = int(img.height * (max_dim / img.width))
        else:
            new_height = max_dim
            new_width = int(img.width * (max_dim / img.height))
        img = img.resize((new_width, new_height), Image.Resampling.LANCZOS)
    
    return img


def _save_image(img, output_path, quality, original_format):
    save_format = original_format if original_format in ["JPEG", "PNG", "WEBP"] else "JPEG"
    
    if save_format == "JPEG" and img.mode in ["RGBA", "LA", "P"]:
        img = img.convert("RGB")
    
    if save_format == "JPEG":
        img.save(output_path, format=save_format, quality=quality, optimize=True)
    elif save_format == "PNG":
        img.save(output_path, format=save_format, optimize=True, compress_level=9)
    else:
        img.save(output_path, format=save_format, quality=quality)


def _get_output_path(path):
    directory = os.path.dirname(path) or "."
    name = os.path.splitext(os.path.basename(path))[0]
    ext = os.path.splitext(path)[1]
    return os.path.join(directory, f"{name}_compressed{ext}")


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
