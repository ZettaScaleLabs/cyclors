extern crate bindgen;

#[allow(unused_imports)]
use std::collections::HashSet;
use std::fs::File;
#[allow(unused_imports)]
use std::io::{LineWriter, Write};
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::process::Command;
use std::{env, fs};

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

        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-lib=c++");
    }
    #[cfg(not(feature = "iceoryx"))]
    {
        cyclonedds = cyclonedds.define("ENABLE_SHM", "NO");
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

    #[allow(unused)]
    let mut prefix = String::from("");

    // Prefix symbols in Cyclone DDS and Cyclocut libraries to ensure uniqueness
    #[cfg(all(target_os = "linux", not(feature = "iceoryx")))]
    {
        // Prefix = cyclors_<version>_
        prefix = env::var("CARGO_PKG_VERSION").unwrap().replace(".", "_");
        prefix.insert_str(0, "cyclors_");
        prefix.push('_');

        let mut symbols = HashSet::new();

        let cyclone_symbols = get_defined_symbols(&cyclonedds_lib, "libddsc.a")
            .expect("Failed to get symbols from libddsc.a!");
        symbols.extend(cyclone_symbols);
        prefix_symbols(&cyclonedds_lib, "libddsc.a", &prefix, &symbols).unwrap();

        let cyclocut_symbols = get_defined_symbols(&cyclocut_lib, "libcdds-util.a")
            .expect("Failed to get symbols from libcdds-util.a!");
        symbols.extend(cyclocut_symbols);
        prefix_symbols(&cyclocut_lib, "libcdds-util.a", &prefix, &symbols).unwrap();

        #[derive(Debug)]
        struct PrefixLinkNameCallback {
            prefix: String,
            symbols: HashSet<String>,
        }

        impl bindgen::callbacks::ParseCallbacks for PrefixLinkNameCallback {
            fn generated_link_name_override(
                &self,
                item_info: bindgen::callbacks::ItemInfo<'_>,
            ) -> Option<String> {
                match self.symbols.contains(item_info.name) {
                    true => {
                        let mut prefix = self.prefix.clone();
                        prefix.push_str(item_info.name);
                        Some(prefix)
                    }
                    false => None,
                }
            }
        }

        bindings = bindings.parse_callbacks(Box::new(PrefixLinkNameCallback {
            prefix: prefix.clone(),
            symbols: symbols.clone(),
        }));
    }

    // Add *IMAGE_TLS_DIRECTORY* to blocklist on Windows due to
    // https://github.com/rust-lang/rust-bindgen/issues/2179
    #[cfg(target_os = "windows")]
    let bindings = bindings
        .clang_arg("-Wno-invalid-token-paste")
        .blocklist_type("^(.*IMAGE_TLS_DIRECTORY.*)$");

    // Set link name prefix on additional wrapStringper functions
    generate_template_src(&prefix, &out_dir).unwrap();

    // Generate bindings
    let bindings = bindings.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(all(target_os = "linux", not(feature = "iceoryx")))]
fn get_defined_symbols(lib_dir: &Path, lib_name: &str) -> Result<HashSet<String>, String> {
    let lib_path = lib_dir.to_path_buf().join(lib_name);

    let rc = Command::new("nm")
        .arg("--defined-only")
        .arg("--print-file-name")
        .arg(lib_path)
        .output();

    match rc {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            match stderr.is_empty() {
                true => {
                    let mut result: HashSet<String> = HashSet::new();
                    for line in stdout.lines() {
                        let tokens: Vec<&str> = line.split_whitespace().collect();
                        let symbol = *tokens.last().unwrap();
                        result.insert(String::from(symbol));
                    }
                    Ok(result)
                }
                false => Err(format!(
                    "Failed to run nm on library {} (stderr: {})",
                    lib_name,
                    String::from(stderr)
                )),
            }
        }
        Err(_) => Err(format!("Failed to run nm on library {}", lib_name)),
    }
}

#[cfg(all(target_os = "linux", not(feature = "iceoryx")))]
fn prefix_symbols(
    lib_dir: &Path,
    lib_name: &str,
    prefix: &str,
    symbols: &HashSet<String>,
) -> Result<(), String> {
    let mut objcopy_file_name = lib_name.to_owned();
    objcopy_file_name.push_str(".objcopy");

    let lib_file_path = lib_dir.to_path_buf().join(lib_name);
    let symbol_file_path = lib_dir.to_path_buf().join(objcopy_file_name);

    match File::create(symbol_file_path.clone()) {
        Ok(symbol_file) => {
            let mut symbol_file = LineWriter::new(symbol_file);

            for symbol in symbols {
                let mut symbol_arg = symbol.clone();
                symbol_arg.push(' ');
                symbol_arg.push_str(prefix);
                symbol_arg.push_str(symbol);
                symbol_arg.push('\n');
                if symbol_file.write_all(symbol_arg.as_bytes()).is_err() {
                    return Err(format!(
                        "Failed to write symbol file for library {}",
                        lib_name
                    ));
                }
            }

            if symbol_file.flush().is_err() {
                return Err(format!(
                    "Failed to write symbol file for library {}",
                    lib_name
                ));
            }
            let arg = format!("--redefine-syms={}", symbol_file_path.to_str().unwrap());
            match Command::new("objcopy").arg(arg).arg(lib_file_path).output() {
                Ok(_) => Ok(()),
                Err(_) => Err(format!("Failed to run objcopy on library {}", lib_name)),
            }
        }
        Err(_) => Err(format!(
            "Failed to create symbol file for library {}",
            lib_name
        )),
    }
}

fn generate_template_src(prefix: &str, out_dir: &Path) -> Result<(), String> {
    let src_path = Path::new("src/functions.template");
    let dst_path = out_dir.join("functions.rs");

    match fs::read_to_string(src_path) {
        Ok(mut contents) => {
            contents = contents.replace("<prefix>", prefix);

            match File::create(&dst_path) {
                Ok(mut file) => {
                    if file.write_all(contents.as_bytes()).is_err() {
                        let path = dst_path.to_str().unwrap();
                        return Err(format!(
                            "Failed to write the modified content to the destination file {}",
                            path
                        ));
                    }

                    println!("cargo:rerun-if-changed=src/lib.rs");
                    println!("cargo:rerun-if-changed=src/functions.template");
                    Ok(())
                }
                Err(_) => {
                    let path = dst_path.to_str().unwrap();
                    Err(format!(
                        "Failed to open the destination file ({}) for writing",
                        path
                    ))
                }
            }
        }
        Err(_) => Err(String::from(
            "Failed to read the source file src/functions.template",
        )),
    }
}
