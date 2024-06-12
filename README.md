# Concurrent Line Counter with TCP Server

## Overview

This application counts the number of lines in a file concurrently and
provides a TCP server for handling client requests.

## Features

- Concurrent file processing for line counting
- TCP server to accept file paths and return line counts
- Comprehensive error handling and performance optimization

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
