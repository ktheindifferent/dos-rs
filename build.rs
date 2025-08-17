use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    
    fs::copy("com.ld", Path::new(&out_dir).join("com.ld"))
        .expect("Failed to copy com.ld to output directory. Make sure com.ld exists in the project root.");
    
    fs::copy("startup.o", Path::new(&out_dir).join("startup.o"))
        .expect("Failed to copy startup.o to output directory. Make sure startup.o exists in the project root.");
}
