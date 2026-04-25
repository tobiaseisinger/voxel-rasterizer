use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lib");

    let target = env::var("TARGET").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    
    let arch = if target.contains("x86_64") { "x64" } else { "x86" };
    let dll_name = "SDL2.dll";
    let src_dll = format!("lib/{}/{}", arch, dll_name);
    
    let dest_path = Path::new(&out_dir).join("../../..").join(dll_name);

    if Path::new(&src_dll).exists() {
        match fs::copy(&src_dll, &dest_path) {
            Ok(_) => println!("cargo:warning=SDL2.dll erfolgreich nach {:?} kopiert", dest_path),
            Err(e) => println!("cargo:warning=Fehler beim Kopieren der DLL: {}", e),
        }
    } else {
        println!("cargo:warning=WARNUNG: {} wurde nicht gefunden!", src_dll);
    }
}