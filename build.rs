extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
	println!("cargo:rustc-link-lib=bcm_host");
	println!("cargo:rustc-link-lib=brcmEGL");
	println!("cargo:rustc-link-lib=brcmGLESv2");
	println!("cargo:rustc-link-lib=brcmOpenVG");
	println!("cargo:rustc-link-lib=brcmWFC");
	println!("cargo:rustc-link-lib=containers");
	println!("cargo:rustc-link-lib=debug_sym");
	println!("cargo:rustc-link-lib=dtovl");
	println!("cargo:rustc-link-lib=EGL");
	println!("cargo:rustc-link-lib=elftoolchain");
	println!("cargo:rustc-link-lib=GLESv2");
	println!("cargo:rustc-link-lib=mmal_components");
	println!("cargo:rustc-link-lib=mmal_core");
	println!("cargo:rustc-link-lib=mmal");
	println!("cargo:rustc-link-lib=mmal_util");
	println!("cargo:rustc-link-lib=mmal_vc_client");
	println!("cargo:rustc-link-lib=openmaxil");
	println!("cargo:rustc-link-lib=OpenVG");
	println!("cargo:rustc-link-lib=vchiq_arm");
	println!("cargo:rustc-link-lib=vcos");
	println!("cargo:rustc-link-lib=vcsm");
	println!("cargo:rustc-link-lib=WFC");


    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
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
}

