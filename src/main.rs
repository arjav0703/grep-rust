use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    if pattern.chars().count() == 1 {
        input_line.contains(pattern)
    } else if pattern.starts_with(r"\") && pattern.len() == 2 {
        regex_handler(input_line, pattern)
    } else {
        panic!("Unhandled pattern: {}", pattern)
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}

fn regex_handler(input_line: &str, pattern: &str) -> bool {
    let special_char = pattern.chars().nth(1).unwrap();
    match special_char {
        'd' => input_line.chars().any(|c| c.is_ascii_digit()),
        'D' => input_line.chars().any(|c| !c.is_ascii_digit()),
        's' => input_line.chars().any(|c| c.is_whitespace()),
        'S' => input_line.chars().any(|c| !c.is_whitespace()),
        'w' => input_line.chars().any(|c| c.is_alphanumeric() || c == '_'),
        'W' => input_line
            .chars()
            .any(|c| !(c.is_alphanumeric() || c == '_')),
        _ => panic!("Unhandled special character: {}", special_char),
    }
}
