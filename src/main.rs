use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::{mpsc, Arc, Mutex};


enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute
    <F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
fn main() {
    println!("Welcome to simple http Server!");

    let pool = ThreadPool::new(4);

    // bind the port and initialize tcp TcpListener
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    // creating the loop for http request
    for stream in listener.incoming() {
        
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_connection(stream);
                });
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
