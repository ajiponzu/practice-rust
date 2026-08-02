mod log;

fn main() {
    let logs = vec![
        "INFO server started".to_string(),
        "WARN not updated".to_string(),
        "ERROR memory leak".to_string(),
        "INFO server running...".to_string(),
    ];
    let result = log::aggregate(&logs);
    match result {
        Ok(counter) => {
            println!("{}", counter.display());
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
