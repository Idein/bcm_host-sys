extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to tell rustc to link the system bcm_host shared library.
    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-lib=vchiq_arm");
    println!("cargo:rustc-link-lib=vcos");
    println!("cargo:rustc-link-lib=pthread");

    let mut include_dirs = Vec::new();

    // Add Paths to directories of VideoCore firmware header
    let dir = env::var("VC_INCLUDE_DIR").unwrap_or("/opt/vc/include".into());
    include_dirs.push(Path::new(&dir).into());
    include_dirs.push(Path::new(&dir).join("interface").join("vmcs_host").join(
        "linux",
    ));
    include_dirs.push(Path::new(&dir).join("interface").join("vcos").join(
        "pthreads",
    ));

    // Add Path to directories of Clang header
    include_dirs.push(
        Path::new(&env::var("CLANG_INCLUDE_DIR").expect(
            "CLANG_INCLUDE_DIR like: /usr/lib/llvm-3.9/lib/clang/3.9.1/include",
        )).into(),
    );

    let args: Vec<&str> = include_dirs
        .iter()
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();
    // for &s in &args {
    // 	debug!("> {}", s);
    // }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
		.clang_args(&["-D", "USE_VCHIQ_ARM"])
		.clang_args(&args)
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
