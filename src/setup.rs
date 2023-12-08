pub mod setup {
    use serde::Serialize;
    use toml::to_string;

    use crate::get_dirs::get_dirs::{get_config_file, get_config_root};
    use crate::get_input::input::{get_input, send_output};
    use std::fs::read_to_string;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[derive(Serialize)]
    struct TomlConfig {
        user_name: String,
    }

    pub async fn set_config_files() {
        println!("Starting initial setup.");
        send_output("Welcome to Clarity! What's your name?").await;
        let potential_user_name: Result<String, std::io::Error> = get_input().await;
        create_config_file(&potential_user_name.unwrap()).await;
    }

    pub async fn get_config_files() {
        let config_path: PathBuf = get_config_file();

        let config_str = read_to_string(config_path).expect("Failed to read config file");

        println!("Config: {:?}", config_str);

        // let cargo_toml: CargoToml =
        //     toml::from_str(&config_string).expect("Failed to deserialize Cargo.toml");

        // println!("{:#?}", cargo_toml);
    }

    // pub async fn get_user() {
    //     let config_path: PathBuf = get_config_file();
    //     println!("{:?}", config_path);
    // }

    async fn create_config_file(name: &str) {
        let config_path = get_config_file();
        let mut config_file = File::create(&config_path).unwrap();
        let config = TomlConfig {
            user_name: name.to_owned(),
        };
        let toml_string = to_string(&config).unwrap();
        _ = config_file.write_all(toml_string.as_bytes());
    }
}
