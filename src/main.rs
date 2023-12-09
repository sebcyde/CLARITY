mod get_dirs;
mod get_input;
mod setup;

use crate::get_dirs::get_dirs::get_config_root;
use crate::setup::setup::set_config_files;
use std::{
    path::{Path, PathBuf},
    process,
};

#[tokio::main]
async fn main() {
    println!("\nStarting CLARITY...");
    println!("Retreiving config...");

    let config_dir_path: PathBuf = get_config_root();
    if !Path::new(&config_dir_path).exists() {
        println!("No config file detected.");
        set_config_files().await
    }

    println!("Welcome. Starting watch.");
    // let config_data = get_config_data().await;

    println!("Stopping...");
    process::exit(0);

    // TODO-LIST
    // - DONE - Config Creation (Initial User Setup)
    // - AutoSort Functionality
    // - Storage optimisations
}
