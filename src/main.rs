use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;

enum TagType { H1, P }

struct Element {
    tag: TagType,
    content: String,
}

impl Element {
    fn render(&self) -> String {
        let tag = match self.tag {
            TagType::H1 => "h1",
            TagType::P => "p",
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

    let description = Element {
        tag: TagType::P,
        content: String::from("Handmade server running ..."),
    };

    let response_body = format!(
        "<!DOCTYPE html><html><body>{} {}</body></html>",
        my_title.render(),
        description.render()
    );

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
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("0.0.0.0:{}", port);

    let listener = TcpListener::bind(&address).expect("Couldn't connnect with port 8080");

    println!("Server running on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => println!("Conexion error: {}", e),
        }
    }
}
