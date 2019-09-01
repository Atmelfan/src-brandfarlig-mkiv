use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{PathBuf};
use std::process::Command;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out_dir = env::var("OUT_DIR").unwrap();
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    let output = Command::new("dtc")
        .args(&["-I", "dts", "-O", "dtb", "-S", "8192"])
        .args(&["-o", &format!("{}/tree.dtb", out_dir)])
        .arg("src/brandfarlig-mkiv.dts").output()
        .expect("failed to execute process");

    println!("cargo:rerun-if-changed=src/brandfarlig-mkiv.dts");

    if !output.status.success() {
        panic!("DTC {}", String::from_utf8_lossy(&output.stderr));
    }


}
