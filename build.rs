use std::{
    env::{self, consts::OS},
    path::PathBuf,
};

use vcpkg;

fn main() {
    // Invalidate build whenever configuration changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");

    // Get the target path.
    let target_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Get the VCPKG_ROOT path.
    let vcpkg_path = env::var("VCPKG_ROOT")
        // If `VCPKG_ROOT` env variable is not set, then return cargo-vcpkg path.
        .unwrap_or_else(|_| format!("{}/target/vcpkg", target_path.display()));

    // Build ALE using CMake.
    let ale = cmake::Config::new("ale")
        // Set VCPKG_ROOT env variable.
        .env("VCPKG_ROOT", &vcpkg_path)
        // Enable Linking Time Optimization (LTO).
        .define("ENABLE_LTO", "ON")
        // Set the build release type.
        .define("CMAKE_BUILD_TYPE", "Release")
        // Disable Python support.
        .define("BUILD_PYTHON_LIB", "OFF")
        // Set SDL2 support.
        .define(
            "SDL_SUPPORT",
            match cfg!(feature = "sdl2") {
                false => "OFF",
                true => "ON",
            },
        )
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

    // Generate bindings using autocxx.
    autocxx_build::Builder::new(
        "src/lib.rs",
        [
            ale.join("include/ale"),
            ale.join("include/ale/games"),
            ale.join("include/ale/games/supported"),
            utils.join("include/utils"),
        ],
    )
    .extra_clang_args(&["-v", "-std=c++17"])
    // Generate the bindings.
    .build()
    .expect("Unable to generate the bindings")
    .flag_if_supported("-std=c++17")
    // Compile and link the cxx bridge.
    .compile("autocxx");

    // Link autocxx bindings.
    println!("cargo:rustc-link-search=native={}", ale.display());
    println!("cargo:rustc-link-lib=autocxx");
    // Link ALE.
    println!("cargo:rustc-link-search=native={}", ale.join("lib").display());
    println!("cargo:rustc-link-lib=ale");
    // Link utils.
    println!("cargo:rustc-link-search=native={}", utils.join("lib").display());
    println!("cargo:rustc-link-lib=utils");

    // Link the C++ standard library.
    match OS {
        "apple" => println!("cargo:rustc-link-lib=dylib=c++"),
        "linux" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        "windows" => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => unimplemented!(),
    }

    // Link ZLIB.
    vcpkg::Config::new()
        .vcpkg_root(vcpkg_path.clone().into())
        .find_package("zlib")
        .expect("Unable to find ZLIB")
        .cargo_metadata
        .into_iter()
        .for_each(|x| {
            println!("{}", x);
        });

    // Link SDL2, if enabled.
    if cfg!(feature = "sdl2") {
        vcpkg::Config::new()
            .vcpkg_root(vcpkg_path.clone().into())
            .find_package("sdl2")
            .expect("Unable to find SDL2")
            .cargo_metadata
            .into_iter()
            .for_each(|x| {
                println!("{}", x);
            });
    }
}
