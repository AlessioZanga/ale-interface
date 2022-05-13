extern crate bindgen;
extern crate cmake;

use std::{
    env::{self, consts::OS},
    path::PathBuf,
};

use vcpkg;

fn main() {
    // Get the target dir.
    let target_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Get the VCPKG_ROOT.
    let vcpkg_root = format!("{}/target/vcpkg", target_path.display());

    // Define the cmake configuration.
    let config = cmake::Config::new("ale")
        // Set VCPKG_ROOT env variable.
        .env("VCPKG_ROOT", &vcpkg_root)
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
        "cargo:rustc-link-search=native={}",
        config.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=ale");
    // Link to the C++ standard library.
    match OS {
        "apple" => println!("cargo:rustc-link-lib=dylib=c++"),
        "linux" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        "windows" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => unimplemented!(),
    }
    // Link SDL2.
    vcpkg::Config::new()
        .vcpkg_root(vcpkg_root.clone().into())
        .find_package("sdl2")
        .expect("Unable to find SDL2")
        .cargo_metadata
        .into_iter()
        .for_each(|x| {
            println!("{}", x);
        });
    // Link to ZLIB.
    vcpkg::Config::new()
        .vcpkg_root(vcpkg_root.into())
        .find_package("zlib")
        .expect("Unable to find ZLIB")
        .cargo_metadata
        .into_iter()
        .for_each(|x| {
            println!("{}", x);
        });
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
        // Map `size_t` as `usize`.
        .size_t_is_usize(true)
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
