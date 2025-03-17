use std::net::TcpListener;
use threadpool::ThreadPool;
mod handlers;

use handlers::handle_connection;
use crate::handle_connection::handle_connection;

use handlers::logo_display;
use crate::logo_display::logo_display;

fn main() {

    const PORT: &str = "8080";
    const ADDRESS: &str = "127.0.0.1";
    let address_and_port = format!("{}:{}", ADDRESS, PORT);
    let pool = ThreadPool::new(4);
    
    let listener = TcpListener::bind(address_and_port)
    .unwrap_or_else(|e| {
        panic!(
            "Cannot open server on PORT: {}: {}", 
            PORT, 
            e
        );
    });

    logo_display();
    println!("Running on Port : {}", PORT);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("Error while unwrapping stream {}",e);
                return;
            }
        };

        pool.execute(move || {
            handle_connection(stream);
        });
    };
}


