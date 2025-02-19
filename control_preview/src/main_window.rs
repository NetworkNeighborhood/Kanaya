use winsafe::{
    gui, prelude::*, AnyResult, MenuItem, HMENU, seq_ids, co,
};

#[derive(Clone)]
pub struct MainWindow {
    wnd: gui::WindowMain
}

impl MainWindow {
    pub fn new() -> Self {
        let wnd = gui::WindowMain::new(
            gui::WindowMainOpts {
                title: "Control preview".to_owned(),
                size: (900, 600),
                style: co::WS::CAPTION | co::WS::SYSMENU | co::WS::CLIPCHILDREN | co::WS::BORDER | co::WS::VISIBLE | co::WS::SIZEBOX | co::WS::MAXIMIZEBOX | co::WS::MINIMIZEBOX,
                ..Default::default()
            }
        );
        
        let new_self: Self = Self { wnd };
        new_self.register_window_procedure();
        new_self
    }
    
    pub fn run(&self) -> AnyResult<i32> {
        self.wnd.run_main(None)
    }
    
    fn register_window_procedure(&self) {
        
    }
}