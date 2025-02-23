/*
 * Builds with MSVC via MSBuild:
 * - rstylkan.dll/librestyle_kanaya (Kanaya Restyle Core Library)
 * - restyle.exe (Kanaya Restyle Stub Executable - Restyle)
 * 
 * The build outputs are moved to be in the same directory as the Rust build artifacts. As you would
 * imagine, Visual Studio is a requirement to build.
 * 
 * This is a little bit abusive of Cargo.
 */

use kanaya_common::auto_length_arr;
use const_format::formatcp;
use std::fs;
use std::path::*;
use std::str::FromStr;
use std::sync::OnceLock;
use anyhow::Error;
use anyhow::anyhow;
use anyhow::Context;

const RESTYLE_ROOT: &str = "../restyle";
const RESTYLE_STATIC_SRC_PATH: &str = formatcp!("{}/src", RESTYLE_ROOT);
const RESTYLE_STATIC_SRCPATCHES_PATH: &str = formatcp!("{}/srcpatches", RESTYLE_ROOT);
const RESTYLE_STATIC_EXTRA_PATH: &str = formatcp!("{}/kanayacpp", RESTYLE_ROOT);
const RESTYLE_STATIC_STUB_PATH: &str = formatcp!("{}/stub", RESTYLE_ROOT);
const BUILD_FOLDER_NAME: &str = "restyle_build";
const BUILD_SRC_PATH: &str = "src";
const BUILD_EXTRA_PATH: &str = "kanayacpp";
const BUILD_STUB_PATH: &str = "stub";

// Network Neighborhood builds are limited to the classic Windows limitation of 8.3
// file names, so we use "rstylkan" instead of "restyle_kanaya". This just looks better
// for Windows binaries in my opinion.
const RESTYLE_KANAYA_BIN_NAME: &str = "rstylkan";                                  // i.e. rstylkan
const RESTYLE_KANAYA_BIN_DLL: &str = formatcp!("{}.dll", RESTYLE_KANAYA_BIN_NAME); // i.e. rstylkan.dll
const RESTYLE_KANAYA_BIN_LIB: &str = formatcp!("{}.lib", RESTYLE_KANAYA_BIN_NAME); // i.e. rstylkan.lib
const RESTYLE_KANAYA_BIN_PDB: &str = formatcp!("{}.pdb", RESTYLE_KANAYA_BIN_NAME); // i.e. rstylkan.pdb

const RESTYLE_STUB_BIN_NAME: &str = "restyle";                                 // i.e. restyle
const RESTYLE_STUB_BIN_EXE: &str = formatcp!("{}.exe", RESTYLE_STUB_BIN_NAME); // i.e. restyle.exe
const RESTYLE_STUB_BIN_PDB: &str = formatcp!("{}.pdb", RESTYLE_STUB_BIN_NAME); // i.e. restyle.pdb

// Path basenames which we should ignore because they're too big and too useless.
auto_length_arr!(const IGNORE_PATHS: [&str; _] = [
    ".vs",
    ".git",
]);

struct RestyleFileScanResult {
    needs_msbuild_rerun: bool,
}

impl Default for RestyleFileScanResult {
    fn default() -> Self {
        Self {
            needs_msbuild_rerun: false,
        }
    }
}

macro_rules! ensure_path_existences {
    ($($path: expr), * $(,)?) => {
        $(
            let path = $path;
            if !path.exists() {
                eprintln!("Path \"{}\" does not exist. Creating...", path.display());
                fs::create_dir(path)
                    .with_context(|| anyhow!("Failed to create path \"{}\"", path.display()))?;
            }
        )
        *
    }
}

// Should only be ran if "restyle-integration" feature flag is set.
pub fn run_build() -> Result<(), Error> {
    println!("cargo::warning=Hi :33");
    let build_dir = get_build_directory()?;
    
    if !Path::new(&build_dir).exists() {
        fs::create_dir(&build_dir)?;
    }
    
    ensure_path_existences!(
        get_build_src_path().with_context(|| anyhow!("Failed to get build source path (expected {}).", RESTYLE_STATIC_SRC_PATH))?,
        get_build_extra_path().with_context(|| anyhow!("Failed to get build extra path (expected {}).", RESTYLE_STATIC_EXTRA_PATH))?,
        get_build_stub_path().with_context(|| anyhow!("Failed to get build stub path (expected {}).", RESTYLE_STATIC_STUB_PATH))?,
    );
    
    let iteration_result = iterate_files()?;
    
    if iteration_result.needs_msbuild_rerun {
        build_restyle_vcxproj().or_else(|e| Err(anyhow!("Failed to build Restyle vcxproj: {}", e)))?;
        build_restyle_stub().or_else(|e| Err(anyhow!("Failed to build Restyle stub: {}", e)))?;
    }
    
    Ok(())
}

