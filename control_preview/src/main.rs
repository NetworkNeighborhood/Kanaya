#![windows_subsystem = "windows"]

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

mod main_window;

use main_window::MainWindow;
use winsafe::{prelude::*, co,};

fn main() {
    if std::env::args().count() < 2 {
        winsafe::HWND::NULL.MessageBox("Pass a path to a .msstyles file via the command line to open the controls preview.", "Error", co::MB::OK | co::MB::ICONERROR)
            .expect("Failed to create message box.");
    }
    else {
        let main_window = MainWindow::new();
        main_window.run().unwrap();
    }
}

