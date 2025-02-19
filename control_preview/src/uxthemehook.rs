//use crate::uxthemehook::{InitializeHooks};
use winsafe::WString;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn initialize_hooks() -> Result<(), &'static str> {
    unsafe {
        match ffi::InitializeHooks() {
            ffi::_INITRESULT_CPIR_SUCCEEDED => Ok(()),
            ffi::_INITRESULT_CPIR_FAILED => Err("Failed to initialize hooks."),
            ffi::_INITRESULT_CPIR_FAILED_MINHOOK_INIT => Err("Failed to initialize MinHook."),
            ffi::_INITRESULT_CPIR_FAILED_MINHOOK_HOOK => Err("Failed to install hook."),
            _ => Err("Failed to initialize hooks.")
        }
    }
}

pub fn load_global_theme(theme: &str) -> Option<()> {
    unsafe {
        return if ffi::LoadGlobalTheme(WString::from_str(theme).as_ptr()) >= 0 {
            // HRESULT succeeded
            Some(())
        }
        else {
            // HRESULT failed
            None
        }
    }
}