extern crate bindgen;
extern crate cmake;

use std::{
    env::{self, consts::OS},
    path::PathBuf,
};

use vcpkg;

fn main() {
    // Invalidate build whenever configuration changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // Get OUT_DIR path.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Get the target path.
    let target_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Get the VCPKG_ROOT path.
    let vcpkg_path = format!("{}/target/vcpkg", target_path.display());

    // Build ALE using CMake.
    let ale = cmake::Config::new("ale")
        // Set VCPKG_ROOT env variable.
        .env("VCPKG_ROOT", &vcpkg_path)
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

    // Build utils using CMake.
    let utils = cmake::Config::new("utils")
        // Enable Linking Time Optimization (LTO).
        .define("ENABLE_LTO", "ON")
        // Set the build release type.
        .define("CMAKE_BUILD_TYPE", "Release")
        // Build the library.
        .build();

    // Link ALE.
    println!("cargo:rustc-link-search={}", ale.join("lib").display());
    println!("cargo:rustc-link-lib=ale");
    // Link utils.
    println!("cargo:rustc-link-search={}", utils.join("lib").display());
    println!("cargo:rustc-link-lib=utils");
    // Link the C++ standard library.
    match OS {
        "apple" => println!("cargo:rustc-link-lib=dylib=c++"),
        "linux" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        "windows" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => unimplemented!(),
    }
    // Link SDL2.
    vcpkg::Config::new()
        .vcpkg_root(vcpkg_path.clone().into())
        .find_package("sdl2")
        .expect("Unable to find SDL2")
        .cargo_metadata
        .into_iter()
        .for_each(|x| {
            println!("{}", x);
        });
    // Link ZLIB.
    vcpkg::Config::new()
        .vcpkg_root(vcpkg_path.into())
        .find_package("zlib")
        .expect("Unable to find ZLIB")
        .cargo_metadata
        .into_iter()
        .for_each(|x| {
            println!("{}", x);
        });

    // Generate the bindings.
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
        // Disable `Copy` derive to avoid implicit copy.
        .derive_copy(false)
        // Map `size_t` as `usize`.
        .size_t_is_usize(true)
        // Set enum style.
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        // Map C++ namespaces to Rust modules.
        .enable_cxx_namespaces()
        // Fix wrong alignments.
        .layout_tests(false)
        // Fix cyclical definitions.
        .blocklist_type("iterator")
        .blocklist_type("int_type")
        .blocklist_type("size_type")
        .blocklist_type("char_type")
        .blocklist_type("__hashtable")
        // Fix missing types.
        .module_raw_line("root", "pub type size_type = usize;")
        .module_raw_line("root::std", "pub type _CharT = ::std::os::raw::c_char;")
        .module_raw_line("root::std", "pub type _Iterator = usize;")
        .module_raw_line("root::std", "pub type _NodeHandle = usize;")
        .module_raw_line("root::std", "pub type _RehashPolicy = usize;")
        .module_raw_line("root::std", "pub type _Tp = usize;")
        .module_raw_line("root::std", "pub type _Traits = usize;")
        .module_raw_line("root::__gnu_cxx", "pub type _Value = usize;")
        // Generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = &out_path.join("bindings.rs");
    bindings.write_to_file(out_path).expect("Unable to write bindings");
}
