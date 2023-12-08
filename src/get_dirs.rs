pub mod get_dirs {
    use dirs::{config_dir, download_dir};
    use std::path::PathBuf;

    pub fn get_config_root() -> PathBuf {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("Clarity");
        return config_root;
    }

    pub fn get_downloads_dir() -> PathBuf {
        let downloads_dir: PathBuf = download_dir().unwrap();
        return downloads_dir;
    }
}