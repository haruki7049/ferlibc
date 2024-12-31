use std::env;
use cbindgen::Config;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config: Config = Config::from_file("cbindgen.toml").expect("Unable to find cbindgen.toml");

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("../../target/ferlibc_c99.h");
}
