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
use ui::menus::show_menu;
use ui::filepicker::pick_file;
use crate::core::registry::Registry;

fn main() {
    // show splash screen on startup
    show_splash();
    
    // load registry of operations
    let registry = Registry::new();

    // main app loop
    loop {
        let choice_id = match show_menu(&registry) {
            Some(id) => id,
            None => {
                println!("Exiting Morphy.");
                break;
            }
        };

        let path = pick_file();
        
        // Dispatch
        if let Some(op) = registry.operations.iter().find(|o| o.id == choice_id) {
            (op.handler)(&path);
        } else {
            println!("ERROR: Operation handler not found.");
        }
    }

    println!("Application finished.");
}
