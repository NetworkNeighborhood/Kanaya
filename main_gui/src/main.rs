#![windows_subsystem = "windows"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod main_window;
mod about_window;

use main_window::MainWindow;

fn main() {
    let main_window = MainWindow::new();
    main_window.run().unwrap();
}