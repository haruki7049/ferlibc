use cbindgen::Config;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = env::var("OUT_DIR").unwrap();
    let config_path = format!("{}/ferlibc.h", target_dir);
    let config: Config = Config::from_file("cbindgen.toml").expect("Unable to find cbindgen.toml");

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file(config_path);
}
