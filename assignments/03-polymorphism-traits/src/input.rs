use anyhow::Result;
use std::{
    fs::File,
    io::{self, BufRead},
    path::PathBuf,
};

pub trait Input {
    fn input(&self) -> Result<Vec<String>>;
}

struct StandardInput {}
struct FileInput {
    file_path: PathBuf,
}

impl Input for StandardInput {
    fn input(&self) -> Result<Vec<String>> {
        let reader = io::stdin().lock();

        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(lines)
    }
}

impl Input for FileInput {
    fn input(&self) -> Result<Vec<String>> {
        let file = File::open(&self.file_path)?;
        let reader = io::BufReader::new(file);

        let lines = reader.lines().collect::<Result<Vec<_>, _>>()?;
        Ok(lines)
    }
}

/// Grab the input from a file or stdin
pub fn get_input_dyn(file: Option<PathBuf>) -> Result<Vec<String>> {
    let input: Box<dyn Input> = match file {
        None => Box::new(StandardInput {}),
        Some(file_path) => Box::new(FileInput { file_path }),
    };
    input.input()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn simulate_stdin(input: &str) -> Vec<String> {
        let reader = Cursor::new(input);
        let lines = reader
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to read from simulated stdin");
        lines
    }

    #[test]
    fn test_standard_input() {
        // Simulate standard input
        let input = "line 1\nline 2\nline 3\n";
        let result = simulate_stdin(input);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line 1");
        assert_eq!(result[1], "line 2");
        assert_eq!(result[2], "line 3");
    }

    // Path to the test file
    const TEST_FILE: &str = "lots_of_words.txt";

    #[test]
    fn test_file_input() {
        // Use lots_of_words.txt for tests
        let path = PathBuf::from(TEST_FILE);

        assert!(path.exists(), "Test file does not exist!");
        let result = get_input_dyn(Some(path)).expect("Failed to read from file");
        assert!(!result.is_empty());
        assert!(result
            .contains(&"-rw-rw-r-- 1 mcranor mcranor 10524 Oct  2 18:50 Cargo.lock".to_string()));
        assert!(result.contains(&"banana boats are cool".to_string()));
        assert!(result.contains(&"i love bananas".to_string()));
    }
}
