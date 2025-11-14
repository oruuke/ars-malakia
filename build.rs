use std::fs;
use std::io::Write;

fn main() {
    // determine pages directory
    let dir = "src/pages";
    let mod_path = format!("{}/mod.rs", dir);
    let mut mod_file = fs::File::create(&mod_path).unwrap();

    // have mod.rs import Page struct
    writeln!(mod_file, "use crate::view::Page;").unwrap();
    let mut modules = Vec::new();

    // iterate all files in directory
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // ensure is rust file
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            let stem = path.file_stem().unwrap().to_str().unwrap();
            // include file if not the mod.rs itself
            if stem != "mod" {
                writeln!(mod_file, "pub mod {};", stem).unwrap();
                modules.push(stem.to_string());
            }
        }
    }

    // generate constant for ordered iteration
    modules.sort();
    writeln!(
        mod_file,
        "\npub const ALL_PAGES: &[fn(&u16, u16) -> Page<'static>] = &["
    )
    .unwrap();

    // add each module
    for module in &modules {
        writeln!(mod_file, "    {}::create_page,", module).unwrap();
    }
    writeln!(mod_file, "];").unwrap();

    println!("cargo:rerun-if-changed=src/pages/");
}
