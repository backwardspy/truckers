use std::{env, path::PathBuf};

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .blocklist_function("scs_telemetry_init")
        .blocklist_function("scs_telemetry_shutdown")
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
