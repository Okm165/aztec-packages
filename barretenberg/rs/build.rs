use cmake::Config;

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
}

