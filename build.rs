use cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_name = env::var("CARGO_PKG_NAME").unwrap();
    let profile = env::var("PROFILE").unwrap();
    let header_path = format!("./target/{}/{}.h", profile, crate_name);

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_cpp_compat(false)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);
}
