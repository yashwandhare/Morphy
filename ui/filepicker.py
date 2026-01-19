"""
filepicker.py - gets and validates file path from user
"""

from pathlib import Path

from rich.console import Console
from rich.prompt import Prompt

from ui.theme import Theme


def pick_file():
    console = Console()

    # loop for file input
    while True:
        path_str = Prompt.ask(
            f"[bold {Theme.INFO}]ENTER PATH TO FILE[/bold {Theme.INFO}]"
        ).strip()
        path = Path(path_str)

        if path.is_file():
            # success message
            console.print(
                f"[{Theme.SUCCESS}]✓ File found:[/{Theme.SUCCESS}] [{Theme.DIM}]{path}[/{Theme.DIM}]\n"
            )
            return str(path)
        else:
            # error message
            console.print(
                f"[{Theme.ERROR}]✗ File not found:[/{Theme.ERROR}] {path_str}"
            )
