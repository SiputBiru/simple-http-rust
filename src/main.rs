use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Welcome to simple http Server!");

    // bind the port and initialize tcp TcpListener
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // creating the loop for http request
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection!");
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Failed to accept client Error: {}", e);
            }
        }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_req: Vec<_> = buf_reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_req.get(0).map(String::as_str).unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let path: &str = if parts.len() > 1 { parts[1] } else { "/" };

    println!("received request: {}", path);

    match path {
        "/" => {
            let status_line = "HTTP/1.1 200 OK\r\n";
            let contents = fs::read_to_string("index.html").unwrap();
            let length = contents.len();
            let response = format!(
                "{status_line}Content-Type: text/html\r\nContent-Length: {length}\r\n\r\n{contents}"
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
        _ => {
            let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
    }
}