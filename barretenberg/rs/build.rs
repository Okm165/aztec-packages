use cmake::Config;

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Build the C++ code using CMake and get the build directory path.
    let dst = Config::new("../cpp")
        .profile("RelWithAssert")
        .define("TARGET_ARCH", "skylake")
        .configure_arg("--preset=clang16")
        .configure_arg("--toolchain=../cmake/toolchains/x86_64-linux.cmake")
        .build_target("bb")
        .build();
}
