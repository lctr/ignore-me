#![allow(unused)]

mod args;
mod assets;
mod data;
mod file_gen;
mod term;

use args::{Cli, Commands};
use assets::AssetDB;
use file_gen::FileGen;

fn main() -> std::io::Result<()> {
    let cli = args::Cli::get();
    let debug = cli.debug();
    if debug {
        println!("[DEBUG] debug mode activated");
    }
    match cli.command() {
        Some(cmd) => match cmd {
            Commands::Add { .. } => {
                eprintln!("The command `add` is not yet implemented :(")
            }
            Commands::Terms { .. } => {
                eprintln!("The command `terms` is not yet implemented :(")
            }
            Commands::For { names } => handle_for(debug, names)?,
        },
        None => {
            eprintln!("Functionality handling empty command is not yet implemented :(")
        }
    };
    Ok(())
}

fn handle_for(debug: bool, names: &[String]) -> std::io::Result<()> {
    if names.is_empty() {
        eprintln!("No names provided. Aborting...");
        return Ok(());
    }
    if debug {
        println!("[DEBUG] searching `.gitignore` files for...");
        for name in names.iter() {
            println!("    {name}");
        }
    }
    let asset_db = AssetDB::new();
    let assets = asset_db.get_by_names(names).cloned().collect::<Vec<_>>();
    if assets.is_empty() {
        eprintln!("No assets were found for the following names: ");
        for name in names.iter() {
            eprintln!("    {name}")
        }
        eprintln!("Aborting...");
        return Ok(());
    }
    if debug {
        println!("[DEBUG] found `.gitignore` files for...");
        for asset in assets.iter() {
            println!("    {asset}");
        }
    }
    let file_gen = FileGen::with_assets(assets);
    let bytes_written = file_gen.write_to_target()?;
    println!(
        "Success! {} bytes were written to `{}`",
        bytes_written,
        file_gen.get_target_path()?.display()
    );
    Ok(())
}
