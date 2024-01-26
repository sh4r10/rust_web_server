use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4000").unwrap();
    let pool = ThreadPool::new(8);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    let (status, file) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "public/index.html"),
        "GET /style.css HTTP/1.1" => ("HTTP/1.1 200 OK", "public/style.css"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "public/404.html"),
    };

    let content = fs::read_to_string(file).unwrap();

    let response = format!(
        "{status}\r\nContent-Length: {}\r\n\r\n{content}",
        content.len()
    );
    stream.write_all(response.as_bytes()).unwrap();
}
