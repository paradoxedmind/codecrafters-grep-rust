use anyhow::Result;
use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "" => panic!("Empty Pattern not allowed"),
        "\\d" => input_line.chars().any(|c| c.is_digit(10)),
        "\\w" => input_line
            .chars()
            .any(|c| c.is_ascii_alphanumeric() || c == '_'),
        positve
            if positve.chars().nth(0).is_some_and(|c| c == '[')
                && positve.chars().last().is_some_and(|c| c == ']') =>
        {
            let p = &positve[1..positve.len()];
            input_line.chars().any(|c| p.contains(c))
        }
        _ => input_line.contains(pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() -> Result<()> {
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
