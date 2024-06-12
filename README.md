# Concurrent Line Counter with TCP Server

## Overview

This application counts the number of lines in a file concurrently and
provides a TCP server for handling client requests.

## Features

- Concurrent file processing for line counting
- TCP server to accept file paths and return line counts
- Comprehensive error handling and performance optimization

## Performance Optimizations

1. Thread Pool:

   - Used `threadpool` cargo crate
   - Implemented `threadpool` in `file_processor.rs` to manage threads more efficiently
   - No need to create/destroy threads for each chunk for each file anymore
   - Reuse threads across other tasks

2. Chunk Size: (Already existed)
   - Determine chunk size for file dynamically based on number of lines read from buffer

## Usage

1. Build and run the application:

```sh
cargo run
```

Connect to the server and send a file path:

```sh
nc 127.0.0.1 7878
samples/sample_text_1.txt
```

## Testing

Run the unit and integration tests:

```sh
cargo test
```
