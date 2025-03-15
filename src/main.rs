use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    println!("Welcome to simple http Server!");

    // bind the port and initialize tcp TcpListener
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // creating the loop for http request
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection!");

                // Read the first 1024 bytes from the stream
                // Creating buffer
                let mut buffer = [0; 1024];

                // Begin the read from the stream
                let bytes_read = stream.read(&mut buffer).unwrap();
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                // Extract the path from the request
                let request_line = request.lines().next().unwrap_or("");
                let parts: Vec<&str> = request_line.split_whitespace().collect();

                // Default to "/" if the path is not specified
                let path: &str = if parts.len() > 1 { parts[1] } else { "/" };

                println!("received request: {}", path);

                match path {
                    "/" => {
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                    _ => {
                        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to accept client {}", e);
            }
        }
    }
}
