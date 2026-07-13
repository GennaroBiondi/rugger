#[macro_export]
/// Core macro that handles formatting and builds a LogMessage to log
macro_rules! log {
    ($level:expr, $msg:expr) => {
        $crate::LogMessage::new($msg, $level).log();
    };
    ($level:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::LogMessage::new(format!($fmt, $($arg)*), $level).log();
    };
}

#[macro_export]
/// Core macro that handles formatting and builds a LogMessage to tee
macro_rules! tee {
    ($level:expr, $file_path:expr, $msg:expr) => {
        let _ = $crate::LogMessage::new($msg, $level).tee($file_path);
    };
    ($level:expr, $file_path:expr, $fmt:expr, $($arg:tt)*) => {
        let _ = $crate::LogMessage::new(format!($fmt, $($arg)*), $level).tee($file_path);
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

#[macro_export]
macro_rules! tee_info {
    ($file_path:expr, $($arg:tt)*) => {
        $crate::tee!($crate::StandardLogLevel::Info, $file_path, $($arg)*);
    };
}

#[macro_export]
macro_rules! tee_warning {
    ($file_path:expr, $($arg:tt)*) => {
        $crate::tee!($crate::StandardLogLevel::Warning, $file_path, $($arg)*);
    };
}

#[macro_export]
macro_rules! tee_error {
    ($file_path:expr, $($arg:tt)*) => {
        $crate::tee!($crate::StandardLogLevel::Error, $file_path, $($arg)*);
    };
}
