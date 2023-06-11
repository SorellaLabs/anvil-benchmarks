use ethers::contract::Abigen;
use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use eyre::Result;

// Read the ABI directory and call build_binding for each file
fn main() -> Result<()> {
    let abi_dir = "./abi";
    let bindings_dir = "./benches/bindings";
    let mod_file_path = Path::new(bindings_dir).join("mod.rs");

    ensure_directory_exists(bindings_dir)?;
    ensure_file_exists(&mod_file_path)?;

    for entry in fs::read_dir(abi_dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name
            .to_str()
            .ok_or_else(|| eyre::eyre!("Error converting file name to string"))?;
        let abi_file_name =
            file_name_str.split('.').next().ok_or_else(|| eyre::eyre!("Invalid ABI file name"))?;

        build_binding(abi_file_name, abi_dir, bindings_dir)?;
        update_mod_file(abi_file_name, &mod_file_path)?;
    }

    Ok(())
}

// Generate a binding for the contract
fn build_binding(abi_file_name: &str, abi_dir: &str, bindings_dir: &str) -> Result<()> {
    let abi_path = Path::new(abi_dir).join(format!("{}.json", abi_file_name));
    let contract_name = format!("{}_mod", abi_file_name);
    let output_path = Path::new(bindings_dir).join(format!("{}.rs", abi_file_name.to_lowercase()));

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

    // Generate the binding if necessary
    if should_generate {
        Abigen::new(
            contract_name,
            abi_path.to_str().ok_or_else(|| eyre::eyre!("Failed to convert path to string"))?,
        )?
        .generate()?
        .write_to_file(&output_path)?;
    }

    Ok(())
}

// Update the mod.rs file with a new module declaration
fn update_mod_file(abi_file_name: &str, mod_file_path: &PathBuf) -> Result<()> {
    // Create the module declaration
    let module_declaration = format!(
        "pub mod {};\npub use {}::*;\n",
        abi_file_name.to_lowercase(),
        abi_file_name.to_lowercase()
    );

    // Open the mod.rs file in read-only mode
    let mod_rs_file = File::open(mod_file_path)?;
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

    // If the module declaration is not present, append it to the file
    if !found {
        let mut mod_rs_file = OpenOptions::new()
            .append(true)
            .open(mod_file_path)
            .map_err(|_| eyre::eyre!("Could not open mod.rs file"))?;
        if mod_rs_file.metadata()?.len() > 0 {
            mod_rs_file.write_all(b"\n")?;
        }
        mod_rs_file
            .write_all(module_declaration.as_bytes())
            .map_err(|_| eyre::eyre!("Could not write to mod.rs file"))?;
    }

    Ok(())
}

fn ensure_directory_exists(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

fn ensure_file_exists(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        File::create(path)?;
    }

    Ok(())
}
