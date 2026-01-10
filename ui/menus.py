"""
menus.py -
display the options to user , collect user choice and return choice to core
"""


def show_menu():
    image, video, doc = "IMAGE", "VIDEO", "DOC"  # Define return values
    while True:  # Loop until valid input
        print("SELECT TYPE OF FILE\n")
        try:
            choice = int(input("1.IMAGE\n2.VIDEO\n3.DOC\n"))
            if choice == 1:
                return image
            elif choice == 2:
                return video
            elif choice == 3:
                return doc
            else:
                print("INPUT ERROR: SELECT VALID OPTION")
        except ValueError:
            print("INPUT ERROR: ENTER A NUMBER")
