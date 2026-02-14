"""
app.py -
entry point of the app
starts the programm
controls the  flow of app
calls the ui func in order when told
"""

from core.dispatcher import dispatch
from ui.filepicker import pick_file
from ui.menus import show_compression_menu
from ui.menus import show_menu
from ui.splash import show_splash


def run_app():
    show_splash()
    choice = show_menu()
    
    if choice == "EXIT":
        print("Exiting Morphy.")
        return
    
    if choice == "COMPRESSION":
        comp_choice = show_compression_menu()
        if comp_choice == "BACK":
            print("Exiting Morphy.")
            return
        path = pick_file()
        dispatch(comp_choice, path)
    else:
        path = pick_file()
        dispatch(choice, path)


if __name__ == "__main__":
    run_app()