#[derive(Clone)]
enum FileSource {
    StaticSource,
    SourcePatches,
    ExtraSource,
}

fn is_ignored_path(path: &Path) -> Result<bool, Error> {
    Ok(IGNORE_PATHS.contains(
        &path.file_name()
        .ok_or_else(|| anyhow!("Failed to get path file name."))?
        .to_str()
        .ok_or_else(|| anyhow!("Failed to get path file name as string."))?
        .to_lowercase()
        .as_str()
    ))
}

/**
 * Unwrap a statically-stored error.
 */
macro_rules! unwrap_static_error {
    ($x: expr) => {
        match $x {
            Ok(ref result) => Ok(result),
            Err(e) => Err(anyhow!(e)),
        }
    }
}

fn get_root_folder() -> Result<&'static PathBuf, Error> {
    static RESULT: OnceLock<Result<PathBuf, Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        match std::env::current_dir()?.parent() {
            Some(parent_dir) => Ok(parent_dir.to_path_buf()),
            None => Err(anyhow!("Failed to get path."))
        }
    }))
}

fn get_absolute_folder(str: &str) -> Result<PathBuf, Error> {
    let result = std::path::absolute(&Path::new(&std::env::current_dir()?).join(str))?;
    eprintln!("get_absolute_folder: {}", result.display());
    Ok(result)
}

fn get_restyle_folder() -> Result<&'static PathBuf, Error> {
    static RESULT: OnceLock<Result<PathBuf,Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        get_absolute_folder(RESTYLE_ROOT)
    }))
}

fn get_build_directory() -> Result<&'static PathBuf, Error> {
    //println!("cargo::warning=build dir: {}", format!("{}/{}", std::env::var("OUT_DIR")?, BUILD_FOLDER_NAME));
    static RESULT: OnceLock<Result<PathBuf, Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        Ok(Path::new(std::env::var("OUT_DIR")?.as_str()).join(BUILD_FOLDER_NAME))
    }))
}

fn get_build_src_path() -> Result<&'static PathBuf, Error> {
    static RESULT: OnceLock<Result<PathBuf, Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        Ok(get_build_directory()?.join(BUILD_SRC_PATH))
    }))
}

fn get_build_extra_path() -> Result<&'static PathBuf, Error> {
    static RESULT: OnceLock<Result<PathBuf, Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        Ok(get_build_directory()?.join(BUILD_EXTRA_PATH))
    }))
}

fn get_build_stub_path() -> Result<&'static PathBuf, Error> {
    static RESULT: OnceLock<Result<PathBuf, Error>> = OnceLock::new();
    unwrap_static_error!(RESULT.get_or_init(|| {
        Ok(get_build_directory()?.join(BUILD_STUB_PATH))
    }))
}

fn iterate_files() -> Result<RestyleFileScanResult, Error> {
    let mut result = RestyleFileScanResult::default();
    let mut propagate_results = |child_result: RestyleFileScanResult| {
        if child_result.needs_msbuild_rerun {
            result.needs_msbuild_rerun = true;
        }
        Ok(())
    };
    
    // Stub:
    let dir = get_absolute_folder(&RESTYLE_STATIC_STUB_PATH)?;
    let dir: &Path = dir.as_path();
    iterate_all_files_in_path(dir, FileSource::StaticSource).and_then(&mut propagate_results)?;
    
    // Main Restyle source code:
    let dir = get_absolute_folder(&RESTYLE_STATIC_SRC_PATH)?;
    let dir: &Path = dir.as_path();
    iterate_all_files_in_path(dir, FileSource::StaticSource).and_then(&mut propagate_results)?;
    
    // Install patches:
    let dir = get_absolute_folder(&RESTYLE_STATIC_SRCPATCHES_PATH)?;
    let dir: &Path = dir.as_path();
    iterate_all_files_in_path(dir, FileSource::SourcePatches).and_then(&mut propagate_results)?;
    
    // Kanaya extensions:
    let dir = get_absolute_folder(&RESTYLE_STATIC_EXTRA_PATH)?;
    let dir: &Path = dir.as_path();
    iterate_all_files_in_path(dir, FileSource::ExtraSource).and_then(&mut propagate_results)?;
    
    Ok(result)
}

