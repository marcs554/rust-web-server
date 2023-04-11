use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader},
    thread::spawn
};



fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap();

    for data in listener.incoming() {
        let data = data.unwrap();

        spawn(|| {
            handler_conn(data);
        });
        
    }
}


fn handler_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    let (status_line, filename) = match &http_request[..] {
        "GET / HTTP/1.1"            => ("HTTP/1.1 200 OK", "index"),
        "GET /redirect HTTP/1.1"    => ("HTTP/1.1 301 MOVED PERMANENTLY", "301"),
        _                           => ("HTTP/1.1 404 NOT FOUND", "404"),
    };

    
    let content = fs::read_to_string(format!("public/{filename}.html")).unwrap();
    let size_content = content.len();  
    let response        = format!("{status_line}\r\nContent-Length: {size_content}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
