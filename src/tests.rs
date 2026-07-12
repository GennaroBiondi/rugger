use crate::{LogLevel, LogMessage};

#[test]
fn test_logging() {
    let my_log_message: String = String::from("this is a log message!");
    let my_log_level: LogLevel = LogLevel::Info;

    let log_message = LogMessage::new(my_log_message, my_log_level);

    println!("{}", log_message);
}
