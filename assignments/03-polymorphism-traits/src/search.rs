use regex::Regex;

/// Search lines for substring, check for flags
pub fn search_lines(
    lines: Vec<String>,
    needle: &str,
    ignore_case: bool,
    invert_match: bool,
) -> Vec<String> {
    let search_needle: Box<dyn Needle> = match (invert_match, Regex::new(needle).is_ok()) {
        (true, true) => Box::new(RegexStringInvert {}),
        (true, false) => Box::new(NormalStringInvert {}),
        (false, true) => Box::new(RegexString {}),
        (false, false) => Box::new(NormalString {}),
    };

    search_needle.find_lines(lines, needle, ignore_case)
}

pub trait Needle {
    fn find_lines(&self, lines: Vec<String>, needle: &str, ignore_case: bool) -> Vec<String>;
}

struct NormalString {}
struct NormalStringInvert {}
struct RegexString {}
struct RegexStringInvert {}

impl Needle for NormalString {
    fn find_lines(&self, lines: Vec<String>, needle: &str, ignore_case: bool) -> Vec<String> {
        if ignore_case {
            let needle_lower = needle.to_lowercase();
            lines
                .into_iter()
                .filter(|line| line.to_lowercase().contains(&needle_lower))
                .collect()
        } else {
            lines
                .into_iter()
                .filter(|line| line.contains(needle))
                .collect()
        }
    }
}

impl Needle for RegexString {
    fn find_lines(&self, lines: Vec<String>, needle: &str, ignore_case: bool) -> Vec<String> {
        let regex = if ignore_case {
            Regex::new(&format!("(?i){}", needle)).expect("Invalid regex pattern")
        } else {
            Regex::new(needle).expect("Invalid regex pattern")
        };

        lines
            .into_iter()
            .filter(|line| regex.is_match(line))
            .collect()
    }
}

impl Needle for NormalStringInvert {
    fn find_lines(&self, lines: Vec<String>, needle: &str, ignore_case: bool) -> Vec<String> {
        if ignore_case {
            let needle_lower = needle.to_lowercase();
            lines
                .into_iter()
                .filter(|line| !line.to_lowercase().contains(&needle_lower))
                .collect()
        } else {
            lines
                .into_iter()
                .filter(|line| !line.contains(needle))
                .collect()
        }
    }
}

impl Needle for RegexStringInvert {
    fn find_lines(&self, lines: Vec<String>, needle: &str, ignore_case: bool) -> Vec<String> {
        let regex = if ignore_case {
            Regex::new(&format!("(?i){}", needle)).expect("Invalid regex pattern")
        } else {
            Regex::new(needle).expect("Invalid regex pattern")
        };

        lines
            .into_iter()
            .filter(|line| !regex.is_match(line))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_string_search() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
        ];

        let result = search_lines(lines.clone(), "test", false, false); // Case-sensitive search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"This is a test line.".to_string()));
        assert!(result.contains(&"Test again with test.".to_string()));
    }

    #[test]
    fn test_normal_string_search_ignore_case() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
        ];

        let result = search_lines(lines.clone(), "TEST", true, false); // Case-insensitive search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"This is a test line.".to_string()));
        assert!(result.contains(&"Test again with test.".to_string()));
    }

    #[test]
    fn test_normal_string_invert() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
        ];

        let result = search_lines(lines.clone(), "test", false, true); // Inverted search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"Another line without the keyword.".to_string()));
        assert!(result.contains(&"No matches here.".to_string()));
    }

    #[test]
    fn test_normal_string_invert_ignore_case() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
        ];

        let result = search_lines(lines.clone(), "TEST", true, true); // Inverted case-insensitive search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"Another line without the keyword.".to_string()));
        assert!(result.contains(&"No matches here.".to_string()));
    }

    #[test]
    fn test_regex_search() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
            "This line contains the number 123.".to_string(),
        ];

        let result = search_lines(lines.clone(), r"\btest\b", false, false); // Case-sensitive regex search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"This is a test line.".to_string()));
        assert!(result.contains(&"Test again with test.".to_string()));
    }

    #[test]
    fn test_regex_search_ignore_case() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
            "This line contains the number 123.".to_string(),
        ];

        let result = search_lines(lines.clone(), r"\bTEST\b", true, false); // Case-insensitive regex search
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"This is a test line.".to_string()));
        assert!(result.contains(&"Test again with test.".to_string()));
    }

    #[test]
    fn test_regex_invert() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
            "This line contains the number 123.".to_string(),
        ];

        let result = search_lines(lines.clone(), r"\btest\b", false, true); // Inverted regex search
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"Another line without the keyword.".to_string()));
        assert!(result.contains(&"No matches here.".to_string()));
        assert!(result.contains(&"This line contains the number 123.".to_string()));
    }

    #[test]
    fn test_regex_invert_ignore_case() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
            "Test again with test.".to_string(),
            "No matches here.".to_string(),
            "This line contains the number 123.".to_string(),
        ];

        let result = search_lines(lines.clone(), r"\bTEST\b", true, true); // Inverted case-insensitive regex search
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"Another line without the keyword.".to_string()));
        assert!(result.contains(&"No matches here.".to_string()));
        assert!(result.contains(&"This line contains the number 123.".to_string()));
    }

    #[test]
    fn test_empty_lines() {
        let lines: Vec<String> = vec![];
        let result = search_lines(lines.clone(), "test", false, false);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_no_matches() {
        let lines = vec!["This is a line.".to_string(), "Another line.".to_string()];

        let result = search_lines(lines.clone(), "test", false, false);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_invalid_regex() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
        ];

        let result = search_lines(lines.clone(), "[a-z", false, false); // Invalid regex
        assert_eq!(result.len(), 0);
    }
}
