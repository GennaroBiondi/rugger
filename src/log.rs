use std::fmt::Display;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "[INFO]"),
            Self::Warning => write!(f, "[WARNING]"),
            Self::Error => write!(f, "[ERROR]"),
        }
    }
}

pub struct LogMessage {
    pub message: String,
    pub log_level: LogLevel,
}

impl LogMessage {
    pub fn new(message: impl Into<String>, log_level: LogLevel) -> Self {
        LogMessage {
            message: message.into(),
            log_level,
        }
    }
}

impl Display for LogMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.log_level, self.message)
    }
}
