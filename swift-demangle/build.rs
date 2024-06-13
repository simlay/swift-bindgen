use std::{
    process::Command,
    path::PathBuf,
    env,
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("env variable OUT_DIR not found"));
    let out_swift = out_dir.join("libswiftDemangle.a");
    let swiftc = Command::new("swiftc")
        .arg("-static")
        .arg("-o")
        .arg(out_swift)
        .arg("-emit-library")
        .arg("./src/demangle.swift")
        .output()
        .expect("Failed to compile swift library");
    if !swiftc.status.success() {
        let stderr = String::from_utf8_lossy(&swiftc.stderr);
        panic!("{}", stderr);
    }
    println!("cargo:rustc-link-search={}", out_dir.display());
    Ok(())
}
