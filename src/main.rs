use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};

fn handle_client(mut stream: TcpStream) {
    println!("{:?}", stream);

    let mut buf = [0u8; 1024];
    let bytes_read = stream.read(&mut buf).unwrap();

    let request = String::from_utf8_lossy(&buf[..bytes_read]);

    /* get server */
    let html = std::fs::read_to_string("index.html")
        .unwrap();

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html,
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Received {} bytes:\n{}", bytes_read, request);
}

pub fn main() -> std::io::Result<()> {
    let localhost = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(&localhost).
        expect("invalid");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("connection failed: {e}");
            }
        }
    }

    Ok(())
}
