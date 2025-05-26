#[macro_export]
macro_rules! xlog_start {
    ($($arg:tt)*) => {
        xlog_start(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! xlog_debug {
    ($($arg:tt)*) => {
        xlog_debug(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! xlog_info {
    ($($arg:tt)*) => {
        xlog_info(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! xlog_error {
    ($($arg:tt)*) => {
        xlog_error(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! xlog_fatal {
    ($($arg:tt)*) => {
        xlog_fatal(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! xlog_panic {
    ($($arg:tt)*) => {{
        $crate::xlog::xlog::xlog_fatal(format_args!($($arg)*));
        panic!("{}", format!($($arg)*));
    }};
}
//xlog_panic(format_args!($($arg)*))

#[macro_export]
macro_rules! xlog_stop {
    ($start:expr, $($arg:tt)*) => {
        xlog_stop($start, format_args!($($arg)*))
    };
}
