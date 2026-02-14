from compression.image import compress_image
from compression.pdf import compress_pdf
from converters.image import conv_image
from converters.pdf import conv_doc
from converters.video import conv_video


def dispatch(choice, path):
    if choice == "IMAGE":
        conv_image(path)
    elif choice == "VIDEO":
        conv_video(path)
    elif choice == "DOC":
        conv_doc(path)
    elif choice == "IMAGE_COMPRESS":
        compress_image(path)
    elif choice == "PDF_COMPRESS":
        compress_pdf(path)
    else:
        print("ERROR: invalid input for file type")
