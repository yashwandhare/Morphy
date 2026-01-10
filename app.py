"""
app.py -
entry point of the app
starts the programm
controls the  flow of app
calls the ui func in order when told
"""

# importing func from UI
from core.dispatcher import dispatch
from ui.filepicker import pick_file
from ui.menus import show_menu
from ui.splash import show_splash


# func that tells what the app.py actually runs
def run_app():
    show_splash()
    choice = show_menu()
    path = pick_file()
    dispatch(choice, path)


# app.py runs only when its executed directly call run func
if __name__ == "__main__":
    run_app()
