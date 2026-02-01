use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

enum TagType {
    H1,
    P,
    Div,
}

struct Element {
    tag: TagType,
    content: String,
}

impl Element {
    fn render(&self) -> String {
        let tag = match self.tag {
            TagType::H1 => "h1",
            TagType::P => "p",
            TagType::Div => "div",
        };

        format!("<{}>{}</{}>", tag, self.content, tag)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer).unwrap();

    let my_title = Element {
        tag: TagType::H1,
        content: String::from("Software Architect with Rust"),
    };

    let response_body = my_title.render();

    let response = format!(
        "HTTP/1.1 200 OK\r\n\
        Content-Type: text/html; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\
        \r\n\
        {}",
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Couldn't connnect with port 8080");

    println!("Server running on http://127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => println!("Conexion error: {}", e),
        }
    }
}
