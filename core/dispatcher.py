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
    else:
        print("ERROR: invalid input for file type")
