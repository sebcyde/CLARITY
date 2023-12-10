mod configuration;
mod editor;
mod get_dirs;
mod get_input;
mod notifications;
mod watch;

use crate::configuration::configuration::setup_config;
use crate::notifications::notifications::send_notif;
use crate::watch::watch::start_watch;

use std::process;

#[tokio::main]
async fn main() {
    send_notif("Starting CLARITY...");

    setup_config().await;
    start_watch().await;

    send_notif("Stopping CLARITY...");

    process::exit(0);

    // TODO-LIST
    //
    // - DONE - Config Creation (Initial User Setup)
    //
    // - AutoSort Functionality
    //
    // - Desktop Notifications
    // --> "Moved X files and Y Directories"
    // --> "Moved X to Y Directory"
    //
    // - Storage optimisations
    // --> File Zipping / Compressing
}
