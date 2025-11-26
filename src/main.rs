use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("listening on port 4000");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        if let Err(e) = handle_connection(stream) {
            eprint!("failed to establish a connection: {}", e);
        }
    }
}

// todo: return Result, handle the code in a more readable way
fn handle_connection(stream: TcpStream) -> Result<(), std::io::Error> {
    let buf_reader = BufReader::new(&stream);
    let mut http_request = Vec::new();

    for line_result in buf_reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }
    println!("Request: {http_request:#?}");
    Ok(())
}
