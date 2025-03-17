use std::net::TcpStream;
use std::io::{BufReader, BufRead};
 
pub fn get_request(stream: TcpStream) -> (TcpStream, Vec<String>, String, String, String) {
    let buf_reader = BufReader::new(&stream);

    let _http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| match result {
        Ok(s) => s,
        Err(e) => {
            println!("Error while unwrapping http request {}", e);
            return "".to_string();
        }
    })
    .take_while(|line| !line.is_empty())
    .collect();
    
    let request_line = _http_request.first().cloned().unwrap_or_else(|| {
        "".to_string()
    });
    
    let mut method = "".to_string();
    let mut path = "".to_string();
    let mut protocol = "".to_string();

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() >= 3 {
        method = parts[0].to_string();
        path = parts[1].to_string();
        protocol = parts[2].to_string();
    }
    return (stream, _http_request, method, path, protocol);
}
