use std::io::Error;

pub fn extract_errors_from_logs(logs: &String) -> Vec<String> {
    let mut errors = vec![];
    for line in logs.lines() {
        if line.contains("ERROR") {
            errors.push(line.to_string());
        }
    }
    errors
}

pub fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        Err(Error::other("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}
