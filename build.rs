extern crate bindgen;

use cmake;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    // Build iceoryx
    let iceoryx_dir = out_dir.join("iceoryx-build");
    dir_builder.create(&iceoryx_dir).unwrap();
    let iceoryx = cmake::Config::new("iceoryx/iceoryx_meta")
        .define("BUILD_SHARED_LIBS", "OFF")
        .out_dir(iceoryx_dir)
        .build();

    let iceoryx_lib = iceoryx.join("lib");

    // Add iceoryx lib to link
    println!("cargo:rustc-link-search=native={}", iceoryx_lib.display());
    println!("cargo:rustc-link-lib=static=iceoryx_binding_c");
    println!("cargo:rustc-link-lib=static=iceoryx_hoofs");
    println!("cargo:rustc-link-lib=static=iceoryx_posh");
    println!("cargo:rustc-link-lib=static=iceoryx_platform");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=acl");

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(any(target_os = "macos"))]
    println!("cargo:rustc-link-lib=c++");

    let iceoryx_install_path = iceoryx.as_os_str();

    // Build cyclonedds
    let cyclonedds_dir = out_dir.join("cyclonedds-build");
    dir_builder.create(&cyclonedds_dir).unwrap();
    let cyclonedds = cmake::Config::new("cyclonedds")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_IDLC", "OFF")
        .define("BUILD_DDSPERF", "OFF")
        .define("ENABLE_LTO", "NO")
        .define("ENABLE_SHM", "ON")
        .define("ENABLE_SSL", "NO")
        .define("ENABLE_SECURITY", "NO")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .env("iceoryx_binding_c_DIR", iceoryx_install_path)
        .env("iceoryx_hoofs_DIR", iceoryx_install_path)
        .env("iceoryx_posh_DIR", iceoryx_install_path)
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
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
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
