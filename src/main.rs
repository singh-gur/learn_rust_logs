use std::fs::read_to_string;

mod log_handler;
use log_handler::utils::{divide, extract_log_type};

fn main() {
    match read_to_string("data/logs.txt") {
        Ok(data) => println!("Warnings:\n{:#?}", extract_log_type(&data, "WARNING")),
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
