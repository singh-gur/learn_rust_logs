pub fn extract_log_type(logs: &str, log_type: &str) -> Vec<String> {
    let mut filtered_logs = vec![];
    for line in logs.split("\n") {
        if line.contains(log_type) {
            filtered_logs.push(line.to_string());
        }
    }
    filtered_logs
}
