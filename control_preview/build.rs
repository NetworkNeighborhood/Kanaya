use std::fs;
use std::env;
use std::path::PathBuf;
use chrono;
use chrono::Datelike;
use kanaya_common::build_common as common;
use kanaya_common::extract_environment;

extern crate winres;

extract_environment!(
    KANAYA_NAME_DISPLAY
    KANAYA_NAME_PROGRAMMATIC
    KANAYA_PUBLISHER_DISPLAY
    KANAYA_PUBLISHER_PROGRAMMATIC
);

fn main() {
    println!("cargo::rustc-link-lib=comctl32");
    println!("cargo::rustc-link-lib=uxtheme");
    
    build_c();
    run_bindgen();
    
    compile_windows_version_info();
}

fn build_c() {
    cc::Build::new()
        .file("src/hook.c")
        .file("minhook/src/buffer.c")
        .file("minhook/src/hook.c")
        .file("minhook/src/trampoline.c")
        .file("minhook/src/hde/hde64.c")
        .compile("ctrlprevc");
    
    println!("cargo::rustc-link-lib=ctrlprevc");
}

fn run_bindgen() {
    let bindings = bindgen::Builder::default()
        .header("src/control_preview.h")
        .allowlist_file("src/control_preview.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings.");
    
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// Debug file flag.
const VS_FF_DEBUG: u64 = 1;

fn compile_windows_version_info() {
    //let debug_type = if env::var("CARGO_")
    
    let build_num = common::get_build_number();
    
    let mut res = winres::WindowsResource::new();
    res.set_output_file("res_versioninfo.lib");
    
    res.set_version_info(winres::VersionInfo::FILEFLAGS, if env::var("PROFILE").unwrap() == "debug" { VS_FF_DEBUG } else { 0 });
    
    res.set("Comments", "Previews user controls with a given msstyles file path.");
    res.set("CompanyName", KANAYA_PUBLISHER_DISPLAY);
    res.set("FileDescription", &format!("{} Control Preview", KANAYA_NAME_DISPLAY));
    res.set("ProductName", &format!("{} Control Preview", KANAYA_NAME_DISPLAY));
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
    
    // CREATEPROCESS_MANIFEST_RESOURCE_ID = 1 RT_MANIFEST = 24
    res.append_rc_content("\n1 24 \"control_preview_manifest.xml\"\n\n");
    create_manifest().expect("Failed to create manifest.");

    if let Err(e) = res.compile() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    
    let output_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", output_dir);
    println!("cargo:rustc-link-lib=dylib=res_versioninfo");
}

fn create_manifest() -> Option<()> {
    let output_file = env::var("OUT_DIR").unwrap() + "/control_preview_manifest.xml";
    
    let contents = format!(r##"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<assemblyIdentity
    version="1.0.0.0"
    processorArchitecture="*"
    name="{publisher}.{product}.ControlPreview"
    type="win32"
/>
<description>Previews user controls with a given msstyles file path.</description>
<dependency>
    <dependentAssembly>
        <assemblyIdentity
            type="win32"
            name="Microsoft.Windows.Common-Controls"
            version="6.0.0.0"
            processorArchitecture="*"
            publicKeyToken="6595b64144ccf1df"
            language="*"
        />
    </dependentAssembly>
</dependency>
</assembly>"##, product = KANAYA_NAME_PROGRAMMATIC, publisher = KANAYA_PUBLISHER_PROGRAMMATIC);

    fs::write(output_file, contents).ok()
}