pub mod setup {
    use serde::Serialize;
    use serde_json::Value;

    use crate::get_dirs::get_dirs::{get_config_file, get_config_root};
    use crate::get_input::input::{get_input, send_output};
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[derive(Serialize)]
    struct UserConfig {
        user_name: String,
        watch_documents: bool,
        watch_downloads: bool,
    }

    pub async fn set_config_files() {
        println!("Starting initial setup.");
        send_output("Welcome to Clarity! What's your name?").await;
        let user_name = get_input().await;
        create_config_file(&user_name.unwrap()).await;
    }

    pub async fn get_config_data() -> Value {
        let config_path: PathBuf = get_config_file();
        println!("config_path: {:?}", config_path);
        let config_value: String = read_to_string(config_path).unwrap();
        let config: Value = serde_json::from_str(&config_value).expect("Failed to deserialize.");
        return config;
    }

    async fn create_config_file(name: &str) {
        let config_path: PathBuf = get_config_root();
        std::fs::create_dir_all(&config_path).unwrap();

        let config_path: PathBuf = get_config_file();
        let mut config_file: File = File::create(&config_path).unwrap();

        let user_config: UserConfig = UserConfig {
            user_name: name.trim_end_matches(&['\r', '\n'][..]).to_owned(),
            watch_documents: false,
            watch_downloads: true,
        };

        let json_data: String = serde_json::to_string(&user_config).unwrap();
        _ = config_file.write_all(json_data.as_bytes());
    }
}
