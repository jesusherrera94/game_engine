fn main() {
    // Compile the C code for the OpenGL wrapper library
    // supporting both macOS and Linux
    let mut build = cc::Build::new();
    build.file("../opengl_wrapper_lib/opengl_wrapper_lib.c");

    if cfg!(target_os = "macos") {
        build.include("/opt/homebrew/include");
    }

    build.compile("opengl_wrapper_lib");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=/opt/homebrew/lib");
        println!("cargo:rustc-link-lib=glfw");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    } else {
        println!("cargo:rustc-link-lib=glfw");
        println!("cargo:rustc-link-lib=GL");
    }
}