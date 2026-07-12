use std::fmt::Display;

/// Trait that every LogLevel needs to implement to be used as a LogLevel.
pub trait LogLevel: Display {
    /// Should the enum print to stderr?
    ///
    /// (if the returned value is true then it prints to stderr, else stdout)
    fn is_error(&self) -> bool {
        false
    }
}

/// a Standard LogLevel, includes:
/// - Info logs
/// - Warning logs
/// - Error logs
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

impl LogLevel for StandardLogLevel {
    fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false,
        }
    }
}

/// Struct that is used to print the log.
pub struct LogMessage<T: LogLevel> {
    pub message: String,
    pub log_level: T,
}

impl<T: LogLevel> LogMessage<T> {
    /// Create a new blank LogMessage.
    pub fn new(message: impl Into<String>, log_level: T) -> Self {
        LogMessage {
            message: message.into(),
            log_level,
        }
    }

    /// Log the LogMessage.
    pub fn log(&self) {
        match self.log_level.is_error() {
            true => eprintln!("{}", self),
            false => println!("{}", self),
        }
    }
}

impl<T: LogLevel> Display for LogMessage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.log_level, self.message)
    }
}
