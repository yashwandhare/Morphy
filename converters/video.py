"""
video.py - converts video to gif using ffmpeg
"""

import os
import subprocess

from rich.console import Console
from rich.prompt import Prompt

from ui.theme import Theme

console = Console()


def check_ffmpeg():
    # verify ffmpeg is installed
    try:
        subprocess.run(
            ["ffmpeg", "-version"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=True,
        )
        return True
    except (FileNotFoundError, subprocess.CalledProcessError):
        return False


def conv_video(path):
    console.rule(f"[bold {Theme.HEADER}]VIDEO TO GIF[/bold {Theme.HEADER}]")

    # check dependencies
    with console.status(f"[{Theme.INFO}]Checking FFmpeg...[/{Theme.INFO}]"):
        if not check_ffmpeg():
            console.print(
                f"[bold {Theme.ERROR}]ERROR:[/bold {Theme.ERROR}] FFmpeg is not installed or not in PATH."
            )
            return
        console.print(f"[{Theme.SUCCESS}]✓ FFmpeg detected.[/{Theme.SUCCESS}]")

    # get conversion settings
    fps = Prompt.ask(
        f"[{Theme.HEADER}]Select FPS[/{Theme.HEADER}]",
        choices=["10", "24", "30"],
        default="10",
    )
    width = Prompt.ask(
        f"[{Theme.HEADER}]Select Width[/{Theme.HEADER}]",
        choices=["320", "480", "720", "1080"],
        default="480",
    )

    convert_to_gif(path, fps, width)


def convert_to_gif(path, fps, width):
    if not os.path.isfile(path):
        console.print(f"[{Theme.ERROR}]File not found[/{Theme.ERROR}]")
        return None

    # define paths
    out_dir = os.path.dirname(path) or "."
    name = os.path.splitext(os.path.basename(path))[0]
    output_gif = os.path.join(out_dir, name + ".gif")
    palette_png = os.path.join(out_dir, f"{name}_palette.png")
    filters = f"fps={fps},scale={width}:-1:flags=lanczos"

    run_kwargs = {
        "check": True,
        "stdout": subprocess.DEVNULL,
        "stderr": subprocess.STDOUT,
    }

    try:
        # process with spinner
        with console.status(
            f"[bold {Theme.INFO}]Processing (this might take a moment)...[/bold {Theme.INFO}]"
        ):
            # generate palette
            subprocess.run(
                [
                    "ffmpeg",
                    "-y",
                    "-i",
                    path,
                    "-vf",
                    f"{filters},palettegen",
                    palette_png,
                ],
                **run_kwargs,
            )
            # generate final gif
            subprocess.run(
                [
                    "ffmpeg",
                    "-y",
                    "-i",
                    path,
                    "-i",
                    palette_png,
                    "-lavfi",
                    f"{filters} [x]; [x][1:v] paletteuse",
                    output_gif,
                ],
                **run_kwargs,
            )

        console.print(
            f"\n[bold {Theme.SUCCESS}]Success![/bold {Theme.SUCCESS}] GIF saved at: {output_gif}"
        )

    except subprocess.CalledProcessError:
        console.print(
            f"[bold {Theme.ERROR}]Error: Conversion failed.[/bold {Theme.ERROR}]"
        )
        return None
    finally:
        # cleanup palette
        if os.path.exists(palette_png):
            os.remove(palette_png)
