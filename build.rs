use std::env;
use std::fs;
use std::path::{Path, PathBuf};
/* 

this code has one main purpose, during the build process, we have a HTML source tree that needs to be in the root directory of the binary, this automatically handles that copying on each build,
it also creates a copy in the top level folder of the project, as while debugging it will be looking there for files

*/
fn main() {
    // Get the current directory where the build script is running
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Determine the target directory (where the executable will be built)
    let target_dir_str = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(&target_dir_str);
    let par_dir_0 = target_dir
        .parent()
        .expect("Failed to get parent directory of OUT_DIR");
    let par_dir_1 = par_dir_0
        .parent()
        .expect("Failed to get parent directory of OUT_DIR's Parent");
    let par_dir_2 = par_dir_1
        .parent()
        .expect("Failed to trace back to target folder for the final executable");
    // Define the source directory (where html_src is located)
    let source_dir = Path::new(&current_dir).join("src").join("html_src");

    // Define the destination directory (inside target/debug or target/release)
    let dest_dir = par_dir_2.join("webcontent");

    // Check if the destination directory exists, and if it does, delete it
    if dest_dir.exists() {
        if let Err(err) = fs::remove_dir_all(&dest_dir) {
            eprintln!("Failed to delete existing destination directory: {}", err);
            std::process::exit(1);
        }
    }

    // Create the destination directory
    fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;
    // Copy the contents of the source directory to the destination directory
    if let Err(err) = fs_extra::dir::copy(&source_dir, &dest_dir, &options) {
        eprintln!("Error copying files to target: {}", err);
        std::process::exit(1);
    }

    let target_folder = par_dir_2.parent().expect("failed to get target dir");
    let top_folder = target_folder
        .parent()
        .expect("failed to get back to the top dir");
    let dest_dir = top_folder.join("webcontent");
    if dest_dir.exists() {
        if let Err(err) = fs::remove_dir_all(&dest_dir) {
            eprintln!("Failed to delete existing destination directory: {}", err);
            std::process::exit(1);
        }
    } else {
        fs::create_dir_all(&dest_dir).expect("Failed to create destination directory");
    }
    if let Err(err) = fs_extra::dir::copy(&source_dir, &dest_dir, &options) {
        eprintln!("Error copying files to top folder: {}", err);
        std::process::exit(1);
    }
}

