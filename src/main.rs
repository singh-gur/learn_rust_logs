use std::fs::{read_to_string, write};
use std::io::Error;

fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        Err(Error::other("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

fn read_file(file_path: String) -> Result<String, Error> {
    read_to_string(file_path)
}

fn write_file(file_path: String, content: &str) -> Result<(), Error> {
    match write(file_path, content) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn main() {
    match read_to_string("data/logs.txt") {
        Ok(data) => println!("File content:\n{:#?}", data),
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
