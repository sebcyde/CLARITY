pub mod watch {
    use crate::configuration::configuration::{get_config_data, UserConfig};
    use notify::{Config, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher};
    use std::path::Path;

    pub async fn start_watch() {
        println!("Welcome. Starting watch.");
        let config_data: UserConfig = get_config_data().await;

        println!("User Name Config: {}", config_data.user_data.user_name);
        println!(
            "Watch Downloads Config: {}",
            config_data.watch_settings.watch_downloads
        );
        println!(
            "Watch Docs Config: {}",
            config_data.watch_settings.watch_documents
        );

        println!("Sort By Date: {}", config_data.watch_settings.sort_by_date);

        // loop {

        // }
    }

    fn watch_folder<P: AsRef<Path>>(path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Create new watcher
        let mut watcher: ReadDirectoryChangesWatcher =
            RecommendedWatcher::new(tx, Config::default())?;

        // Path to be watched - Non-Recursive Mode
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        // for res in rx {
        //     match res {
        //         Ok(event) => {}
        //         Err(e) => println!("watch error: {:?}", e),
        //     }

        Ok(())
        // }
    }
}
