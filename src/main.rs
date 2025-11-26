use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4000").unwrap();
    println!("listening on port 4000");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("failed to establish a connection: {}", e);
                continue;
            }
        };

        if let Err(e) = handle_connection(stream) {
            eprintln!("failed to handle a connection: {}", e);
        }
    }
}

// todo: return Result, handle the code in a more readable way
fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let buf_reader = BufReader::new(&stream);
    let mut http_request = Vec::new();

    for line_result in buf_reader.lines() {
        let line = line_result?;
        if line.is_empty() {
            break;
        }
        http_request.push(line);
    }
    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string("hello.html")?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
