def show_splash():
    """
    Displays the styled ASCII splash screen with correct alignment.
    """
    # ANSI Color 208 is 'Chrome Orange'
    ORANGE = "\033[38;5;208m"
    RESET = "\033[0m"

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

    # The width of the empty space inside the box (between the vertical borders)
    inner_width = content_width + (padding * 2)

    # --- Text Content ---
    left_text = "simple file converter"
    right_text = "made by kazuto"

    # Calculate the spacer to push texts to opposite sides
    # inner_width - (left_margin + right_margin) - text_lengths
    spacer_len = inner_width - 8 - len(left_text) - len(right_text)

    # --- Render ---
    print(ORANGE)

    # Top Border: The '═' line must match the inner_width exactly
    print("╔" + "═" * inner_width + "╗")

    # Logo Section
    print("║" + " " * inner_width + "║")  # Top spacer
    for line in logo:
        print(f"║{' ' * padding}{line}{' ' * padding}║")
    print("║" + " " * inner_width + "║")  # Middle spacer

    # Subtext Section
    # ║ + 4 spaces + Left Text + Variable Spacer + Right Text + 4 spaces + ║
    print(f"║    {left_text}{' ' * spacer_len}{right_text}    ║")

    # Bottom Spacer & Border
    print("║" + " " * inner_width + "║")
    print("╚" + "═" * inner_width + "╝")

    print(RESET)
