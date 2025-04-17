use std::env;
use std::path::PathBuf;

use bindgen::RustTarget;

fn main() {
    let target = match RustTarget::stable(75, 0) {
        Ok(target) => target,
        Err(_) => panic!("Invalid rust target"),
    };

    let bindings = bindgen::Builder::default()
        .rust_target(target)
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
