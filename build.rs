use ethers::contract::Abigen;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use eyre::Result;

// Reads the abi directory and calls build_binding for each file
fn main() {
    for entry in fs::read_dir("./abi").expect("could not read abi directory") {
        let entry = entry.expect("error reading directory entry");
        let file_name = entry.file_name();
        let file_name_str = file_name.to_str().expect("error converting file name to string");
        let abi_file_name = file_name_str.split('.').next().expect("invalid ABI file name");
        build_binding(abi_file_name).expect("error building binding");
    }
}

// Create a binding for the contract and adds submodule declaration to the mod.rs file
fn build_binding(abi_file_name: &str) -> Result<()> {
    let abi_path = Path::new("./abi").join(format!("{}.json", abi_file_name));
    let contract_name = format!("{}_mod", abi_file_name);
    let output_path =
        Path::new("./src/bindings").join(format!("{}.rs", abi_file_name.to_lowercase()));
    let mod_rs_path = Path::new("./src/bindings/mod.rs");

    let mut should_generate = true;

    // Check if the binding file exists and if it's newer than the ABI file
    if let Ok(output_meta) = fs::metadata(&output_path) {
        if let Ok(abi_meta) = fs::metadata(&abi_path) {
            if let Ok(output_time) = output_meta.modified() {
                if let Ok(abi_time) = abi_meta.modified() {
                    should_generate = output_time < abi_time;
                }
            }
        }
    }

    if should_generate {
        Abigen::new(&contract_name, abi_path.to_str().unwrap())?
            .generate()?
            .write_to_file(&output_path)?;

        // Create the module declaration
        let module_declaration = format!(
            "pub mod {};\npub use {}::*;\n",
            abi_file_name.to_lowercase(),
            abi_file_name.to_lowercase()
        );

        // Open the mod.rs file in read-only mode
        let mod_rs_file = File::open(mod_rs_path)?;
        let mod_rs_reader = BufReader::new(mod_rs_file);

        // Check if the module declaration is already present in the file
        let mut found = false;
        for line in mod_rs_reader.lines() {
            let line = line?;
            if line.starts_with("pub") &&
                line.contains(&abi_file_name.to_lowercase()) &&
                line.ends_with(';')
            {
                found = true;
                break
            }
        }

        // If the module declaration is not present, write it to the file
        if !found {
            let mut mod_rs_file = fs::OpenOptions::new()
                .append(true)
                .open(mod_rs_path)
                .expect("could not open mod.rs file");
            if mod_rs_file.metadata()?.len() > 0 {
                mod_rs_file.write_all(b"\n")?;
            }
            mod_rs_file
                .write_all(module_declaration.as_bytes())
                .expect("could not write to mod.rs file");
        }
    }

    Ok(())
}
