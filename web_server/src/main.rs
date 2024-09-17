use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    // bind can fail if there's already a proc bound to the port, or if the port
    // is < 1024 and we aren't root
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    // incoming returns an iterator of TcpStream. A single stream represents an
    // open connection between the client and server
    for stream in listener.incoming().take(2) {
        // incoming returns connection attempts - a connection might fail for
        // many reasons, e.g. an OS limit of the number of simult. open connections
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
    // we only take the first 2 connections so we can see the graceful shutdown
    // when pool goes out of scope
}

fn handle_connection(mut stream: TcpStream) {
    // BufReader implements the std::io::BufRead trait
    let buf_reader = BufReader::new(&stream);
    let req_line = buf_reader
        // lines returns the lines of the req sent by the browser
        .lines()
        .next()
        .unwrap()
        .unwrap();
    let (status_line, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    // println!("Req: {:#?}", http_req);
    // let res = "HTTP/1.1 200 OK\r\n\r\n";
    let body = fs::read_to_string(filename).unwrap();
    let headers = format!("Content-Length: {}", body.len());
    let res = format!("{status_line}\r\n{headers}\r\n\r\n{body}");
    stream.write_all(res.as_bytes()).unwrap();
    // stream goes out of scope, and the connection is closed as part of
    // its Drop implementation.
}