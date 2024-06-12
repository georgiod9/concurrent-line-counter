mod file_processor;
mod server;

fn main() {
    server::start_server("127.0.0.1:7878");
}
