#![windows_subsystem = "windows"]

mod uxthemehook;

mod main_window;

use main_window::MainWindow;
use winsafe::{prelude::*, co,};

fn main() {
    uxthemehook::initialize_hooks().unwrap();
    
    if std::env::args().count() < 2 {
        winsafe::HWND::NULL.MessageBox("Pass a path to a .msstyles file via the command line to open the controls preview.", "Error", co::MB::OK | co::MB::ICONERROR)
            .expect("Failed to create message box.");
    }
    else {
        // Get the theme path from arguments:
        let mut theme_path: Vec<_> = std::env::args().collect();
        theme_path.remove(0);
        let theme_path = theme_path.join("");
        
        uxthemehook::load_global_theme(&theme_path);
        
        winsafe::HWND::NULL.MessageBox("This is a test message box to see how the visual style loader is doing.", "Test", co::MB::CANCELTRYCONTINUE | co::MB::ICONINFORMATION)
            .expect("Failed to create message box.");
        
        let main_window = MainWindow::new();
        main_window.run().unwrap();
    }
}