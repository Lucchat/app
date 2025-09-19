#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::logger::syslog::info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::logger::syslog::warn(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::logger::syslog::error(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::logger::syslog::debug(&format!($($arg)*));
    };
}
