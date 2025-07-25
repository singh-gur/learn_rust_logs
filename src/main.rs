use std::fs::read_to_string;

mod log_handler;
use log_handler::utils::{divide, extract_errors_from_logs};

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
