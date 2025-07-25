use std::fs::read_to_string;
use std::io::Error;

fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        Err(Error::other("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match read_to_string("data/logs.txt") {
        Ok(data) => println!("File content:\n{:?}", data),
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
