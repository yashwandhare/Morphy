"""
pdf.py - handles pdf and image conversions
"""

import os

import fitz
from rich.console import Console
from rich.progress import track
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme

console = Console()


def conv_doc(path):
    ext = os.path.splitext(path)[1].lower()

    # show menu options
    console.print(f"\n[bold {Theme.HEADER}]PDF TOOLS[/bold {Theme.HEADER}]")
    console.print("1. Image -> PDF")
    console.print("2. PDF -> Images")

    choice = Prompt.ask(
        f"[{Theme.PROMPT}]Select option[/{Theme.PROMPT}]", choices=["1", "2"]
    )

    # route logic based on file type
    if choice == "1":
        if ext not in {".png", ".jpg", ".jpeg", ".webp", ".bmp"}:
            console.print(
                f"[{Theme.ERROR}]Invalid file. Select an image.[/{Theme.ERROR}]"
            )
            return
        img_to_pdf(path)
    else:
        if ext != ".pdf":
            console.print(f"[{Theme.ERROR}]Invalid file. Select a PDF.[/{Theme.ERROR}]")
            return
        pdf_to_img(path)


def img_to_pdf(path):
    # display file stats
    file_size = os.path.getsize(path) / 1024
    table = Table(show_header=False, box=None)
    table.add_row("File", os.path.basename(path), style=Theme.TEXT)
    table.add_row("Size", f"{file_size:.2f} KB", style=Theme.TEXT)
    console.print(table)

    # get layout choice
    console.print(f"\n[bold {Theme.HEADER}]PAGE SIZING[/bold {Theme.HEADER}]")
    console.print("1. A4 Centered")
    console.print("2. Original Fit")
    size_opt = Prompt.ask(
        f"[{Theme.PROMPT}]Select sizing[/{Theme.PROMPT}]",
        choices=["1", "2"],
        default="1",
    )

    # create pdf
    with console.status(f"[bold {Theme.INFO}]Building PDF...[/bold {Theme.INFO}]"):
        out_dir = os.path.dirname(path) or "."
        name = os.path.splitext(os.path.basename(path))[0]
        output_pdf = os.path.join(out_dir, name + ".pdf")

        doc = fitz.open()
        img = fitz.open(path)
        rect = img[0].rect

        if size_opt == "1":
            page = doc.new_page(width=595, height=842)
            x = (595 - rect.width) / 2
            y = (842 - rect.height) / 2
            page.insert_image(
                fitz.Rect(x, y, x + rect.width, y + rect.height), filename=path
            )
        else:
            page = doc.new_page(width=rect.width, height=rect.height)
            page.insert_image(rect, filename=path)

        doc.save(output_pdf)
        doc.close()
        img.close()

    console.print(f"[bold {Theme.SUCCESS}]✓ Saved:[/bold {Theme.SUCCESS}] {output_pdf}")


def pdf_to_img(path):
    out_dir = os.path.dirname(path) or "."
    base = os.path.splitext(os.path.basename(path))[0]

    pdf = fitz.open(path)
    output_paths = []

    # loop pages with progress bar
    for i, page in track(
        enumerate(pdf),
        total=len(pdf),
        description=f"[{Theme.INFO}]Extracting pages...[/{Theme.INFO}]",
    ):
        pix = page.get_pixmap(matrix=fitz.Matrix(3, 3))
        img_path = os.path.join(out_dir, f"{base}_page_{i + 1}.png")
        pix.save(img_path)
        output_paths.append(img_path)

    pdf.close()
    console.print(
        f"[bold {Theme.SUCCESS}]✓ Extracted {len(output_paths)} pages.[/bold {Theme.SUCCESS}]"
    )
