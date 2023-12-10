pub mod editor {
    use std::fs::{copy, File};
    use std::ops::DerefMut;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::Duration;

    use infer::{MatcherType, Type};

    use crate::get_dirs::get_dirs::get_clarity_directory;
    use crate::notifications::notifications::send_notif;

    pub fn process_file(file_path: PathBuf) {
        println!("Processing File...");
        println!("Source Path: {:?}\n", &file_path);

        let file_type_option: Option<Type> = infer_type(&file_path);
        let mut clarity_dir: PathBuf = get_clarity_directory();

        match file_type_option {
            Some(file_type) => {
                println!("\nFile Type: {}", &file_type);
                println!("File Matcher Type: {:?}\n", &file_type.matcher_type());

                let processed_file_type: &str = match file_type.matcher_type() {
                    MatcherType::Image => "Image",
                    MatcherType::Video => "Video",
                    MatcherType::Audio => "Audio",
                    MatcherType::Archive => "Archive",
                    MatcherType::Book => "Book",
                    MatcherType::Doc => "Doc",
                    MatcherType::Font => "Font",
                    MatcherType::App => "App",
                    MatcherType::Text => "Text",
                    MatcherType::Custom => "Custom",
                };

                // File -> Target Dir
                let file_name: &str = file_path.file_name().unwrap().to_str().unwrap();
                let target_path: PathBuf = clarity_dir.join(&processed_file_type).join(file_name);
                _ = std::fs::copy(&file_path, target_path);

                // Send Notification
                let content: String =
                    format!("Moved file to '{:?}' directory.", &processed_file_type);
                send_notif(&content);
            }
            None => {
                println!("No File Type Detected\n");
                let file_name: &str = file_path.file_name().unwrap().to_str().unwrap();
                let target_path: PathBuf = clarity_dir.join("Other").join(file_name);
                _ = std::fs::copy(&file_path, target_path);
                send_notif("Failed to process. Moved file to 'Other' directory.");
            }
        }
    }

    fn infer_type(file_path: &PathBuf) -> Option<Type> {
        std::thread::sleep(Duration::from_millis(100));
        return infer::get_from_path(file_path).unwrap();
    }
}
