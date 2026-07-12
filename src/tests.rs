use std::fmt::Display;

use crate::{LogLevel, LogMessage, StandardLogLevel, log_error, log_info, log_warning};

#[test]
fn test_logging() {
    let my_log_message: String = String::from("this is a log message!");
    let my_log_level: StandardLogLevel = StandardLogLevel::Info;

    let mut log_message = LogMessage::new(my_log_message, my_log_level);
    assert_eq!(
        log_message.to_string(),
        "[INFO]: this is a log message!",
        "Logs don't follow the specification"
    );
    log_message.log_level = StandardLogLevel::Warning;
    assert_eq!(
        log_message.to_string(),
        "[WARNING]: this is a log message!",
        "Logs don't follow the specification"
    );

    log_message.log_level = StandardLogLevel::Error;
    assert_eq!(
        log_message.to_string(),
        "[ERROR]: this is a log message!",
        "Logs don't follow the specification"
    );
}

#[test]
fn test_flexibility() {
    enum MyLogLevel {
        Foo,
        Bar,
    }

    impl Display for MyLogLevel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Foo => write!(f, "[FOO]"),
                Self::Bar => write!(f, "[BAR]"),
            }
        }
    }

    impl LogLevel for MyLogLevel {}

    let my_log_message: String = String::from("this is a log message!");
    let my_log_level = MyLogLevel::Bar;

    let mut log_message = LogMessage::new(my_log_message, my_log_level);
    assert_eq!(
        log_message.to_string(),
        "[BAR]: this is a log message!",
        "Logs don't follow the specification"
    );
    log_message.log_level = MyLogLevel::Foo;
    assert_eq!(
        log_message.to_string(),
        "[FOO]: this is a log message!",
        "Logs don't follow the specification"
    );
}

#[test]
fn test_macros() {
    log_info!("This is an incredibly convenient and fast log info message!");
    log_warning!("This is an incredibly convenient and fast log warning message!");
    log_error!("Oh no! An error occurred apparently... but at least it was convenient to throw!");
}
