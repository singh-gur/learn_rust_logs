use std::io::Error;

pub fn extract_log_type(logs: &String, log_type: &str) -> Vec<String> {
    let mut filtered_logs = vec![];
    for line in logs.lines() {
        if line.contains(log_type) {
            filtered_logs.push(line.to_string());
        }
    }
    filtered_logs
}

pub fn divide(a: i32, b: i32) -> Result<i32, Error> {
    if b == 0 {
        Err(Error::other("Division by zero is not allowed"))
    } else {
        Ok(a / b)
    }
}
