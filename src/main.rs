mod get_dirs;
mod get_input;
mod setup;

use crate::get_dirs::get_dirs::get_config_root;
use crate::get_input::input::{get_input, send_output};
use crate::setup::setup::set_config_files;
use std::path::Path;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("Hello, from CLARITY!");

    let config_dir_path: PathBuf = get_config_root();
    if !Path::new(&config_dir_path).exists() {
        set_config_files().await
    }

    println!("CONFIG ROOT: {:?}", &config_dir_path);

    // TODO-LIST
    // - Config Creation (Initial User Setup)
    // - AutoSort Functionality
    // - Storage optimisations
}
