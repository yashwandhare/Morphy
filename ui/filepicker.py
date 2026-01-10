"""
filepicker.py -
lets user pick file to convert returns path
"""

# to validate the path entered
from pathlib import Path


def pick_file():
    while True:
        path_str = input("ENTER PATH TO FILE: ").strip()
        path = Path(path_str)
        if path.is_file():
            print(f"Valid file found: {path}")
            return str(path)
        else:
            print(f"Invalid or non-existent file: {path_str}. Please try again.")
