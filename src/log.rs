use std::{fmt::Display, fs::OpenOptions, path::Path};

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
/// - Error Log
#[derive(Debug, Clone, Copy)]
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
        !matches!(self, Self::Info)
    }
}

/// Struct that is used to print the log.
#[derive(Debug, Clone)]
pub struct LogMessage<T: LogLevel> {
    message: String,
    log_level: T,
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

    /// Log the LogMessage to the output stream and appends to a file
    pub fn tee(&self, path: impl AsRef<Path>) -> Result<(), std::io::Error> {
        use std::io::Write;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        writeln!(file, "{}", self)?;

        self.log();

        Ok(())
    }
}

impl<T: LogLevel> Display for LogMessage<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.log_level, self.message)
    }
}
