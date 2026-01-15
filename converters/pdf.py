# importing modules
import os

import fitz  # for pdf to img  ; img to pdf conversion


# func to extract the path of input
def get_ext(path):
    return os.path.splitext(path)[1].lower()


# func to extract user input
def conv_doc(path):
    ext = get_ext(path)

    print("\n CONVERT TO")
    print("1.PDF (from Image)")
    print("2.Image (from PDF)")

    output = input("Enter the option(num): ").strip()
    IMAGE_EXTS = {".png", ".jpg", ".jpeg", ".webp", ".bmp"}
    PDF_EXTS = {".pdf"}

    if output == "1":
        # Image → PDF
        if ext not in IMAGE_EXTS:
            print("Invalid input: expected an image file")
            return None
        return img_to_pdf(path)

    elif output == "2":
        # PDF → Image
        if ext not in PDF_EXTS:
            print("Invalid input: expected a PDF file")
            return None
        return pdf_to_img(path)

    else:
        print("Invalid option")
        return None


def img_to_pdf(path):
    if not os.path.isfile(path):  # validates file path
        print("File not found")
        return None
    # extracts the path dir
    out_dir = os.path.dirname(path) or "."
    # extract file name extension & size
    file_name = os.path.basename(path)
    name, ext = os.path.splitext(file_name)
    file_size = os.path.getsize(path)

    print(f"File name : {file_name}")
    print(f"Extension : {ext}")
    print(f"Size      : {file_size / 1024:.2f} KB")
    # output will be image name + .pdf
    output_pdf = os.path.join(out_dir, name + ".pdf")

    # Ask user for page size preference
    print("\nPAGE SIZE")
    print("1. Standard A4 (Centered, Original Size)")
    print("2. Original Image Size (Fit Page to Image)")
    size_opt = input("Enter option(num): ").strip()

    doc = fitz.open()  # creates new empty pdf doc in memory
    img = fitz.open(path)  # open img file as a single page doc

    rect = img[0].rect  # image wrapped in page 0 in rectangle

    if size_opt == "1":
        # A4 size is 595 x 842 points
        page = doc.new_page(width=595, height=842)

        # Calculate coordinates to center the image (Original Size)
        # x = (PageWidth - ImageWidth) / 2
        x = (595 - rect.width) / 2
        y = (842 - rect.height) / 2

        # Insert image at calculated position without scaling
        page.insert_image(
            fitz.Rect(x, y, x + rect.width, y + rect.height), filename=path
        )
    else:
        page = doc.new_page(
            width=rect.width, height=rect.height
        )  # new page in pdf with page size = img size
        page.insert_image(rect, filename=path)  # inserts image into page

    doc.save(output_pdf)  # saves it to disk
    doc.close()  # removes from memory
    img.close()

    print(f"Saved as  : {output_pdf}")
    return output_pdf


def pdf_to_img(path):
    if not os.path.isfile(path):
        print("File not found")
        return None

    out_dir = os.path.dirname(path) or "."

    pdf = fitz.open(path)  # opens the file and loads it in memory
    base = os.path.splitext(os.path.basename(path))[
        0
    ]  # extracts file name without extension

    output_paths = []

    for i, page in enumerate(pdf):  # iterates over each pdf page , starts at 0 ofc
        # matrix(3, 3) zooms in 3x (~216 DPI) for higher quality output
        pix = page.get_pixmap(
            matrix=fitz.Matrix(3, 3)
        )  # renders per page into a bitmap
        img_path = os.path.join(
            out_dir, f"{base}_page_{i + 1}.png"
        )  # builds image name ; pdf name + page number from 1 to x + .png
        pix.save(img_path)  # writes the rendered img to disk ; lossless
        output_paths.append(img_path)
        print(f"Saved      : {img_path}")  # status

    pdf.close()  # frees memory
    return output_paths
