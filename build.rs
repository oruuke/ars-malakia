use std::fs;
use std::io::Write;

fn main() {
    let dir = "src/pages";
    let mod_path = format!("{}/mod.rs", dir);
    let mut mod_file = fs::File::create(&mod_path).unwrap();

    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            if stem != "mod" {
                writeln!(mod_file, "pub mod {};", stem).unwrap();
            }
        }
    }
    println!("cargo:rerun-if-changed=src/pages/");
}
