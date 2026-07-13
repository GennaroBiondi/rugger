#[macro_export]
/// Core macro that handles formatting and builds a LogMessage
macro_rules! log {
    ($level:expr, $msg:expr) => {
        $crate::LogMessage::new($msg, $level).log();
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::LogMessage::new(format!($fmt, $($arg)*), $level).log();
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::log!($crate::StandardLogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_warning {
    ($($arg:tt)*) => {
        $crate::log!($crate::StandardLogLevel::Warning, $($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::log!($crate::StandardLogLevel::Error, $($arg)*);
    };
}
