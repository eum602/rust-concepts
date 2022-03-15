use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); //bind is like the "new" keyword in other languages

    for _stream in listener.incoming() {
        let _stream = _stream.unwrap(); // not handling erros - just uisng unwrap to stop the program if errors happens
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // mutable to allow the TcpStream handle the data it returns to us properly ...
    let mut buffer = [0; 1024]; // holds the data that is gonna read in ... 1024 bytes ~ to hold the data of a basic request
                                //for arbitrary data we would need more comple buffer management
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n"; //transforming into a byte string
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}", //1. 'length' to ensure a valid response
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap(); //.write method sends the bytes directly to the connection
                                                //also simply using "unwrap" instead of properly handling the errors
    stream.flush().unwrap(); //forces to wait until all bytes are wrtten to the connection
}