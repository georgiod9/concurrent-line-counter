#[cfg(test)]
mod tests {
    use crate::file_processor;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_file_processing() {
        let mut file = File::create("data/test.txt").unwrap();
        writeln!(file, "Line 1\nLine 2\nLine 3\nLine 4").unwrap();
        let result = file_processor::process_file("data/test.txt").unwrap();
        assert_eq!(result, 4);
    }
}
