use std::net::TcpStream;
use std::io::Write;
use crate::handlers::generate_response::generate_response;
use crate::handlers::router::router;

pub fn handle_response(mut stream: TcpStream, method: String, path: String) {
    let (html_response, contents) = generate_response(router(&method, &path));

    stream.write_all(html_response.as_bytes()).unwrap();
    match contents {
        Some(contents) => {
            if let Err(e) = stream.write_all(&contents) {
                eprintln!("Failed to send contents to stream! Path: {} | Method: {} | Error: {}", path, method, e);
                
                // Send an internal server error response
                let (html_response, _) = generate_response("internal_server_error".to_string());
                if let Err(err) = stream.write_all(html_response.as_bytes()) {
                    eprintln!("Critical failure: Unable to send error response! {}", err);
                }
            }        
        }
        None => {
            return; 
        }
    }

}
