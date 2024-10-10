use std::env;
use std::path::{Path};

fn main() {
    // let pwd_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let path = Path::new(&*pwd_dir).join("lib");
    // println!("cargo:rustc-link-search=native={}", path.to_str().unwrap());
    println!("cargo:rustc-link-arg=-Wl,-rpath,/home/chani/Ubuntu/Advisory/YOLO/rust_darknet/darknet");
    println!("cargo:rustc-link-search=native=/home/chani/Ubuntu/Advisory/YOLO/rust_darknet/darknet");
    println!("cargo:rustc-link-lib=dylib=darknet");
    println!("cargo:rustc-link-lib=dylib=stdc++");
}
