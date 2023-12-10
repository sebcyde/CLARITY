mod configuration;
mod get_dirs;
mod get_input;
mod watch;

use crate::configuration::configuration::setup_config;
use crate::watch::watch::start_watch;
use std::process;

#[tokio::main]
async fn main() {
    println!("\nStarting CLARITY...");

    setup_config().await;
    start_watch().await;

    println!("Stopping...");
    process::exit(0);

    // TODO-LIST
    // - DONE - Config Creation (Initial User Setup)
    // - AutoSort Functionality
    // - Storage optimisations
}
