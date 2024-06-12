use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let file_path = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
            println!("[server.rs]: File Path: {}", file_path);

            match crate::file_processor::process_file(&file_path) {
                Ok(line_count) => {
                    println!("[server.rs]: Line Count: {}", line_count);

                    let response = format!("Line count: {}", line_count);
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("[server.rs]: Caught exception: {}", e);

                    let error_message = format!("Error processing file: {}", e);
                    stream.write_all(error_message.as_bytes()).unwrap();
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
        }
    }
}

pub fn start_server(address: &str) {
    let listener = TcpListener::bind(address).expect("Could not bind to address");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
