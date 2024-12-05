extern crate bindgen;

#[allow(unused_imports)]
use std::collections::HashSet;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::{Path, PathBuf};
#[allow(unused_imports)]
use std::process::Command;
use std::{env, fs};
use std::{ffi::OsStr, fs::metadata, fs::File};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dir_builder = std::fs::DirBuilder::new();
    dir_builder.recursive(true);

    // Check features
    let iceoryx_enabled = is_iceoryx_enabled();
    let prefix_symbols_enabled = is_prefix_symbols_enabled();

    if iceoryx_enabled && prefix_symbols_enabled {
        print!("cargo:warning=iceoryx and prefix_symbols features cannot both be enabled!");
        std::process::exit(1);
    }

    // Determine symbol prefix
    let prefix = match prefix_symbols_enabled {
        true => {
            let mut prefix = env::var("CARGO_PKG_VERSION").unwrap().replace('.', "_");
            prefix.insert_str(0, "cyclors_");
            prefix.push('_');
            prefix
        }
        false => String::new(),
    };

    // Build Iceoryx (if enabled)
    let mut iceoryx = PathBuf::new();
    if iceoryx_enabled {
        let iceoryx_src_dir = Path::new("iceoryx/iceoryx_meta");
        let iceoryx_out_dir = out_dir.join("iceoryx-build");
        dir_builder.create(&iceoryx_out_dir).unwrap();
        iceoryx = build_iceoryx(iceoryx_src_dir, &iceoryx_out_dir);
    }

    // Build Cyclone DDS
    let cyclonedds_src_dir = prepare_cyclonedds_src("cyclonedds", &out_dir, &prefix);
    let cyclonedds_out_dir = out_dir.join("cyclonedds-build");
    dir_builder.create(&cyclonedds_out_dir).unwrap();
    let cyclonedds = build_cyclonedds(
        &cyclonedds_src_dir,
        &cyclonedds_out_dir,
        iceoryx.as_os_str(),
    );

    // Prefix Cyclone DDS library symbols if enabled
    let mut symbols = HashSet::new();
    if prefix_symbols_enabled {
        let cyclonedds_lib = cyclonedds.join("lib");
        let ddsc_lib_name = get_library_name("ddsc").unwrap();
        let cyclone_symbols = get_defined_symbols(&cyclonedds_lib, &ddsc_lib_name)
            .expect("Failed to get symbols from ddsc library!");
        symbols.extend(cyclone_symbols);
        prefix_symbols(&cyclonedds_lib, &ddsc_lib_name, &prefix, &symbols).unwrap();
    }

    // Build cyclocut
    let cyclocut_src_dir = Path::new("cyclocut");
    let cyclocut_out_dir = out_dir.join("cyclocut-build");
    dir_builder.create(&cyclocut_out_dir).unwrap();
    let cyclocut = build_cyclocut(cyclocut_src_dir, &cyclocut_out_dir, &cyclonedds);

    // Prefix Cyclocut library symbols if enabled
    if prefix_symbols_enabled {
        let cyclocut_lib = cyclocut.join("lib");
        let cyclocut_lib_name = get_library_name("cdds-util").unwrap();
        let cyclocut_symbols = get_defined_symbols(&cyclocut_lib, &cyclocut_lib_name)
            .expect("Failed to get symbols from cdds-util library!");
        symbols.extend(cyclocut_symbols);
        prefix_symbols(&cyclocut_lib, &cyclocut_lib_name, &prefix, &symbols).unwrap();
    }

    // Configure bindings build
    let cyclonedds_include = cyclonedds.join("include");
    let cyclocut_include = cyclocut.join("include");

    let mut bindings = bindgen::Builder::default();
    bindings = bindings
        .header("wrapper.h")
        .clang_arg(format!("-I{}", cyclonedds_include.to_str().unwrap()))
        .clang_arg(format!("-I{}", cyclocut_include.to_str().unwrap()))
        .generate_comments(false);

    // Set link name if prefix enabled
    if prefix_symbols_enabled {
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
                let mut item = String::from("");
                #[cfg(target_os = "macos")]
                item.push('_');
                item.push_str(item_info.name);
                match self.symbols.contains(&item) {
                    true => {
                        let mut prefix = String::from("");
                        #[cfg(target_os = "macos")]
                        prefix.push('_');
                        prefix.push_str(&self.prefix);
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

    // Set link name prefix on additional wrapper functions
    generate_template_src(&prefix, &out_dir).unwrap();

    // Generate bindings
    let bindings = bindings.generate().expect("Unable to generate bindings");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn is_iceoryx_enabled() -> bool {
    #[cfg(feature = "iceoryx")]
    {
        #[cfg(target_os = "windows")]
        {
            print!("cargo:warning=Cyclone DDS Iceoryx PSMX plugin is not supported on Windows!");
            std::process::exit(1);
        }
        true
    }
    #[cfg(not(feature = "iceoryx"))]
    {
        false
    }
}

fn is_prefix_symbols_enabled() -> bool {
    #[cfg(feature = "prefix_symbols")]
    {
        true
    }
    #[cfg(not(feature = "prefix_symbols"))]
    {
        false
    }
}

fn build_iceoryx(src_dir: &Path, out_dir: &Path) -> PathBuf {
    let mut iceoryx = cmake::Config::new(src_dir);
    let iceoryx_path = iceoryx
        .define("BUILD_SHARED_LIBS", "OFF")
        .out_dir(out_dir)
        .build();

    // Add iceoryx lib to link
    let iceoryx_lib = iceoryx_path.join("lib");
    println!("cargo:rustc-link-search=native={}", iceoryx_lib.display());
    println!("cargo:rustc-link-lib=static=iceoryx_hoofs");
    println!("cargo:rustc-link-lib=static=iceoryx_posh");
    println!("cargo:rustc-link-lib=static=iceoryx_platform");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=acl");

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    iceoryx_path
}

fn build_cyclonedds(src_dir: &Path, out_dir: &Path, iceoryx_path: &OsStr) -> PathBuf {
    // Create Cyclone DDS build initial config
    let mut cyclonedds = cmake::Config::new(src_dir);
    let mut cyclonedds = cyclonedds.out_dir(out_dir);

    // Configure cyclonedds build
    cyclonedds = cyclonedds
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("BUILD_IDLC", "OFF")
        .define("BUILD_DDSPERF", "OFF")
        .define("ENABLE_LTO", "NO")
        .define("ENABLE_SSL", "NO")
        .define("ENABLE_SECURITY", "NO")
        .define("CMAKE_INSTALL_LIBDIR", "lib");

    if !iceoryx_path.is_empty() {
        cyclonedds = cyclonedds
            .env("iceoryx_hoofs_DIR", iceoryx_path)
            .env("iceoryx_posh_DIR", iceoryx_path)
            .define("ENABLE_ICEORYX", "YES");
    } else {
        cyclonedds = cyclonedds.define("ENABLE_ICEORYX", "NO");
    }

    // Force compilation of Cyclone DDS in release mode on Windows due to
    // https://github.com/rust-lang/rust/issues/39016
    #[cfg(all(debug_assertions, target_os = "windows"))]
    let cyclonedds = cyclonedds.profile("Release");

    // Build cyclonedds
    let cyclonedds_path = cyclonedds.build();

    // Add cyclonedds lib to link
    let cyclonedds_lib = cyclonedds_path.join("lib");
    println!(
        "cargo:rustc-link-search=native={}",
        cyclonedds_lib.display()
    );
    println!("cargo:rustc-link-lib=static=ddsc");

    // Add Windows libraries required by Cyclone to link
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=Iphlpapi");
        println!("cargo:rustc-link-lib=DbgHelp");
        println!("cargo:rustc-link-lib=Bcrypt");
    }

    cyclonedds_path
}

fn build_cyclocut(src_dir: &Path, out_dir: &Path, cyclonedds_dir: &Path) -> PathBuf {
    let mut cyclocut = cmake::Config::new(src_dir);

    // Force compilation of Cyclocut in release mode on Windows due to
    // https://github.com/rust-lang/rust/issues/39016
    #[cfg(all(debug_assertions, target_os = "windows"))]
    let cyclocut = cyclocut.profile("Release");

    let cyclonedds_include = cyclonedds_dir.join("include");
    let cyclonedds_lib = cyclonedds_dir.join("lib");

    let cyclocut_path = cyclocut
        .env("CYCLONE_INCLUDE", &cyclonedds_include)
        .env("CYCLONE_LIB", &cyclonedds_lib)
        .define("CYCLONE_INCLUDE", cyclonedds_include.clone())
        .define("CYCLONE_LIB", cyclonedds_lib.clone())
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .out_dir(out_dir)
        .build();

    // Add cyclocut lib to link
    let cyclocut_lib = cyclocut_path.join("lib");
    println!("cargo:rustc-link-search=native={}", cyclocut_lib.display());
    println!("cargo:rustc-link-lib=static=cdds-util");

    cyclocut_path
}

#[allow(unused_variables)]
fn prepare_cyclonedds_src(src_dir: &str, out_dir: &Path, prefix: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    if !prefix.is_empty() {
        let mut dst_dir = src_dir.to_string();
        dst_dir.push_str("-src");
        let dst_dir = out_dir.join(dst_dir);

        // Delete copied source directory if it already exists
        if dst_dir.exists() {
            fs::remove_dir_all(dst_dir.clone()).unwrap();
        }
        copy_dir_recursive(&PathBuf::from(src_dir), &dst_dir).unwrap();

        // Prefix tls_callback_func in cyclonedds-src/src/ddsrt/src/cdtors.c
        let mut prefixed_func = prefix.to_string();
        prefixed_func.push_str("tls_callback_func");
        let cdtors = dst_dir
            .join("src")
            .join("ddsrt")
            .join("src")
            .join("cdtors.c");
        replace_in_file(&cdtors, "tls_callback_func", &prefixed_func).unwrap();

        return dst_dir;
    }
    PathBuf::from(src_dir)
}

#[allow(unused)]
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    println!(
        "src = {}, dir = {}",
        src.to_str().unwrap(),
        dst.to_str().unwrap()
    );
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

#[allow(unused)]
fn replace_in_file(file_path: &Path, from: &str, to: &str) -> std::io::Result<()> {
    // Read the file content into a string
    let content = fs::read_to_string(file_path)?;

    // Replace all occurrences of `from` with `to`
    let new_content = content.replace(from, to);

    // Write the modified content back to the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true) // Clear the file before writing
        .open(file_path)?;

    file.write_all(new_content.as_bytes())?;
    Ok(())
}

#[allow(unused_variables)]
fn get_library_name(lib_name: &str) -> Option<String> {
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        let mut file_name = String::from("lib");
        file_name.push_str(lib_name);
        file_name.push_str(".a");
        Some(file_name)
    }
    #[cfg(target_os = "windows")]
    {
        let mut file_name = String::from(lib_name);
        file_name.push_str(".lib");
        Some(file_name)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    None
}

fn get_defined_symbols(lib_dir: &Path, lib_name: &str) -> Result<HashSet<String>, String> {
    let lib_path = lib_dir.to_path_buf().join(lib_name);
    let mut nm_file_name = lib_name.to_owned();
    nm_file_name.push_str(".nm");
    let symbol_file_path = lib_dir.to_path_buf().join(nm_file_name);

    let mut nm = cmake::Config::new("nm");
    nm.build_target("read_symbols")
        .define("LIB_PATH", lib_path.clone())
        .build();

    // Check for unexpected errors in stderr.txt
    let mut stderr_file_name = lib_name.to_owned();
    stderr_file_name.push_str(".nm.stderr");
    let stderr_file_path = lib_dir.to_path_buf().join(stderr_file_name);
    check_nm_stderr(&stderr_file_path).unwrap();

    match File::open(symbol_file_path.clone()) {
        Ok(symbol_file) => {
            let reader = BufReader::new(symbol_file);

            let mut result: HashSet<String> = HashSet::new();
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        let tokens: Vec<&str> = line.split_whitespace().collect();
                        let symbol = *tokens.last().unwrap();
                        #[cfg(target_os = "windows")]
                        if !symbol.ends_with("tls_callback_func") {
                            result.insert(String::from(symbol));
                        }
                        #[cfg(not(target_os = "windows"))]
                        result.insert(String::from(symbol));
                    }
                    Err(_) => return Err(format!("Failed to run nm on library {}", lib_name)),
                }
            }
            Ok(result)
        }
        Err(_) => {
            println!(
                "nm file open problem: {}",
                symbol_file_path.to_str().unwrap()
            );
            Err(format!("Failed to run nm on library {}", lib_name))
        }
    }
}

