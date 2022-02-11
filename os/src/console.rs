use crate::sbi::console_putchar;
use core::fmt::{self, Write};

#[allow(unused)]
pub enum LogLevel {
    ERROR = 5, WARN = 4, INFO = 3, DEBUG = 2, TRACE = 1
}

const LOG: LogLevel = LogLevel::INFO;

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn log_print(args: fmt::Arguments, logl: LogLevel) {
    if logl as usize >= LOG as usize{
        Stdout.write_fmt(args).unwrap();
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log_print(format_args!(concat!("\x1b[34m",$fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::INFO);
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log_print(format_args!(concat!("\x1b[31m",$fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::ERROR);
    }
}


#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log_print(format_args!(concat!("\x1b[93m",$fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::WARN);
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log_print(format_args!(concat!("\x1b[32m",$fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::DEBUG);
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log_print(format_args!(concat!("\x1b[90m",$fmt, "\x1b[0m\n") $(, $($arg)+)?), $crate::console::LogLevel::TRACE);
    }
}

