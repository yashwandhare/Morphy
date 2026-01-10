# imports
import os

from PIL import Image


# main func
def conv_image(path):
    # uses os module to extract files dir,extension & name
    original_dir = os.path.dirname(os.path.abspath(path))
    ext = os.path.splitext(path)[1].lower()
    name = os.path.splitext(os.path.basename(path))[0]
    # image info
    with Image.open(path) as img:
        print(f"FileName: {img.filename}")
        print(f"Extension: {ext}")
        print(f"FileFormat: {img.format}")
        print(f"FileMode: {img.mode}")
        # conversion options for user
        print("\n CONVERT TO")
        print("1.PNG: Lossless")
        print("2.JPG: Smaller size")
        print("3.WEBP: Modern, efficient compression")
        # get ouptut and map it accordinly
        output = input("Enter the option(num): ").strip()
        formats = {"1": ("png", "PNG"), "2": ("jpg", "JPEG"), "3": ("webp", "WEBP")}
        if output not in formats:  # validation
            print("Invalid option! Defaulting to PNG.")
            output = "1"

        # Convert to RGB only when needed for JPG/WEBP
        if img.mode in ["RGBA", "LA", "L"] and output in ["2", "3"]:
            print("CONVERTING IMAGE TO RGB")
            img = img.convert("RGB")

        out_ext, out_format = formats[output]
        output_path = os.path.join(original_dir, f"{name}_converted.{out_ext}")

        # ✅ Quality=100 ONLY for JPG/WEBP, PNG uses lossless
        if out_format in ["JPEG", "WEBP"]:
            img.save(output_path, out_format, quality=100)
        else:
            img.save(output_path, out_format)

        print(f"✓ Converted and saved: {output_path}")
        print(f"New format: {out_ext.upper()}")

        return output_path
