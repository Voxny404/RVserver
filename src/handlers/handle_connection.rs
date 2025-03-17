use crate::handlers::get_request::get_request;
use std::net::TcpStream;
use crate::handlers::handle_response::handle_response;

pub fn handle_connection(stream: TcpStream) {
    let (stream, _http_request, method, path, protocol) = get_request(stream);
    
    //println!("Request: {_http_request:#?}");
    println!("Request : Method: {} Protocol: {} Path:{}", method, protocol, path);

    handle_response(stream, method, path);    
}
