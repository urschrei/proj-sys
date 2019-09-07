extern crate bindgen;

use std::env;
use std::path::PathBuf;

#[cfg(feature = "docs-rs")]
fn main() {} // Skip the build script on docs.rs

#[cfg(not(feature = "docs-rs"))]
fn main() {
    // Tell cargo to tell rustc to link the system proj
    // shared library.
    println!("cargo:rustc-link-lib=proj");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .trust_clang_mangling(false)
        .blacklist_type("max_align_t")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    // we're doing this so docs.rs can build the library
    let backup_out_path = PathBuf::from(r"src");
    bindings
        .write_to_file(backup_out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
