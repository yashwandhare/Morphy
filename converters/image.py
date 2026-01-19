"""
image.py - shows image info and converts formats
"""

import os

from PIL import Image
from rich.console import Console
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme

console = Console()


def conv_image(path):
    # read image details
    with Image.open(path) as img:
        table = Table(
            title="Image Metadata",
            show_header=False,
            box=None,
            title_style=f"{Theme.HEADER} bold",
        )
        table.add_row("Filename", img.filename, style=Theme.TEXT)
        table.add_row("Format", img.format, style=Theme.TEXT)
        table.add_row("Mode", img.mode, style=Theme.TEXT)
        table.add_row("Size", f"{img.size[0]}x{img.size[1]}", style=Theme.TEXT)
        console.print(table)

        # show options
        console.print(f"\n[bold {Theme.HEADER}]CONVERT TO[/bold {Theme.HEADER}]")
        console.print(f"1. PNG  [{Theme.DIM}](Lossless)[/{Theme.DIM}]")
        console.print(f"2. JPG  [{Theme.DIM}](Smaller size)[/{Theme.DIM}]")
        console.print(f"3. WEBP [{Theme.DIM}](Modern)[/{Theme.DIM}]")

        # get target format
        choice_map = {"1": "PNG", "2": "JPEG", "3": "WEBP"}
        output = Prompt.ask(
            f"[{Theme.PROMPT}]Select format[/{Theme.PROMPT}]",
            choices=list(choice_map.keys()),
            default="1",
        )
        target_format = choice_map[output]

    # run conversion
    return convert_image(path, target_format)


def convert_image(path, target_format):
    original_dir = os.path.dirname(os.path.abspath(path))
    name = os.path.splitext(os.path.basename(path))[0]

    # show spinner
    with console.status(
        f"[bold {Theme.INFO}]Converting to {target_format}...[/bold {Theme.INFO}]"
    ):
        with Image.open(path) as img:
            # convert to rgb for jpg/webp
            if img.mode in ["RGBA", "LA", "L"] and target_format in ["JPEG", "WEBP"]:
                img = img.convert("RGB")

            out_ext = (
                "png"
                if target_format == "PNG"
                else "jpg"
                if target_format == "JPEG"
                else "webp"
            )
            output_path = os.path.join(original_dir, f"{name}_converted.{out_ext}")

            # save with quality settings if needed
            if target_format in ["JPEG", "WEBP"]:
                img.save(output_path, target_format, quality=100)
            else:
                img.save(output_path, target_format)

    console.print(
        f"[bold {Theme.SUCCESS}]✓ Done![/bold {Theme.SUCCESS}] Saved at: [underline]{output_path}[/underline]"
    )
    return output_path
