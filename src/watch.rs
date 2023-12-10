pub mod watch {
    use crate::{
        configuration::configuration::{get_config_data, UserConfig},
        editor::editor::process_file,
        get_dirs::get_dirs::get_downloads_dir,
    };
    use notify::{Config, ReadDirectoryChangesWatcher, RecommendedWatcher, RecursiveMode, Watcher};
    use std::path::{Path, PathBuf};

    pub async fn start_watch() {
        println!("Welcome. Starting watch.");

        let config_data: UserConfig = get_config_data().await;
        // let is_watching_docouments: bool = config_data.watch_settings.watch_documents;
        let is_watching_downloads: bool = config_data.watch_settings.watch_downloads;
        let downloads_path: PathBuf = get_downloads_dir();

        loop {
            if is_watching_downloads {
                _ = watch_folder(&downloads_path)
            }
        }
    }

    fn watch_folder<P: AsRef<Path>>(path: P) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Create new watcher
        let mut watcher: ReadDirectoryChangesWatcher =
            RecommendedWatcher::new(tx, Config::default())?;

        // Path to be watched - Non-Recursive Mode
        watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

        Ok(for res in rx {
            match res {
                Ok(event) => match event.kind {
                    notify::EventKind::Access(a) => {
                        println!("Access Event: {:?}.", a)
                    }
                    notify::EventKind::Create(c) => {
                        println!("\nCreate Event Triggered.\n");
                        process_file(event.paths[0].to_owned());
                    }
                    notify::EventKind::Modify(m) => {
                        // println!("Modify Event: {:?}.", m)
                    }
                    notify::EventKind::Remove(r) => {
                        // println!("Remove Event: {:?}.", r)
                    }
                    _ => println!("Unreachable Code."),
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        })
    }
}
