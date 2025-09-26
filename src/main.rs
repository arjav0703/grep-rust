use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn match_advanced(input_line: &str, pattern: &str) -> Result<bool, String> {
    let mut input_iter = input_line.chars();
    let mut pattern_iter = pattern.chars();

    while let Some(pattern_char) = pattern_iter.next() {
        if pattern_char == '\\' {
            match pattern_iter.next() {
                Some('d') => {
                    if !input_iter.any(|c| c.is_ascii_digit()) {
                        return Ok(false);
                    }
                }
                Some('w') => {
                    if !input_iter.any(|c| c.is_alphanumeric() || c == '_') {
                        return Ok(false);
                    }
                }
                Some('s') => {
                    if !input_iter.any(|c| c.is_whitespace()) {
                        return Ok(false);
                    }
                }
                Some('^') => {}
                Some(other) => return Err(format!("Unhandled escape sequence: \\{}", other)),
                None => return Err("Pattern ends with a single backslash".to_string()),
            }
        } else if pattern_char == '[' {
            let mut char_group = String::new();
            while let Some(c) = pattern_iter.next() {
                if c == ']' {
                    break;
                }
                char_group.push(c);
            }
            if char_group.is_empty() {
                return Err("Empty character group".to_string());
            }
            let is_negated = char_group.starts_with('^');
            let chars: Vec<char> = if is_negated {
                char_group[1..].chars().collect()
            } else {
                char_group.chars().collect()
            };
            let matches = input_iter.any(|c| {
                if is_negated {
                    !chars.contains(&c)
                } else {
                    chars.contains(&c)
                }
            });
            if !matches {
                return Ok(false);
            }
        } else if pattern_char == '^' {
            if input_iter.next() != Some(pattern_iter.next().unwrap_or('\0'))
                && pattern_iter.next().is_some()
            {
                return Ok(false);
            }
        } else if pattern_char == '$' {
            if input_iter.next().is_some() {
                return Ok(false);
            }
        } else {
            if !input_iter.any(|c| c == pattern_char) {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        eprintln!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let file_name = env::args().nth(3).unwrap();

    let file = File::open(&file_name).unwrap();
    let reader = io::BufReader::new(file);

    let mut found = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if match_advanced(&line, &pattern).unwrap_or(false) {
            println!("{}", line);
            found = true;
        }
    }

    process::exit(if found { 0 } else { 1 });
}
