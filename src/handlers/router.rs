
use std::fs;
use std::path::Path;

pub fn router(method: &str, path: &str) -> String {
    if method == "GET" {
        let public_dir = Path::new("public");

        if let Ok(entries) = fs::read_dir(public_dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_path = file_name.to_string_lossy().to_string();

                if path == "/" {
                    return "index.html".to_string();
                }

                if path == format!("/{}", file_path) {
                    return file_path; 
                }
            }
        }

        // Default route for unknown paths
        return "not_found".to_string(); 
    }

    "bad_request".to_string() 
}