fn iterate_all_files_in_path(path: &Path, source_type: FileSource) -> Result<RestyleFileScanResult, Error> {
    println!("cargo::warning=iterating: {}", path.to_str().ok_or_else(|| anyhow!("Failed to get path."))?);
    
    let mut result = RestyleFileScanResult::default();
    let mut propagate_results = |child_result: RestyleFileScanResult| {
        if child_result.needs_msbuild_rerun {
            result.needs_msbuild_rerun = true;
        }
        Ok(())
    };
    
    return if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry_path.is_dir() && !is_ignored_path(&entry_path)? {
                iterate_all_files_in_path(&entry_path, source_type.clone())?;
            }
            else {
                handle_file(&entry_path, source_type.clone()).and_then(&mut propagate_results)?;
            }
        }
        
        Ok(result)
    }
    else if path.is_file() {
        handle_file(path, source_type.clone()).and_then(&mut propagate_results)?;
        Ok(result)
    }
    else {
        Err(anyhow!("Input path is invalid."))
    }
}

fn handle_file(src_path: &Path, source_type: FileSource) -> Result<RestyleFileScanResult, Error> {
    let mut result = RestyleFileScanResult::default();
    
    let build_dir = get_build_directory()?;
    
    if is_ignored_path(&src_path)? {
        return Ok(result);
    }
    
    let restyle_folder = get_restyle_folder().or_else(|e| Err(anyhow!("Failed to get Restyle folder: {}", e)))?;
    let base_path = src_path.strip_prefix(&restyle_folder)?;
    let dest_path = build_dir.join(&base_path);
    
    println!("cargo::warning=Source path: {}", src_path.to_str().ok_or_else(|| anyhow!("Failed to get name as string."))?);
    println!("cargo::warning=Base path: {}", base_path.to_str().ok_or_else(|| anyhow!("Failed to get name as string."))?);
    println!("cargo::warning=Destination path: {}", dest_path.to_str().ok_or_else(|| anyhow!("Failed to get name as string."))?);
    
    println!("cargo::rerun-if-changed={}", src_path.display());
    
    if matches!(source_type, FileSource::StaticSource | FileSource::ExtraSource) {
        if dest_path.exists() && dest_path.metadata()?.modified()? == src_path.metadata()?.modified()? {
            // This file is up to date.
            println!("cargo::warning=File up to date.");
        }
        else {
            // The file is out of date or does not exist.
            fs::copy(&src_path, &dest_path)?;
            
            // To ensure our cache system works correctly, touch the file with the last-modified date
            // of the original:
            let file = fs::OpenOptions::new().write(true).open(dest_path)?;
            file.set_modified(src_path.metadata()?.modified()?)?;
            
            println!("cargo::warning=File out of date, rebuilt.");
            
            // Since some file changed, schedule an MSBuild rebuild.
            result.needs_msbuild_rerun = true;
        }
    }
    else if matches!(source_type, FileSource::SourcePatches) {
        let patch_file_path = build_dir.join(&find_patch_file_target_relative_path(src_path).ok_or_else(|| anyhow!("Failed to find patch file target relative path."))?);
        println!("cargo::warning=Patch file: {}", patch_file_path.display());
        patch_file(&patch_file_path, &src_path).expect("Failed to patch file");
    }
    
    Ok(result)
}

fn find_patch_file_target_relative_path(diff_file_path: &Path) -> Option<String> {
    let restyle_folder = get_absolute_folder(RESTYLE_STATIC_SRCPATCHES_PATH).ok()?;
    Some("src/".to_owned() + diff_file_path.strip_prefix(&restyle_folder).ok()?.to_str()?.strip_suffix(".diff")?)
}

// Hell.
fn patch_file(dest_path: &Path, diff_file_path: &Path) -> Result<(), Error> {
    let original_lmt = dest_path.metadata()?.modified()?;
    
    std::process::Command::new("git")
        .current_dir("C:\\") // Current directory must not be in a Git project or it breaks. TODO: Ensure the path is valid; C:\ CAN be invalid on Windows.
        .arg("apply")
        .arg(diff_file_path)
        .arg("--directory")
        .arg(dest_path.parent().ok_or_else(|| anyhow!("Failed to get the patch root folder."))?)
        .arg("--unsafe-paths")
        .arg("--verbose")
        .arg("--no-index")
        .spawn()
        .or_else(|e| Err(anyhow!("Failed to spawn Git process for diffing: {}", e)))?
        .wait_with_output()?;
    
    // To ensure that the modified file does not break the build cache, touch it now:
    let file = fs::OpenOptions::new().write(true).open(dest_path)?;
    file.set_modified(original_lmt)?;
    
    Ok(())
}

