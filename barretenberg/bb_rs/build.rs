use std::env;
use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Build the C++ code using CMake and get the build directory path.
    let dst = Config::new("../cpp")
        .profile("RelWithAssert")
        .define("TARGET_ARCH", "x86-64")
        .configure_arg("--toolchain=cmake/toolchains/x86_64-linux.cmake")
        .build_target("barretenberg")
        .build();

    // Add the library search path for Rust to find during linking.
    println!("cargo:rustc-link-search={}/lib", dst.display());

    // Link the `barretenberg` static library.
    println!("cargo:rustc-link-lib-static=barretenberg");

    // Link the C++ standard library.
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    let bindings = bindgen::Builder::default()
        // Add the include path for headers.
        .clang_args([format!("-I{}/include", dst.display())])
        // The input header we would like to generate bindings for.
        .header_contents(
            "wrapper.hpp",
            r#"
                
            "#,
        )
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
