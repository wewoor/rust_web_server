use std::{
    fs,
    thread,
    time::{Duration},
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

pub fn single_web_server() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get_req = b"GET / HTTP/1.1\r\n";
    let sleep_req = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get_req) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep_req) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{} \r\nContent-Length:{}\r\n\r\n{}", 
        status_line, 
        contents.len(), 
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
