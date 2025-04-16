use winsafe::{
    gui, prelude::*, AnyResult, MenuItem, HMENU, seq_ids, co,
};

use fluent::{FluentArgs, FluentBundle, FluentResource};
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

type I18nBundle = fluent_fallback::Bundles<FluentResourceManager>;

pub struct MainWindow {
    wnd: gui::WindowMain,
    fluent_bundles: Rc<I18nBundle>,
}

impl MainWindow {
    pub fn new() -> Self {
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
                title: env!("KANAYA_NAME_DISPLAY").to_string(),
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
    
    pub fn create_menu(i18n: Rc<I18nBundle>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        let mut i18n_errors = vec![];
        
        menu.append_item(&[
            MenuItem::Submenu(&Self::create_menu_file(i18n.clone()), &i18n.format_value_sync("menu-file", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Submenu(&Self::create_menu_edit(i18n.clone()), &i18n.format_value_sync("menu-edit", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&Edit"))).as_ref().unwrap().to_string()),
            MenuItem::Submenu(&Self::create_menu_windows(i18n.clone()), &i18n.format_value_sync("menu-windows", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&Windows"))).as_ref().unwrap().to_string()),
            MenuItem::Submenu(&Self::create_menu_help(i18n.clone()), &i18n.format_value_sync("menu-help", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&Help"))).as_ref().unwrap().to_string()),
        ]).expect("Failed to create menu.");
        
        menu
    }
    
    pub fn create_menu_file(i18n: Rc<I18nBundle>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        let mut i18n_errors = vec![];
        
        menu.append_item(&[
            MenuItem::Entry(IDM_OPEN, &i18n.format_value_sync("menu-file-new", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Entry(IDM_OPEN, &i18n.format_value_sync("menu-file-open", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Entry(IDM_SAVE, &i18n.format_value_sync("menu-file-save", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Entry(IDM_SAVE, &i18n.format_value_sync("menu-file-save-as", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Separator,
            MenuItem::Entry(IDM_EXIT, &i18n.format_value_sync("menu-file-exit", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
        ]).expect("Failed to create File menu.");
        
        menu
    }
    
    pub fn create_menu_edit(i18n: Rc<I18nBundle>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        let mut i18n_errors = vec![];
        
        menu.append_item(&[
            MenuItem::Entry(0, &i18n.format_value_sync("menu-edit-cut", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Entry(0, &i18n.format_value_sync("menu-edit-copy", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Entry(0, &i18n.format_value_sync("menu-edit-paste", None, &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
            MenuItem::Separator,
            // Placeholder item:
            MenuItem::Entry(0, "Create new class"),
        ]).expect("Failed to create Edit menu.");
        
        menu
    }
    
    pub fn create_menu_windows(i18n: Rc<I18nBundle>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        //let mut i18n_errors = vec![];
        
        menu.append_item(&[
            // Placeholder items:
            MenuItem::Entry(0, "D&ock all"),
            MenuItem::Entry(0, "D&etach all"),
        ]).expect("Failed to create Windows menu.");
        
        menu
    }
    
    pub fn create_menu_help(i18n: Rc<I18nBundle>) -> HMENU {
        let menu: HMENU = HMENU::CreateMenu().unwrap();
        
        let mut i18n_errors = vec![];
        
        let mut cut_i18n_args = FluentArgs::new();
        cut_i18n_args.set("kanaya-brand-name", env!("KANAYA_NAME_DISPLAY"));
        
        menu.append_item(&[
            MenuItem::Entry(IDM_ABOUT, &i18n.format_value_sync("menu-help-about", Some(&cut_i18n_args), &mut i18n_errors).unwrap_or(Some(Cow::Borrowed("&File"))).as_ref().unwrap().to_string()),
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