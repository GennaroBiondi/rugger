use crate::{LogLevel, LogMessage};

#[test]
fn test_logging() {
    let my_log_message: String = String::from("this is a log message!");
    let my_log_level: LogLevel = LogLevel::Info;

    let mut log_message = LogMessage::new(my_log_message, my_log_level);
    assert_eq!(
        log_message.to_string(),
        "[INFO]: this is a log message!",
        "Logs don't follow the specification"
    );
    log_message.log_level = LogLevel::Warning;
    assert_eq!(
        log_message.to_string(),
        "[WARNING]: this is a log message!",
        "Logs don't follow the specification"
    );

    log_message.log_level = LogLevel::Error;
    assert_eq!(
        log_message.to_string(),
        "[ERROR]: this is a log message!",
        "Logs don't follow the specification"
    );
}
