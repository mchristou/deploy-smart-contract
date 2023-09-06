use std::{path::PathBuf, process::Command};

fn compile(path: PathBuf) {
    if !path.exists() {
        panic!("Path does not exist");
    }

    let output = Command::new("solc")
        .arg(path)
        .arg("--overwrite")
        .arg("--bin")
        .arg("--abi")
        .arg("-o")
        .arg("out")
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        panic!("Failed to compile")
    }
}

fn main() {
    let path = PathBuf::from("contracts/HelloWorld.sol");

    println!("cargo:rerun-if-changed={}", path.display());

    compile(path);
}
