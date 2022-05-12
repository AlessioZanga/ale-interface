extern crate bindgen;
extern crate cmake;

use std::{env, path::PathBuf};

fn main() {
    // Define the cmake configuration.
    let config = cmake::Config::new("ale")
        // Set CXX version.
        // Set the build release type.
        .define("CMAKE_BUILD_TYPE", "Release")
        // Disable Python lib.
        .define("BUILD_PYTHON_LIB", "OFF")
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
        // Include the library headers.
        .clang_arg("-Iale/src/")
        // Include the library version header.
        .clang_arg(format!("-I{}/include/ale/", out_path.display()))
        // Set the wrapper to generate the bindings for.
        .header("wrapper.hpp")
        // Invalidate the built crate whenever the wrapper changes.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Handle opaque types.
        .opaque_type("std::.*")
        .blocklist_type("iterator")
        .blocklist_type("int_type")
        .blocklist_type("char_type")
        .blocklist_type("size_type")
        .blocklist_type("__hashtable")
        // Generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = &out_path.join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Unable to write bindings");

    // FIXME: Read and patch the bindings.
    let bindings = std::fs::read_to_string(out_path)
        .expect("Unable to read bindings")
        // Replace `_Tp` placeholder with `usize`.
        .replace("_Tp", "usize")
        // Replace `_Value` placeholder with `usize`.
        .replace("_Value", "usize");

    // Write the patched bindings.
    std::fs::write(&out_path, bindings).expect("Unable to write bindings");
}
