//use crate::uxthemehook::{InitializeHooks};
use winsafe::WString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn initialize_hooks() -> Result<(), String> {
    unsafe {
        match InitializeHooks() {
            _INITRESULT_CPIR_SUCCEEDED => Ok(()),
            _INITRESULT_CPIR_FAILED => Err("Failed to initialize hooks.".to_owned()),
            _INITRESULT_CPIR_FAILED_MINHOOK_INIT => Err("Failed to initialize MinHook.".to_owned()),
            _INITRESULT_CPIR_FAILED_MINHOOK_HOOK => Err("Failed to install hook.".to_owned()),
            _ => Err("Failed to initialize hooks.".to_owned())
        }
    }
}

pub fn load_global_theme(theme: &str) -> Option<()> {
    unsafe {        
        return if LoadGlobalTheme(WString::from_str(theme).as_ptr()) >= 0 {
            // HRESULT succeeded
            Some(())
        }
        else {
            // HRESULT failed
            None
        }
    }
}