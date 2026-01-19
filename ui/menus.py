"""
menus.py - shows main menu and gets user choice
"""

from rich.console import Console
from rich.prompt import Prompt
from rich.table import Table

from ui.theme import Theme


def show_menu():
    console = Console()
    image, video, doc, exit_app = ("IMAGE", "VIDEO", "DOC", "EXIT")

    # setup table structure
    table = Table(show_header=False, box=None, padding=(0, 2))
    table.add_column("ID", style=f"{Theme.BORDER} bold")
    table.add_column("Option", style=Theme.TEXT)

    table.add_row("1", "Image Conversion")
    table.add_row("2", "Video to GIF")
    table.add_row("3", "PDF Tools")
    table.add_row("0", "Exit")

    # print menu with info header
    console.print(f"\n[bold {Theme.INFO}]SELECT OPERATION[/bold {Theme.INFO}]")
    console.print(table)
    console.print()

    choice_map = {"1": image, "2": video, "3": doc, "0": exit_app}

    # loop until valid input
    while True:
        choice = Prompt.ask(
            f"[{Theme.PROMPT}]Enter choice[/{Theme.PROMPT}]",
            choices=list(choice_map.keys()),
            default="0",
        )
        return choice_map[choice]
