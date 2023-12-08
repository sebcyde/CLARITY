pub mod setup {
    use crate::get_input::input::{get_input, send_output};

    pub async fn set_config_files() {
        println!("Starting setup.");

        send_output("Welcome to Clarity! What's your name?");
        let user_name: String = get_input().await;
        set_user(&user_name).await;
    }
    pub async fn get_config_files() {}
    pub async fn set_user(name: &str) {
        // Set username in the config file
    }
    pub async fn get_user() {}
}
