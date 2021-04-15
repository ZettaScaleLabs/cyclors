extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    // Build cyclonedds
    let cyclonedds_dir = out_dir.join("cyclonedds-build");
    dir_builder.create(&cyclonedds_dir).unwrap();
    let cyclonedds = cmake::Config::new("cyclonedds")
        .define("BUILD_SHARED_LIBS", "false")
        .define("BUILD_IDLC", "off")
        .out_dir(cyclonedds_dir)
        .build();
    let cyclonedds_include = cyclonedds.join("include");
    let cyclonedds_lib = cyclonedds.join("lib");

    // Add cyclonedds lib to link
    println!(
        "cargo:rustc-link-search=native={}",
        cyclonedds_lib.display()
    );
    println!("cargo:rustc-link-lib=static=ddsc");

    // Build cyclocut
    let cyclocut_dir = out_dir.join("cyclocut-build");
    dir_builder.create(&cyclocut_dir).unwrap();
    let cyclocut = cmake::Config::new("cyclocut")
        .env("CYCLONE_INCLUDE", &cyclonedds_include)
        .env("CYCLONE_LIB", &cyclonedds_lib)
        .define("CYCLONE_INCLUDE", cyclonedds_include.clone())
        .define("CYCLONE_LIB", cyclonedds_lib.clone())
        .define("BUILD_CDDS_UTIL_EXAMPLES", "off")
        .define("BUILD_SHARED_LIBS", "false")
        .out_dir(cyclocut_dir)
        .build();
    let cyclocut_include = cyclocut.join("include");
    let cyclocut_lib = cyclocut.join("lib");

    // Add cyclocut lib to link
    println!("cargo:rustc-link-search=native={}", cyclocut_lib.display());
    println!("cargo:rustc-link-lib=static=cdds-util");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cyclonedds_include.to_str().unwrap()))
        .clang_arg(format!("-I{}", cyclocut_include.to_str().unwrap()))
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
