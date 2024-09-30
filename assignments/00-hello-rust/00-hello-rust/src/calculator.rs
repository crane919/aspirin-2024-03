use std::io;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum Operator {
    And,
    Or,
    Xor,
    Invalid,
}

/// Get input for stdin
fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// Parse input to be a decimal
fn parse_num_input(num_input: &str) -> Result<i32, ParseIntError> {
    if let Some(stripped) = num_input.strip_prefix("0x") {
        i32::from_str_radix(stripped, 16) // Hexadecimal
    } else if let Some(stripped) = num_input.strip_prefix("0b") {
        i32::from_str_radix(stripped, 2) // Binary
    } else {
        num_input.parse() // Decimal
    }
}

/// Parse operator text to get which operator it is
fn parse_operator(operator_input: &str) -> Operator {
    match operator_input.to_lowercase().as_str() {
        "&" | "and" => Operator::And,
        "|" | "or" => Operator::Or,
        "^" | "xor" => Operator::Xor,
        _ => Operator::Invalid,
    }
}

/// Calculate what the result of the operation is
fn get_result(operator: Operator, num1: i32, num2: i32) -> String {
    match operator {
        Operator::And => format!("The result of {} & {} is {}", num1, num2, num1 & num2),
        Operator::Or => format!("The result of {} | {} is {}", num1, num2, num1 | num2),
        Operator::Xor => format!("The result of {} ^ {} is {}", num1, num2, num1 ^ num2),
        Operator::Invalid => "Invalid operator. Try again.".to_string(),
    }
}

/// Calculate the bitwise opperation between two numbers
pub fn calculate() {
    println!("Please enter the first number:");
    let num_input = get_input();
    let num1 = match parse_num_input(&num_input) {
        Ok(value) => value,
        Err(e) => {
            println!("Failed to parse number: {}", e);
            return; // Exit if error
        }
    };

    println!("Please enter the second number:");
    let num_input = get_input();
    let num2 = match parse_num_input(&num_input) {
        Ok(value) => value,
        Err(e) => {
            println!("Failed to parse number: {}", e);
            return; // Exit if error
        }
    };

    println!("Please enter the desired operation:");
    let operator_input = get_input();
    let operator = parse_operator(&operator_input);

    // Get and print the result of the operation
    println!("{}", get_result(operator, num1, num2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_num_input_decimal() {
        assert_eq!(parse_num_input("42"), Ok(42));
    }

    #[test]
    fn test_parse_num_input_hexadecimal() {
        assert_eq!(parse_num_input("0x2A"), Ok(42));
    }

    #[test]
    fn test_parse_num_input_binary() {
        assert_eq!(parse_num_input("0b101010"), Ok(42));
    }

    #[test]
    fn test_parse_num_input_invalid() {
        assert!(parse_num_input("invalid").is_err());
    }

    #[test]
    fn test_parse_operator_and() {
        assert_eq!(parse_operator("&"), Operator::And);
        assert_eq!(parse_operator("AND"), Operator::And);
        assert_eq!(parse_operator("and"), Operator::And);
    }

    #[test]
    fn test_parse_operator_or() {
        assert_eq!(parse_operator("|"), Operator::Or);
        assert_eq!(parse_operator("OR"), Operator::Or);
        assert_eq!(parse_operator("or"), Operator::Or);
    }

    #[test]
    fn test_parse_operator_xor() {
        assert_eq!(parse_operator("^"), Operator::Xor);
        assert_eq!(parse_operator("XOR"), Operator::Xor);
        assert_eq!(parse_operator("xor"), Operator::Xor);
    }

    #[test]
    fn test_parse_operator_invalid() {
        assert_eq!(parse_operator("invalid"), Operator::Invalid);
        assert_eq!(parse_operator("42"), Operator::Invalid);
    }

    #[test]
    fn test_get_result_and() {
        assert_eq!(
            get_result(Operator::And, 5, 3),
            "The result of 5 & 3 is 1".to_string()
        );
    }

    #[test]
    fn test_get_result_or() {
        assert_eq!(
            get_result(Operator::Or, 5, 3),
            "The result of 5 | 3 is 7".to_string()
        );
    }

    #[test]
    fn test_get_result_xor() {
        assert_eq!(
            get_result(Operator::Xor, 5, 3),
            "The result of 5 ^ 3 is 6".to_string()
        );
    }

    #[test]
    fn test_get_result_invalid() {
        assert_eq!(
            get_result(Operator::Invalid, 5, 3),
            "Invalid operator. Try again.".to_string()
        );
    }
}
