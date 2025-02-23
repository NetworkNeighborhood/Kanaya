use std::path::PathBuf;
use std::env;
use chrono;
use chrono::Datelike;
use kanaya_common::build_common as common;
use kanaya_common::extract_environment;

#[path = "build/build_restyle.rs"]
mod build_restyle;

extern crate winres;

extract_environment!(
    KANAYA_NAME_DISPLAY
    KANAYA_NAME_PROGRAMMATIC
    KANAYA_PUBLISHER_DISPLAY
    KANAYA_PUBLISHER_PROGRAMMATIC
);

const RESOURCE_SOURCE_FILE_PATH: &str = "resources/resource.rc";

fn main() {
    println!("cargo::rustc-link-lib=comctl32");
    println!("cargo::rustc-link-lib=uxtheme");
    
    if std::env::var("CARGO_FEATURE_RESTYLE_INTEGRATION").is_ok() {
        build_restyle::run_build().unwrap();
    }
    
    run_bindgen();
    
    compile_windows_resources();
    compile_windows_version_info();
}

fn run_bindgen() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings.");
    
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn compile_windows_resources() {
    //create_c_header_stubs();
    //let combined_resource_path = create_combined_resource_file();
    println!("cargo::rerun-if-changed={RESOURCE_SOURCE_FILE_PATH}");
    
    let mut res = winres::WindowsResource::new();
    res.set_output_file("res_main.lib");
    res.add_toolkit_include(true);
    res.set_resource_file(RESOURCE_SOURCE_FILE_PATH);
    
    if let Err(e) = res.compile() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    
    let output_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", output_dir);
    println!("cargo:rustc-link-lib=dylib=res_main");
}

// // Hack: C headers are expected to exist by the resource compiler, but their contents are entirely irrelevant.
// fn create_c_header_stubs() {
//     fs::write(env::var("OUT_DIR").unwrap() + "/stdarg.h", "").unwrap();
//     fs::write(env::var("OUT_DIR").unwrap() + "/string.h", "").unwrap();
//     fs::write(env::var("OUT_DIR").unwrap() + "/ctype.h", "").unwrap();
//     fs::write(env::var("OUT_DIR").unwrap() + "/excpt.h", "").unwrap();
// }

// Debug file flag.
const VS_FF_DEBUG: u64 = 1;

fn compile_windows_version_info() {
    //let debug_type = if env::var("CARGO_")
    
    let build_num = common::get_build_number();
    
    let mut res = winres::WindowsResource::new();
    res.set_output_file("res_versioninfo.lib");
    
    res.set_version_info(winres::VersionInfo::FILEFLAGS, if env::var("PROFILE").unwrap() == "debug" { VS_FF_DEBUG } else { 0 });
    
    res.set("Comments", "GUI editor for Microsoft Windows visual styles.");
    res.set("CompanyName", KANAYA_PUBLISHER_DISPLAY);
    res.set("FileDescription", &format!("{} Visual Style Editor", KANAYA_NAME_DISPLAY));
    res.set("ProductName", &format!("{} Visual Style Editor", KANAYA_NAME_DISPLAY));
    res.set("InternalName", &env::var("CARGO_PKG_NAME").unwrap());
    
    if env::var("KANAYA_PUBLISHER_DISPLAY").unwrap() == "Network Neighborhood" {
        // First-party (Network Neighborhood) build:
        res.set("LegalCopyright", &format!("(C) {build_year} Network Neighborhood", build_year = chrono::Utc::now().year()));
    }
    else {
        // Third-party build:
        res.set("LegalCopyright", &format!("(C) {build_year} {other_publisher} & Network Neighborhood",
            build_year = chrono::Utc::now().year(),
            other_publisher = &env::var("KANAYA_PUBLISHER_DISPLAY").unwrap()
        ));
    }
    
    res.set("OriginalFilename", &format!("{}.exe", &env::var("CARGO_PKG_NAME").unwrap()));
    
    let version_string = format!("{}.{}", &env::var("CARGO_PKG_VERSION").unwrap(), build_num);
    res.set("FileVersion", &version_string);
    res.set("ProductVersion", &version_string);
    
    let mut version = 0_u64;
    version |= env::var("CARGO_PKG_VERSION_MAJOR")
        .unwrap()
        .parse()
        .unwrap_or(0)
        << 48;
    version |= env::var("CARGO_PKG_VERSION_MINOR")
        .unwrap()
        .parse()
        .unwrap_or(0)
        << 32;
    version |= env::var("CARGO_PKG_VERSION_PATCH")
        .unwrap()
        .parse()
        .unwrap_or(0)
        << 16;
    version |= build_num as u64;
    
    res.set_version_info(winres::VersionInfo::FILEVERSION, version);
    res.set_version_info(winres::VersionInfo::PRODUCTVERSION, version);

    if let Err(e) = res.compile() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    
    let output_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", output_dir);
    println!("cargo:rustc-link-lib=dylib=res_versioninfo");
}