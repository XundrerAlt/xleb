pub mod uart;
use core::fmt;
use core::fmt::Write;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Verbose,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Verbose => "VERBOSE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
    fn color(&self) -> &'static str {
        match self {
            LogLevel::Verbose => "\x1b[0m",
            LogLevel::Debug => "\x1b[36m",
            LogLevel::Info  => "\x1b[32m",
            LogLevel::Warning  => "\x1b[33m",
            LogLevel::Error => "\x1b[31m",
            LogLevel::Fatal => "\x1b[31m",
        }
    }
}

struct UartWriter;
impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            uart::uart_putc(b);
        }
        Ok(())
    }
}

pub fn log(level: LogLevel, args: fmt::Arguments) {
    let mut writer = UartWriter;
    let _ = write!(writer, "{}{}{}:\x1b[0m ",
        level.color(), "", level.as_str()
    );
    let _ = writer.write_fmt(args);
    let _ = writer.write_str("\n");
}

// macros

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Verbose,
            format_args!($($arg)*),
        )
    };
}


#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Debug,
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Info,
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Warn,
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Error,
            format_args!($($arg)*),
        )
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {
        $crate::logger::log(
            $crate::logger::LogLevel::Fatal,
            format_args!($($arg)*),
        )
    };
}
