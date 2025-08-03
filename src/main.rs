use std::fs::{read_to_string, write};
use std::io::Error;
mod log_handler;
use log_handler::utils::extract_log_type;

fn main() -> Result<(), Error> {
    let text = read_to_string("data/run.log")?;

    let error_logs = extract_log_type(text.as_str(), "ERROR");
    write("data2/errors.log", error_logs.join("\n"))?;

    Ok(())
}
