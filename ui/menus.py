"""
menus.py -
display the options to user , collect user choice and return choice to core
"""


def show_menu():
    image, video, doc, exit_app = (
        "IMAGE",
        "VIDEO",
        "DOC",
        "EXIT",
    )  # Define return values
    while True:  # Loop until valid input
        print("SELECT TYPE OF FILE\n")
        try:
            choice = int(input("1.IMAGE\n2.VIDEO\n3.DOC\n0.EXIT\n"))
            if choice == 1:
                return image
            elif choice == 2:
                return video
            elif choice == 3:
                return doc
            elif choice == 0:
                return exit_app
            else:
                print("INPUT ERROR: SELECT VALID OPTION")
        except ValueError:
            print("INPUT ERROR: ENTER A NUMBER")