/**
 * Moves a file from a source path to a destination path.
 * 
 * Unlike the standard API fs::rename, this will also work across drives.
 */
fn move_file(src_path: &Path, dest_path: &Path) -> Result<(), Error> {
    // First, try fs::rename, as it works for moving files on the same drive.
    Ok(fs::rename(src_path, dest_path).or_else(|_| -> Result<(), Error> {
        // If that failed, then copy the file and remove the original.
        fs::copy(src_path, dest_path)?;
        fs::remove_file(src_path)?;
        Ok(())
    })?)
}

/**
 * Finds the location of the vswhere.exe Visual Studio binary.
 * 
 * This aids in finding the location of MSVC build tools.
 */
fn find_vswhere_executable() -> Result<String, Error> {
    // These are fixed paths since VS 2017.
    // https://github.com/Microsoft/vswhere/wiki/Installing
    const VSWHERE_PATHS: &[&str] = &[
        "%ProgramFiles(x86)%\\Microsoft Visual Studio\\Installer\\vswhere.exe",
        
        // Chocolatey, which is some package manager I've never used before, apparently
        // can install it to its own location, despite Microsoft's guidance. I will still
        // account for this.
        // https://stackoverflow.com/a/56677470
        "%ProgramData%\\chocolatey\\lib\\vswhere\\tools\\vswhere.exe",
    ];
    
    for &path in VSWHERE_PATHS {
        let path = match winsafe::ExpandEnvironmentStrings(&path) {
            Ok(result) => result,
            Err(_err) => return Err(anyhow!("Failed to expand environment strings.")),
        };
        
        if !fs::exists(&path)? {
            continue;
        }
        
        return Ok(path);
    }
    
    Err(anyhow!("Failed to find vswhere executable."))
}

fn find_visual_studio_binaries_path() -> Result<PathBuf, Error> {
    let vswhere_path = find_vswhere_executable()?;
    eprintln!("VSWhere path: {}", &vswhere_path);
    
    let output = match std::process::Command::new(&vswhere_path)
        .stdout(std::process::Stdio::piped())
        .args(["-legacy", "-prerelease", "-utf8", "-format", "json"])
        .output()
        {
            Ok(result) => result,
            Err(_err) => return Err(anyhow!("Failed to run vswhere.")),
        };
        
    eprintln!("stdout info: {}", output.stdout.len());
    
    let json_string = String::from_utf8(output.stdout)?;
    
    eprintln!("JSON output: {}", json_string);
    
    let json_obj: serde_json::Value = serde_json::from_str(&json_string)?;
    
    for item in json_obj.as_array().ok_or_else(|| anyhow!("Failed to get JSON object."))?.iter() {
        let catalog = item.get("catalog").ok_or_else(|| anyhow!("Failed to get catalog value."))?;
        let _version = catalog.get("productLineVersion").ok_or_else(|| anyhow!("Failed to get productLineVersion value."))?;
        let path = item.get("installationPath").ok_or_else(|| anyhow!("Failed to get installationPath value."))?;
        let pb: PathBuf = PathBuf::from(path.as_str().ok_or_else(|| anyhow!("Failed to get path as string."))?);
        
        return Ok(pb);
    }
    
    Err(anyhow!("Failed to find Visual Studio binaries path."))
}

/**
 * Find the path to the MSBuild executable.
 */
fn find_msbuild_path() -> Result<PathBuf, Error> {
    let mut path = find_visual_studio_binaries_path()?.join("MSBuild");
    
    for entry in fs::read_dir(&path)? {
        let name = entry?.file_name();
        let name = name.to_str().ok_or_else(|| anyhow!("Failed to get name as string."))?;
        
        let version_folder_name: Result<f32, std::num::ParseFloatError> = name.parse();
        if version_folder_name.is_ok() {
            path = path.join(name);
        }
        
        if name == "Current" {
            path = path.join(name);
        }
    }
    
    let pb: PathBuf = path.join("Bin").join("MSBuild.exe");
    Ok(pb)
}

