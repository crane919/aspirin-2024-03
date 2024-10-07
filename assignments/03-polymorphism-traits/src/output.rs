use colored::Color;

pub trait Output {
    fn output(&self, final_lines: Vec<String>);
}

struct PlainOutput {}
struct ColorOutput {
    color: Color,
    needle: String,
}

impl Output for PlainOutput {
    fn output(&self, final_lines: Vec<String>) {
        for line in final_lines {
            println!("{}", line);
        }
    }
}

impl Output for ColorOutput {
    fn output(&self, final_lines: Vec<String>) {
        !todo!()
        // All my clippy warnings are because I decieded not finish this code :(
    }
}

/// Print out input line by line and highlight substring is specified
pub fn print_output(final_lines: Vec<String>, color: Option<Color>, needle: String) {
    let output: Box<dyn Output> = match color {
        Some(c) => Box::new(ColorOutput { color: c, needle }),
        None => Box::new(PlainOutput {}),
    };
    output.output(final_lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_output() {
        let lines = vec![
            "This is a test line.".to_string(),
            "Another line without the keyword.".to_string(),
        ];
        let output = PlainOutput {};
        output.output(lines);
    }
}
