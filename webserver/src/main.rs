mod thread_pool;

use std::time::Duration;
use std::thread;
use std::{net::{TcpListener, TcpStream}, io::{prelude::*, BufReader}, fs};

use thread_pool::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7788").unwrap();
    let pools = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Client[{}] connect", stream.peer_addr().unwrap().to_string());
        pools.execute(|| { handle_connection(stream) });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);


    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("Request:{request_line}");

    let (status_line, fn_content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "webserver/res/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "webserver/res/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "webserver/res/404.html"),
    };


    let content = fs::read_to_string(fn_content).unwrap();
    let length = content.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
    );

    println!("Response:{response}");
    stream.write_all(response.as_bytes()).unwrap();
}