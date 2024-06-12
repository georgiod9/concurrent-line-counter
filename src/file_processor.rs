use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn process_file(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    let chunk_size = (lines.len() / 4).max(1);
    let line_count = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let line_count = Arc::clone(&line_count);
        let handle = thread::spawn(move || {
            let count = chunk.len();
            let mut line_count = line_count.lock().unwrap();
            *line_count += count;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let line_count = Arc::try_unwrap(line_count).expect("Lock still has multiple owners");
    let line_count = line_count.into_inner().expect("Mutex cannot be locked");
    Ok(line_count)
}
