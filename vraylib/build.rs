extern crate bindgen;
use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let raylib_src = "external/raylib";

    // // Build raylib using CMake
    // let dst = Config::new(raylib_src)
    //     .define("BUILD_SHARED_LIBS", "OFF")
    //     .define("CMAKE_BUILD_TYPE", "Release")
    //     .build();
    //
    // // Link the compiled static library
    println!("cargo:rustc-link-search=usr/local/lib64");
    println!("cargo:rustc-link-lib=raylib");

     // Generate Rust bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(format!("{}/src/raylib.h", raylib_src))
        .clang_arg(format!("-I{}/src", raylib_src))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the output directory
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
