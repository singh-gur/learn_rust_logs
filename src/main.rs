use std::fs::{read_to_string, write};
use std::io::Error;

fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        Err(Error::other("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

fn extract_errors_from_logs(logs: &String) -> Vec<String> {
    let mut errors = vec![];
    for line in logs.lines() {
        if line.contains("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}

fn main() {
    match read_to_string("data/logs.txt") {
        Ok(data) => println!("Errors:\n{:#?}", extract_errors_from_logs(&data)),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    match divide(4, 2) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => eprintln!("Error dividing numbers: {}", e),
    }
    match divide(4, 0) {
        Ok(result) => println!("Division result: {}", result),
        Err(e) => eprintln!("Error dividing numbers: {}", e),
    }
}
