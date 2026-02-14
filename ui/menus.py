"""
menus.py - shows main menu and gets user choice
"""

from rich.console import Console
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme


def show_menu():
    console = Console()
    image, video, doc, compression, exit_app = ("IMAGE", "VIDEO", "DOC", "COMPRESSION", "EXIT")

    table = Table(show_header=False, box=None, padding=(0, 2))
    table.add_column("ID", style=f"{Theme.BORDER} bold")
    table.add_column("Option", style=Theme.TEXT)

    table.add_row("1", "Image Conversion")
    table.add_row("2", "Video to GIF")
    table.add_row("3", "PDF Tools")
    table.add_row("4", "Compression")
    table.add_row("0", "Exit")

    console.print(f"\n[bold {Theme.INFO}]SELECT OPERATION[/bold {Theme.INFO}]")
    console.print(table)
    console.print()

    choice_map = {"1": image, "2": video, "3": doc, "4": compression, "0": exit_app}

    while True:
        choice = Prompt.ask(
            f"[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
            choices=list(choice_map.keys()),
            default="0",
        )
        return choice_map[choice]


def show_compression_menu():
    console = Console()
    image_compress, pdf_compress, back = ("IMAGE_COMPRESS", "PDF_COMPRESS", "BACK")

    table = Table(show_header=False, box=None, padding=(0, 2))
    table.add_column("ID", style=f"{Theme.BORDER} bold")
    table.add_column("Option", style=Theme.TEXT)

    table.add_row("1", "Image Compression")
    table.add_row("2", "PDF Compression")
    table.add_row("0", "Back")

    console.print(f"\n[bold {Theme.INFO}]COMPRESSION[/bold {Theme.INFO}]")
    console.print(table)
    console.print()

    choice_map = {"1": image_compress, "2": pdf_compress, "0": back}

    while True:
        choice = Prompt.ask(
            f"[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
            choices=list(choice_map.keys()),
            default="0",
        )
        return choice_map[choice]
