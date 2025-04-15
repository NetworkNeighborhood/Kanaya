use winsafe::{
    gui, prelude::*, AnyResult, MenuItem, HMENU, seq_ids, co,
};

use fluent::{FluentBundle, FluentResource};
use kanaya_common::fluent_resource_manager::FluentResourceManager;
use fluent_fallback::{Localization};
use unic_langid::langid;
use std::{borrow::Cow, env, path::PathBuf, rc::Rc};

use crate::about_window::AboutWindow;

// Private IDs used for window controls:
seq_ids!(
    IDM_OPEN = 2001;
    IDM_SAVE
    IDM_EXIT
    IDM_ABOUT
);

pub struct MainWindow {
    wnd: gui::WindowMain,
    fluent_bundles: Rc<fluent_fallback::Bundles<FluentResourceManager>>,
}

impl MainWindow {
    pub fn new() -> Self {
        // fluent_resmgr STILL USES HARDCODED PATH "./tests/resources".
        let resource_manager = FluentResourceManager::new("./locale/{locale}/{res_id}".to_string());
        
        let loc = Localization::with_env(
            vec!["main_window.ftl".into()],
            true,
            vec![langid!("en-US")],
            resource_manager
        );
        
        let fluent_bundles = loc.bundles().clone();
        
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: env!("KANAYA_NAME_DISPLAY").to_owned(),
                size: (900, 600),
                menu: Self::create_menu(fluent_bundles.clone()),
                style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE | co::WS::SIZEBOX | co::WS::MAXIMIZEBOX | co::WS::MINIMIZEBOX,
                ..Default::default()
            }
        );
        
        let new_self: Self = Self { 
            wnd, 
            fluent_bundles
        };
        
        unsafe { new_self.register_window_procedure(); }
        new_self
    }
    
    pub fn create_menu(i18n: Rc<fluent_fallback::Bundles<FluentResourceManager>>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        let mut i18n_errors = vec![];
        
        if let Ok(file_text) = i18n.format_value_sync("menu-file", None, &mut i18n_errors) {
            winsafe::HWND::NULL.MessageBox(&file_text.as_ref().unwrap().to_string(), "Test", winsafe::co::MB::OK);
            
            menu.append_item(&[
                MenuItem::Submenu(&Self::create_menu_file(), &file_text.as_ref().unwrap().to_string()),
                MenuItem::Submenu(&Self::create_menu_edit(), "&Edit"),
                MenuItem::Submenu(&Self::create_menu_windows(), "&Windows"),
                MenuItem::Submenu(&Self::create_menu_help(), "&Help"),
            ]).expect("Failed to create menu.");
        }
        else {
            winsafe::HWND::NULL.MessageBox("Internal i18n error.", "Le troll", winsafe::co::MB::OK).unwrap();
        }
        
        menu
    }
    
    pub fn create_menu_file() -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        menu.append_item(&[
            MenuItem::Entry(IDM_OPEN, "&New.."),
            MenuItem::Entry(IDM_OPEN, "&Open\tCtrl+O"),
            MenuItem::Entry(IDM_SAVE, "&Save\tCtrl+S"),
            MenuItem::Entry(IDM_SAVE, "Save &As...\tCtrl+Shift+S"),
            MenuItem::Separator,
            MenuItem::Entry(IDM_EXIT, "E&xit\tAlt+F4"),
        ]).expect("Failed to create File menu.");
        
        menu
    }
    
    pub fn create_menu_edit() -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        menu.append_item(&[
            MenuItem::Entry(0, "C&ut\tCtrl+X"),
            MenuItem::Entry(0, "&Copy\tCtrl+C"),
            MenuItem::Entry(0, "&Paste\tCtrl+V"),
            MenuItem::Separator,
            MenuItem::Entry(0, "Create new class"),
        ]).expect("Failed to create Edit menu.");
        
        menu
    }
    
    pub fn create_menu_windows() -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        menu.append_item(&[
            MenuItem::Entry(0, "D&ock all"),
            MenuItem::Entry(0, "D&etach all"),
        ]).expect("Failed to create Windows menu.");
        
        menu
    }
    
    pub fn create_menu_help() -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        menu.append_item(&[
            MenuItem::Entry(IDM_ABOUT, &format!("&About {}", env!("KANAYA_NAME_DISPLAY"))),
        ]).expect("Failed to create Help menu.");
        
        menu
    }
    
    pub fn run(&self) -> AnyResult<i32> {
        self.wnd.run_main(None)
    }
    
    // Unsafe = more efficient code, reuse same pointer instead of cloning the
    // structure needlessly.
    unsafe fn register_window_procedure(&self) {
        let self_ptr = self as *const Self;
        
        self.wnd.on().wm_close(move || {
            winsafe::PostQuitMessage(0);
            Ok(())
        });
        
        // Exit menu item:
        self.wnd.on().wm_command(IDM_EXIT, winsafe::co::BN::CLICKED, move || {
            (*self_ptr).wnd.close();
            Ok(gui::WmRet::HandledOk)
        });
        
        // About menu item:
        self.wnd.on().wm_command(IDM_ABOUT, winsafe::co::BN::CLICKED, move || {
            (*self_ptr).on_menu_about();
            Ok(gui::WmRet::HandledOk)
        });
    }
    
    fn on_menu_about(&self) {
        let about_window: AboutWindow = AboutWindow::new(&self.wnd);
        about_window.show_modal();
    }
}