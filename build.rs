extern crate bindgen;
extern crate cmake;

use std::{env, path::PathBuf};

fn main() {
    // Get the target dir.
    let target_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Define the cmake configuration.
    let config = cmake::Config::new("ale")
        // Set VCPKG_ROOT env variable.
        .env(
            "VCPKG_ROOT",
            format!("{}/target/vcpkg", target_path.display()),
        )
        // Enable Linking Time Optimization (LTO).
        .define("ENABLE_LTO", "ON")
        // Set the build release type.
        .define("CMAKE_BUILD_TYPE", "Release")
        // Disable Python lib.
        .define("BUILD_PYTHON_LIB", "OFF")
        // Enable SDL support.
        .define("SDL_SUPPORT", "ON")
        // Build the library.
        .build();

    // Link to the native library.
    println!(
        "cargo:rustc-link-search=native={}/libale",
        config.join("lib").display()
    );
    // Invalidate the build crate whenever the configuration changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Get OUT_DIR path.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Define the bindings.
    let bindings = bindgen::Builder::default()
        // Set verbose.
        .clang_arg("-v")
        // Set the C++17 language.
        .clang_arg("-std=c++17")
        // Include the library version header.
        .clang_arg(format!("-I{}/include/ale/", out_path.display()))
        // Include the library headers.
        .detect_include_paths(true)
        // Set the wrapper to generate the bindings for.
        .header("wrapper.hpp")
        // Invalidate the built crate whenever the wrapper changes.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Set enum style.
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        // Map C++ namespaces to Rust modules.
        .enable_cxx_namespaces()
        // Handle opaque types.
        .opaque_type(".*")
        .blocklist_type("iterator")
        .blocklist_type("int_type")
        .blocklist_type("char_type")
        .blocklist_type("size_type")
        .blocklist_type("__hashtable")
        // Patch missing types.
        .module_raw_line("root::std", "pub type _Tp = usize;")
        .module_raw_line("root::__gnu_cxx", "pub type _Value = usize;")
        // Generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = &out_path.join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");
}
