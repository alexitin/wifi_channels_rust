use std::process::Command;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftTargetInfo {
//    triple: String,
    unversioned_triple: String,
//    module_triple: String,
//    swift_runtime_compatibility_version: String,
    #[serde(rename = "librariesRequireRPath")]
    libraries_require_rpath: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SwiftPaths {
    runtime_library_paths: Vec<String>,
//    runtime_library_import_paths: Vec<String>,
//    runtime_resource_path: String,
}

#[derive(Deserialize)]
struct SwiftTarget {
    target: SwiftTargetInfo,
    paths: SwiftPaths,
}

fn build_mac_selector() {
    let profile = env::var("PROFILE").unwrap();
//    let target = format!("{}-apple-macosx", MACBOOK_ARCH);

    let swift_target_info_str = Command::new("swift")
        .args(&["-print-target-info"])
        .output()
        .unwrap()
        .stdout;
    
    let swift_target_info: SwiftTarget = serde_json::from_slice(&swift_target_info_str).unwrap();
    if swift_target_info.target.libraries_require_rpath {
        panic!("Libraries require RPath! Change minimum MacOS value to fix.")
    }

    if !Command::new("swift")
    .args(&["build", "-c", &profile])
    .current_dir("./mac_select_channel")
    .status()
    .unwrap()
    .success()
    {
        panic!("Swift library mac_select_channel compilation failed.")
    }

    swift_target_info.paths.runtime_library_paths.iter().for_each(|path| {
        println!("cargo:rustc-link-search=native={}", path);
    });

    println!("cargo:rustc-link-search=native=./mac_select_channel/.build/{}/{}/",
        swift_target_info.target.unversioned_triple, profile
    );

    println!("cargo:rustc-link-lib=static=mac_select_channel");

    println!("cargo:rerun-if-changed=mac_select_channel/src/*.swift");

}

fn build_lin_selector() {
    cc::Build::new()
        .file("./lin_select_channel/lin_select_channel.c")
        .compile("lin_select_channel");
}

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        build_mac_selector();
    }
    if target_os == "linux" {
        build_lin_selector();
    }
}
