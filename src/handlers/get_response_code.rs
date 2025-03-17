pub fn get_response_code(status_code: u16) -> String {

    let response_line = match status_code {
        200 => "HTTP/1.1 200 OK",
        400 => "HTTP/1.1 400 Bad Request",
        404 => "HTTP/1.1 404 Not Found",
        500 => "HTTP/1.1 500 Internal Server Error",
        _ => "HTTP/1.1 500 Internal Server Error", // Default for undefined codes
    };

    return response_line.to_string();
}
