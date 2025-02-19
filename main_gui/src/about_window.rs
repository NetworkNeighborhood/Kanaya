use winsafe::{co, gui, prelude::*};

use crate::{IDC_ABOUT_COPYRIGHT, IDC_ABOUT_DESCRIPTION, IDC_ABOUT_HOMEPAGE, IDC_ABOUT_OK, IDD_ABOUT_DIALOG};

pub struct AboutWindow {
    wnd: gui::WindowModal,
    button_ok: gui::Button,
    button_homepage: gui::Button,
    label_description: gui::Label,
    label_copyright: gui::Label,
}

impl AboutWindow {
    pub fn new(parent: &impl GuiParent) -> Self {
        let dont_move = (gui::Horz::None, gui::Vert::None);
        
        let wnd = gui::WindowModal::new_dlg(parent, IDD_ABOUT_DIALOG as u16);
        let button_ok = gui::Button::new_dlg(&wnd, IDC_ABOUT_OK as u16, dont_move);
        let button_homepage = gui::Button::new_dlg(&wnd, IDC_ABOUT_HOMEPAGE as u16, dont_move);
        let label_description = gui::Label::new_dlg(&wnd, IDC_ABOUT_DESCRIPTION as u16, dont_move);
        let label_copyright = gui::Label::new_dlg(&wnd, IDC_ABOUT_COPYRIGHT as u16, dont_move);
        
        let new_self = Self { wnd, button_ok, button_homepage, label_description, label_copyright };
        unsafe { new_self.register_dialog_procedure(); }
        new_self
    }
    
    pub fn show_modal(&self) {
        self.wnd.show_modal().unwrap();
    }
    
    pub fn close(&self) {
        self.wnd.hwnd().EndDialog(0).expect("Failed to close dialog.");
    }
    
    // Unsafe = more efficient code, reuse same pointer instead of cloning the
    // structure needlessly.
    unsafe fn register_dialog_procedure(&self) {
        let self_ptr = self as *const Self;
        
        self.wnd.on().wm_init_dialog(move |_| {
            (*self_ptr).install_strings();
            Ok(true)
        });
        
        self.wnd.on().wm_command(IDC_ABOUT_HOMEPAGE as u16, winsafe::co::BN::CLICKED, move || {
            winsafe::HWND::NULL.ShellExecute("open", "https://github.com/NetworkNeighborhood/Kanaya", None, None, co::SW::SHOWNORMAL)
                .expect("Failed to open homepage website.");
            (*self_ptr).close();
            Ok(gui::WmRet::HandledOk)
        });
        
        //
        // Dialog closing methods:
        //
        self.wnd.on().wm_command(IDC_ABOUT_OK as u16, winsafe::co::BN::CLICKED, move || {
            (*self_ptr).close();
            Ok(gui::WmRet::HandledOk)
        });
        
        self.wnd.on().wm_command_accel_menu(co::DLGID::CANCEL, move || {
            (*self_ptr).close();
            Ok(())
        });
    }
    
    fn install_strings(&self) {
        self.wnd.set_text(&format!("About {}", env!("KANAYA_NAME_DISPLAY")));
        self.button_ok.set_text("&OK");
        self.button_homepage.set_text("&Homepage");
        self.label_description.set_text(&format!("{} Visual Style editor", env!("KANAYA_NAME_DISPLAY")));
        self.label_copyright.set_text(&format!("Copyright (C) 2025 {}", env!("KANAYA_PUBLISHER_DISPLAY")));
    }
}