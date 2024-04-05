use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use pkg_config::find_library;

fn main() {
    // Clone submodules if that hasn't already happened.
    if !Path::new("vendor/.git").exists() || !Path::new("vendor/hidapi/.git").exists() {
        let _ = Command::new("git")
            .args(["submodule", "update", "--init", "--recursive"])
            .status();
    }

    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate bindings.
    let bindings = bindgen::Builder::default()
        .header("vendor/src/wooting-rgb-sdk.h")
        // .header("vendor/src/wooting-usb.h")
        .generate()
        .expect("Unable to generate bindings for the Wooting RGB SDK");
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Unable to write Wooting RGB SDK bindings");

    // Build hidapi to link against.
    let mut cfg = cc::Build::new();
    cfg.warnings(false)
        .extra_warnings(false)
        .include("vendor/hidapi/hidapi");

    if target.contains("linux") {
        let lib = find_library("hidapi-hidraw").expect("Unable to find hidapi-hidraw");
        for path in lib.include_paths {
            cfg.include(path.to_str().unwrap());
        }
    } else if target.contains("windows") {
        cfg.file("vendor/hidapi/windows/hid.c");
        println!("cargo:rustc-link-lib=setupapi");
    } else if target.contains("apple") {
        cfg.file("vendor/hidapi/mac/hid.c");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    } else {
        panic!("Unsupported target for wooting-rgb-sys");
    };

    // Build SDK to link against.
    cfg.file("vendor/src/wooting-rgb-sdk.c")
        .file("vendor/src/wooting-usb.c")
        .compile("wooting-rgb-sdk");
}
