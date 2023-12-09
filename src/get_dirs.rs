pub mod get_dirs {
    use dirs::{config_dir, document_dir, download_dir};
    use std::path::PathBuf;

    pub fn get_config_root() -> PathBuf {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("Clarity");
        return config_root;
    }

    pub fn get_config_file() -> PathBuf {
        let mut config_root: PathBuf = config_dir().unwrap();
        config_root.push("Clarity");
        config_root.push("config.json");
        return config_root;
    }

    pub fn get_downloads_dir() -> PathBuf {
        let downloads_dir: PathBuf = download_dir().unwrap();
        return downloads_dir;
    }

    pub fn get_documents_dir() -> PathBuf {
        let documents_dir: PathBuf = document_dir().unwrap();
        return documents_dir;
    }
}
