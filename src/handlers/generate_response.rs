
use std::fs;
use std::path::Path;
use crate::handlers::get_response_code::get_response_code;

pub fn generate_response(file: String) -> (String, Option<Vec<u8>>) {
    let directory = "public";
    let path = format!("{}/{}", directory, file);
    let path = Path::new(&path); // Convert file string to a Path

    let response_code = get_response_code(200);
    let mut response;
    let mut contents: Option<Vec<u8>> = None;

    // Extract file extension
    let extension = path.extension().and_then(|ext| ext.to_str());
    
    if extension.is_none() {
        return (get_response_code(404) + "\r\n\r\n", None);
    }
    // Map extensions to MIME types
    let mime_type = match extension {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("json") => "application/json",
        Some("ico") => "image/x-icon",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("txt") => "text/plain",
        Some("internal_server_error") => return (get_response_code(500) + "\r\n\r\n", None),
        Some("bad_request") => return (get_response_code(400) + "\r\n\r\n", None),
        Some("not_found") => return (get_response_code(404) + "\r\n\r\n", None),
        _ => {
            println!("Unsupported file type: {:?}", extension);
            return (get_response_code(404) + "\r\n\r\n", None);
        }
    };

    // Read file contents (text or binary)
    match fs::read(&path) {
        Ok(data) => {
            let contents_length = data.len();
            response = format!(
                "{response_code}\r\nContent-Type: {mime_type}\r\nContent-Length: {contents_length}\r\n\r\n"
            );

            // If it's a text file, convert bytes to string
            if mime_type.starts_with("text/") || mime_type == "application/json" {
                if let Ok(text) = String::from_utf8(data.clone()) {
                    response.push_str(&text);
                }
            } else {
                contents = Some(data);
            }
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", path.display(), e);
            response = format!("{}\r\n\r\n", get_response_code(500));
        }
    }

    (response, contents)
}