fn check_nm_stderr(stderr: &Path) -> Result<(), String> {
    match File::open(stderr) {
        Ok(file) => {
            let reader = BufReader::new(file);

            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        // Some object files within the library may report no symbols - this is okay
                        if !line.ends_with(": no symbols") {
                            return Err(format!(
                                "nm completed with errors - see {} for details",
                                stderr.to_str().unwrap()
                            ));
                        }
                    }
                    Err(_) => {
                        return Err(format!(
                            "Failed to read nm stderr file: {}",
                            stderr.to_str().unwrap()
                        ))
                    }
                }
            }
        }
        Err(_) => {
            return Err(format!(
                "Failed to open nm stderr file: {}",
                stderr.to_str().unwrap()
            ));
        }
    }
    Ok(())
}

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

                #[cfg(target_os = "macos")]
                {
                    let mut symbol_stripped = symbol.clone();
                    symbol_stripped.remove(0);
                    symbol_arg.push(' ');
                    symbol_arg.push('_');
                    symbol_arg.push_str(prefix);
                    symbol_arg.push_str(&symbol_stripped);
                }
                #[cfg(not(target_os = "macos"))]
                {
                    symbol_arg.push(' ');
                    symbol_arg.push_str(prefix);
                    symbol_arg.push_str(symbol);
                }
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

            let mut objcopy = cmake::Config::new("objcopy");
            objcopy
                .build_target("mangle_library")
                .define("LIB_PATH", lib_file_path.clone())
                .define("SYMBOL_FILE_PATH", symbol_file_path.clone())
                .build();

            // Check for unexpected errors in stderr.txt
            let mut stderr_file_name = lib_name.to_owned();
            stderr_file_name.push_str(".objcopy.stderr");
            let stderr_file_path = lib_dir.to_path_buf().join(stderr_file_name);
            check_objcopy_stderr(&stderr_file_path).unwrap();
            Ok(())
        }
        Err(_) => Err(format!(
            "Failed to create symbol file for library {}",
            lib_name
        )),
    }
}

fn check_objcopy_stderr(stderr: &Path) -> Result<(), String> {
    if let Ok(metadata) = metadata(stderr) {
        if metadata.is_file() {
            if metadata.len() > 0 {
                println!("File exists and has a size greater than 0.");
                Err(format!(
                    "Objcopy command failed with errors - see {} for details",
                    stderr.to_str().unwrap()
                ))
            } else {
                Ok(())
            }
        } else {
            Err(format!(
                "Objcopy stderr file is not a file: {}",
                stderr.to_str().unwrap()
            ))
        }
    } else {
        Err(format!(
            "Failed to read objcopy stderr file metadata: {}",
            stderr.to_str().unwrap()
        ))
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
