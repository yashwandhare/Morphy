# importing modules as usual
import os
import subprocess


# func to extract path
def get_ext(path):
    return os.path.splitext(path)[1].lower()


# func to check ffmpeg ; validation
def check_ffmpeg():
    try:  # try block
        subprocess.run(  # runs command via subprocess and stores it in result for python
            ["ffmpeg", "-version"],  # using list to prevent injections
            stdout=subprocess.DEVNULL,  # fetch standard output from ffmpeg
            stderr=subprocess.DEVNULL,
            check=True,
        )
        return True  # returns result
    except (
        FileNotFoundError,
        subprocess.CalledProcessError,
    ):  # error handeling is ffmpeg doesn't exists
        return False


# func to display stuff
def conv_video(path):
    print("CONVERT YOUR VIDEOS TO GIF")
    print("checking ffmpeg...")

    ffmpeg_status = check_ffmpeg()  # validation

    if not ffmpeg_status:
        print("FFmpeg is not installed or not in PATH")
        return

    print("FFmpeg detected.\n")  # else

    # ask user for settings
    print("Select FPS: 10, 24, 30")  # fps
    fps_input = input("FPS: ").strip()
    if not fps_input.isdigit():
        fps_input = "10"  # Default safe value

    print("\nSelect Width: 320, 480, 720, 1080")  # res
    width_input = input("Width: ").strip()
    if not width_input.isdigit():
        width_input = "480"  # Default safe value

    print("\nStarting process...")

    # Call core function with user inputs
    saved_path = convert_to_gif(path, fps_input, width_input)

    if saved_path:
        print(f"\n[+] Final Output Saved at: {saved_path}")


# func to convert video to gif using ffmpeg
def convert_to_gif(path, fps, width):
    # file validation
    if not os.path.isfile(path):
        print("File not found")
        return None

    # basic declarations
    out_dir = os.path.dirname(path) or "."
    file_name = os.path.basename(path)
    name, ext = os.path.splitext(file_name)

    output_gif = os.path.join(out_dir, name + ".gif")  # output gif ; video_name + .gif
    # Make palette unique to avoid conflicts
    palette_png = os.path.join(out_dir, f"{name}_palette.png")

    # Filters for ffmpeg
    filters = f"fps={fps},scale={width}:-1:flags=lanczos"

    # Common args to reduce duplication
    run_kwargs = {
        "check": True,
        "stdout": subprocess.DEVNULL,  # no logs
        "stderr": subprocess.STDOUT,
    }

    try:
        print(f"Generating palette for {file_name}...")

        # Step 1: Generate Palette
        subprocess.run(
            ["ffmpeg", "-y", "-i", path, "-vf", f"{filters},palettegen", palette_png],
            **run_kwargs,
        )

        print(f"Creating GIF...")

        # Step 2: Use Palette to create GIF
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

        print("Success! GIF created.")

    except subprocess.CalledProcessError:
        print("Error: Conversion failed.")
        return None
    finally:
        # Cleanup: Delete the palette png
        if os.path.exists(palette_png):
            os.remove(palette_png)
            print("Cleaned up temporary palette file.")

    return output_gif
