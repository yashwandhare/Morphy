from rich.console import Console

from .theme import Theme


def show_splash():
    """
    Displays the styled ASCII splash screen with correct alignment using Theme colors.
    """
    console = Console()

    # Geometric ASCII Art
    logo = [
        r"███╗   ███╗  ██████╗  ██████╗  ██████╗  ██╗  ██╗ ██╗   ██╗",
        r"████╗ ████║ ██╔═══██╗ ██╔══██╗ ██╔══██╗ ██║  ██║ ╚██╗ ██╔╝",
        r"██╔████╔██║ ██║   ██║ ██████╔╝ ██████╔╝ ███████║  ╚████╔╝ ",
        r"██║╚██╔╝██║ ██║   ██║ ██╔══██╗ ██╔═══╝  ██╔══██║   ╚██╔╝  ",
        r"██║ ╚═╝ ██║ ╚██████╔╝ ██║  ██║ ██║      ██║  ██║    ██║   ",
        r"╚═╝     ╚═╝  ╚═════╝  ╚═╝  ╚═╝ ╚═╝      ╚═╝  ╚═╝    ╚═╝   ",
    ]

    # --- Dimensions ---
    content_width = len(logo[0])
    padding = 4
    inner_width = content_width + (padding * 2)

    # --- Text Content ---
    left_text = "simple file converter"
    right_text = "made by kazuto"
    spacer_len = inner_width - 8 - len(left_text) - len(right_text)

    # --- Render ---
    # Define colors from Theme
    border_c = Theme.BORDER
    logo_c = Theme.HEADER
    text_c = Theme.TEXT

    # Top Border
    console.print(f"[{border_c}]╔{'═' * inner_width}╗[/{border_c}]")

    # Top Spacer
    console.print(f"[{border_c}]║{' ' * inner_width}║[/{border_c}]")

    # Logo Section
    for line in logo:
        console.print(
            f"[{border_c}]║[/{border_c}]{' ' * padding}[{logo_c}]{line}[/{logo_c}]{' ' * padding}[{border_c}]║[/{border_c}]"
        )

    # Middle Spacer
    console.print(f"[{border_c}]║{' ' * inner_width}║[/{border_c}]")

    # Subtext Section
    console.print(
        f"[{border_c}]║[/{border_c}]    [{text_c}]{left_text}{' ' * spacer_len}{right_text}[/{text_c}]    [{border_c}]║[/{border_c}]"
    )

    # Bottom Spacer
    console.print(f"[{border_c}]║{' ' * inner_width}║[/{border_c}]")

    # Bottom Border
    console.print(f"[{border_c}]╚{'═' * inner_width}╝[/{border_c}]")
