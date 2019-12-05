extern crate prost_build;
extern crate cbindgen;
use std::process::Command;
use std::io::{self, Write};

use std::fs::remove_dir_all;

use std::env;

fn main() {
    prost_build::compile_protos(
        &[
            "../prototypes/analysis.proto",
            "../prototypes/release.proto",
            "../prototypes/types.proto",
            "../prototypes/dataset.proto"
        ],
        &["../prototypes/"]).unwrap();

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate_with_config(
        crate_dir,
        cbindgen::Config::from_file("cbindgen.toml").unwrap())
        .expect("Unable to generate bindings")
        .write_to_file("api.h");

    // generate openapi files
    remove_dir_all("./src/models");
    remove_dir_all("./docs");

    let output = Command::new("java")
        .args(&["-Dmodels", "-DsupportingFiles=mod.rs"])
        .args(&["-jar", "../openapi-generator-cli.jar"])
        .arg("generate")
        .args(&["-i", "../definitions/analysis.yaml"])
        .args(&["-g", "rust"])
        .args(&["-o", "."])
        .output()
        .expect("failed to execute process");
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    remove_dir_all("./src/apis");
}