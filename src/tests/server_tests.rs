#[cfg(test)]
mod tests {
    use crate::server;
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;

    #[test]
    fn test_server() {
        thread::spawn(|| {
            server::start_server("127.0.0.1:7878");
        });

        let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
        stream.write_all(b"data/test.txt").unwrap();
        let mut buffer = [0; 128];
        let size = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..size]);
        assert!(response.contains("Line count: 4"));
    }
}
