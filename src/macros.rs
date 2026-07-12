#[macro_export]
/// Macro to quickly log.
///
/// The macro uses format!(...) under the hood, so formatting is available.
macro_rules! log_info {
    ($msg:expr) => {
        println!("{}: {}", $crate::StandardLogLevel::Info, $msg);
    };
}

#[macro_export]
/// Macro to quickly log a warning.
///
/// The macro uses format!(...) under the hood, so formatting is available.
macro_rules! log_warning {
    ($msg:expr) => {
        println!("{}: {}", $crate::StandardLogLevel::Warning, $msg);
    };
}

#[macro_export]
/// Macro to quickly log an error.
///
/// The macro uses format!(...) under the hood, so formatting is available.
macro_rules! log_error {
    ($msg:expr) => {
        println!("{}: {}", $crate::StandardLogLevel::Error, $msg);
    };
}
