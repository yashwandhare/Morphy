/**
 * main.rs -
 * entry point of the app
 * starts the program
 * controls the flow of app
 * calls the ui functions in order when told
 */

mod ui;
mod core;
mod converters;
mod compression;

use ui::splash::show_splash;
use ui::menus::{show_menu, show_compression_menu, MenuChoice, CompressionChoice};
use ui::filepicker::pick_file;
use core::dispatcher::{dispatch, dispatch_compression};

fn main() {
    // show splash screen on startup
    show_splash();

    // main app loop
    loop {
        let choice = show_menu();

        if choice == MenuChoice::Exit {
            println!("Exiting Morphy.");
            break;
        }

        // compression has a sub-menu
        if choice == MenuChoice::Compression {
            let comp_choice = show_compression_menu();
            if comp_choice == CompressionChoice::Back {
                continue;
            }
            let path = pick_file();
            dispatch_compression(&comp_choice, &path);
            continue;
        }

        let path = pick_file();
        dispatch(&choice, &path);
    }

    println!("Application finished.");
}
