extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    // Create Cyclone DDS build directory and initial config
    let cyclonedds_dir = out_dir.join("cyclonedds-build");
    dir_builder.create(&cyclonedds_dir).unwrap();

    let mut cyclonedds = cmake::Config::new("cyclonedds");
    let mut cyclonedds = cyclonedds.out_dir(cyclonedds_dir);

    // Create initial bindings builder
    let mut bindings = bindgen::Builder::default();

    #[cfg(feature = "iceoryx")]
    {
        // Build iceoryx
        let iceoryx_dir = out_dir.join("iceoryx-build");
        dir_builder.create(&iceoryx_dir).unwrap();
        let mut iceoryx = cmake::Config::new("iceoryx/iceoryx_meta");

        // Force compilation of Iceoryx in release mode on Windows due to
        // https://github.com/rust-lang/rust/issues/39016
        #[cfg(all(debug_assertions, target_os = "windows"))]
        let iceoryx = iceoryx.profile("Release");

        let iceoryx = iceoryx
            .define("BUILD_SHARED_LIBS", "OFF")
            .out_dir(iceoryx_dir)
            .build();

        let iceoryx_lib = iceoryx.join("lib");
        let iceoryx_include = iceoryx.join("include/iceoryx/v2.0.3");

        // Add iceoryx lib to link
        println!("cargo:rustc-link-search=native={}", iceoryx_lib.display());
        println!("cargo:rustc-link-lib=static=iceoryx_binding_c");
        println!("cargo:rustc-link-lib=static=iceoryx_hoofs");
        println!("cargo:rustc-link-lib=static=iceoryx_posh");
        println!("cargo:rustc-link-lib=static=iceoryx_platform");

        let iceoryx_install_path = iceoryx.as_os_str();

        cyclonedds = cyclonedds
            .env("iceoryx_binding_c_DIR", iceoryx_install_path)
            .env("iceoryx_hoofs_DIR", iceoryx_install_path)
            .env("iceoryx_posh_DIR", iceoryx_install_path)
            .define("ENABLE_SHM", "YES");

        bindings = bindings
            .clang_arg(format!("-I{}", iceoryx_include.to_str().unwrap()))
            .clang_arg("-DDDS_HAS_SHM=1");

        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-lib=acl");

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        println!("cargo:rustc-link-lib=stdc++");

        #[cfg(any(target_os = "macos"))]
        println!("cargo:rustc-link-lib=c++");
    }

    // Finish configuration of cyclonedds build
    cyclonedds = cyclonedds
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_IDLC", "OFF")
        .define("BUILD_DDSPERF", "OFF")
        .define("ENABLE_LTO", "NO")
        .define("ENABLE_SSL", "NO")
        .define("ENABLE_SECURITY", "NO")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    // Force compilation of Cyclone DDS in release mode on Windows due to
    // https://github.com/rust-lang/rust/issues/39016
    #[cfg(all(debug_assertions, target_os = "windows"))]
    let cyclonedds = cyclonedds.profile("Release");

    // Build cyclonedds
    let cyclonedds = cyclonedds.build();

    let cyclonedds_include = cyclonedds.join("include");
    let cyclonedds_lib = cyclonedds.join("lib");

    // Add cyclonedds lib to link
    println!(
        "cargo:rustc-link-search=native={}",
        cyclonedds_lib.display()
    );
    println!("cargo:rustc-link-lib=static=ddsc");

    // Add Windows libraries required by Cyclone to link
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=Iphlpapi");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=DbgHelp");

    // Build cyclocut
    let cyclocut_dir = out_dir.join("cyclocut-build");
    dir_builder.create(&cyclocut_dir).unwrap();
    let mut cyclocut = cmake::Config::new("cyclocut");

    // Force compilation of Cyclocut in release mode on Windows due to
    // https://github.com/rust-lang/rust/issues/39016
    #[cfg(all(debug_assertions, target_os = "windows"))]
    let cyclocut = cyclocut.profile("Release");

    let cyclocut = cyclocut
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

    // Finish configuration of bindings build
    bindings = bindings
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cyclonedds_include.to_str().unwrap()))
        .clang_arg(format!("-I{}", cyclocut_include.to_str().unwrap()))
        .generate_comments(false);

    // Add *IMAGE_TLS_DIRECTORY* to blocklist on Windows due to
    // https://github.com/rust-lang/rust-bindgen/issues/2179
    #[cfg(target_os = "windows")]
    let bindings = bindings
        .clang_arg("-Wno-invalid-token-paste")
        .blocklist_type("^(.*IMAGE_TLS_DIRECTORY.*)$");

    // Generate bindings
    let bindings = bindings
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
