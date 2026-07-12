use std::fmt::Display;

pub trait LogLevel: Display {}

pub enum StandardLogLevel {
    Info,
    Warning,
    Error,
}

impl Display for StandardLogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "[INFO]"),
            Self::Warning => write!(f, "[WARNING]"),
            Self::Error => write!(f, "[ERROR]"),
        }
    }
}

impl LogLevel for StandardLogLevel {}

pub struct LogMessage<T: LogLevel> {
    pub message: String,
    pub log_level: T,
}

impl<T: LogLevel> LogMessage<T> {
    pub fn new(message: impl Into<String>, log_level: T) -> Self {
        LogMessage {
            message: message.into(),
            log_level,
        }
    }
}

impl<T: LogLevel> Display for LogMessage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.log_level, self.message)
    }
}
