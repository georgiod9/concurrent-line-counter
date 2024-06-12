use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use threadpool::ThreadPool;

pub fn process_file(file_path: &str) -> io::Result<usize> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Error opening file: {}", e),
            ))
        }
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = match reader.lines().collect::<Result<_, _>>() {
        Ok(lines) => lines,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Error reading lines: {}", e),
            ))
        }
    };

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

// More efficient method using thread pool instead of spawning multiple threads
pub fn process_file_with_thread_pool(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let thread_count: usize = 4; // Number of threads
    let chunk_size = (lines.len() / thread_count).max(1); // Determine chunk size dynamically
    let line_count = Arc::new(Mutex::new(0));
    let pool = ThreadPool::new(thread_count); // Use a pool of `thread_count`

    for chunk in lines.chunks(chunk_size) {
        let chunk = chunk.to_owned();
        let line_count = Arc::clone(&line_count);
        pool.execute(move || {
            let count = chunk.len();
            let mut line_count = line_count.lock().unwrap();
            *line_count += count;
        });
    }

    pool.join();
    let line_count = Arc::try_unwrap(line_count).expect("Lock still has multiple owners");
    let line_count = line_count.into_inner().expect("Mutex cannot be locked");
    Ok(line_count)
}
