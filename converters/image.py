from PIL import Image


def conv_image(path):
    print("image convert called")
    print(path)
    with Image.open(path) as img:
        print(img.filename)
        print(img.format)
        print(img.size)
