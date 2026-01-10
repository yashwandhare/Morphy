# imports
import os

from PIL import Image


# UI wrapper - shows metadata, gets choice, maps to format, calls core
def conv_image(path):
    with Image.open(path) as img:
        ext = os.path.splitext(path)[1].lower()

        print(f"FileName: {img.filename}")
        print(f"Extension: {ext}")
        print(f"FileFormat: {img.format}")
        print(f"FileMode: {img.mode}")
        print("\n CONVERT TO")
        print("1.PNG: Lossless")
        print("2.JPG: Smaller size")
        print("3.WEBP: Modern, efficient compression")

        output = input("Enter the option(num): ").strip()
        choice_map = {"1": "PNG", "2": "JPEG", "3": "WEBP"}
        if output not in choice_map:
            print("Invalid option! Defaulting to PNG.")
            target_format = "PNG"
        else:
            target_format = choice_map[output]

    # Call core with just path and target_format
    return convert_image(path, target_format)


# Core engine - self-contained, computes path info internally
def convert_image(path, target_format):
    original_dir = os.path.dirname(os.path.abspath(path))
    name = os.path.splitext(os.path.basename(path))[0]

    with Image.open(path) as img:
        # Convert to RGB only when needed for JPG/WEBP
        if img.mode in ["RGBA", "LA", "L"] and target_format in ["JPEG", "WEBP"]:
            print("CONVERTING IMAGE TO RGB")
            img = img.convert("RGB")

        out_ext = (
            "png"
            if target_format == "PNG"
            else "jpg"
            if target_format == "JPEG"
            else "webp"
        )
        output_path = os.path.join(original_dir, f"{name}_converted.{out_ext}")

        # Quality=100 ONLY for JPEG/WEBP, PNG uses lossless
        if target_format in ["JPEG", "WEBP"]:
            img.save(output_path, target_format, quality=100)
        else:
            img.save(output_path, target_format)

        print(f"✓ Converted and saved: {output_path}")
        print(f"New format: {out_ext.upper()}")

    return output_path
