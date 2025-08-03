use std::fs::{read_to_string, write};

mod log_handler;
use log_handler::utils::extract_log_type;

fn main() {
    let text = read_to_string("data/run.log").expect("Failed to read logs");

    let error_logs = extract_log_type(text.as_str(), "ERROR");
    write("data/errors.log", error_logs.join("\n")).expect("Unable to write error logs");
}
