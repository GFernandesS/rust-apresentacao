use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Conexão estabelecida");

        handle_requests(stream);
    }
}

fn handle_requests(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/hello.html")
    } else if request_line == "GET /json HTTP/1.1" {
        ("HTTP/1.1 200 OK", "./src/response.json")
    }
    else{
        ("HTTP/1.1 404 NOT FOUND", "./src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let content_type = if filename.contains("json") {"application/json"} else {"text/html"};

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {content_type} \r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}