/**
 * Map the Cargo profile to a convential MSBuild configuration.
 */
fn get_msbuild_configuration() -> Result<&'static str, Error> {
    match std::env::var("PROFILE").ok() {
        Some(result) => {
            if result == "debug" {
                Ok("Debug")
            }
            else {
                Ok("Release")
            }
        }
        
        None => Err(anyhow!("Failed to get build profile."))
    }
}

/**
 * Map the Cargo build architecture (from its target triple) to a convential MSBuild platform name.
 */
fn get_msbuild_platform() -> Result<&'static str, Error> {
    use target_lexicon::Architecture::*;
    match target_lexicon::Triple::from_str(&std::env::var("TARGET")?).or_else(|e| Err(anyhow!(e)))?.architecture {
        X86_64 => Ok("x64"),
        X86_32(_) => Ok("Win32"),
        Aarch64(_) => Ok("ARM64"),
        // ARM32 Windows has been deprecated.
        
        _ => Err(anyhow!("Unsupported target architecture."))
    }
}

struct MsbuildResult {
    msvc_artifacts_path: PathBuf
}

fn build_vcxproj(project_path: &Path) -> Result<MsbuildResult, Error> {
    let msbuild_path = find_msbuild_path()?;
    
    let configuration = get_msbuild_configuration()?;
    let platform = get_msbuild_platform()?;
    
    let command_result = std::process::Command::new(&msbuild_path)
        .current_dir(project_path)
        .arg(format!("/p:Configuration={}", configuration))
        .arg(format!("/p:Platform={}", platform))
        .spawn()
        .with_context(|| format!("Failed to spawn MSBuild process for project {}", project_path.display()))?
        .wait();
    
    if command_result.is_err() {
        return Err(anyhow!("Failed to run MSBuild to build project {}", project_path.display()));
    }
    
    Ok(MsbuildResult {
        // This is standard, but not guaranteed for all project configurations. Restyle
        // follows this pattern, so it's safe to make this assumption in our case.
        msvc_artifacts_path: project_path.join(&platform).join(&configuration)
    })
}

macro_rules! move_msbuild_results {
    () => {};
    
    ($($src_path: expr => $dest_path: expr),* $(,)?) => {
        $($src_path.exists()
            .then(|| {
                move_file(&$src_path, &$dest_path).ok()
            })
            .ok_or_else(|| anyhow!("Could not find MSBuild artifact file \"{}\"", $src_path.display()))?;)
        *
    };
}

fn get_rust_artifacts_path() -> Result<PathBuf, Error> {
    Ok(get_root_folder()?.join("target").join(&std::env::var("PROFILE")?).to_path_buf())
}

fn build_restyle_vcxproj() -> Result<(), Error> {
    let msbuild_result = build_vcxproj(&get_build_directory()?.join(BUILD_SRC_PATH))?;
    
    let out_dir = &std::env::var("OUT_DIR")?;
    let out_dir = Path::new(out_dir);
    let final_artifacts_path = get_rust_artifacts_path()?;
    move_msbuild_results!(
        msbuild_result.msvc_artifacts_path.join(RESTYLE_KANAYA_BIN_LIB) => out_dir.join(RESTYLE_KANAYA_BIN_LIB),
        msbuild_result.msvc_artifacts_path.join(RESTYLE_KANAYA_BIN_DLL) => final_artifacts_path.join(RESTYLE_KANAYA_BIN_DLL),
        msbuild_result.msvc_artifacts_path.join(RESTYLE_KANAYA_BIN_PDB) => final_artifacts_path.join(RESTYLE_KANAYA_BIN_PDB),
    );
    
    Ok(())
}

fn build_restyle_stub() -> Result<(), Error> {
    let msbuild_result = build_vcxproj(&get_build_directory()?.join(BUILD_STUB_PATH))?;
    
    let final_artifacts_path = get_rust_artifacts_path()?;
    move_msbuild_results!(
        msbuild_result.msvc_artifacts_path.join(RESTYLE_STUB_BIN_EXE) => final_artifacts_path.join(RESTYLE_STUB_BIN_EXE),
        msbuild_result.msvc_artifacts_path.join(RESTYLE_STUB_BIN_PDB) => final_artifacts_path.join(RESTYLE_STUB_BIN_PDB)
    );
    
    Ok(())
}