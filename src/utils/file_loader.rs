use std::fs;

pub fn read_logs_from_folder(folder: &str) -> String {
    let mut all_logs = String::new();

    let paths = fs::read_dir(folder).expect("Failed to read folder");

    for path in paths {
        let entry = path.unwrap();
        let file_path = entry.path();

        if let Some(ext) = file_path.extension() {
            if ext == "log" {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    println!("Loaded: {}", file_path.display());
                    all_logs.push_str(&content);
                    all_logs.push('\n');
                }
            }
        }
    }

    all_logs
}
