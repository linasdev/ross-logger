#![no_std]

use cortex_m::iprint;
use stm32f1xx_hal_bxcan::pac::ITM;

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $fmt:expr) => {
        unsafe {
            $logger.borrow_mut().log_debug($fmt);
        }
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            $logger.borrow_mut().log_debug(&alloc::format!($fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $fmt:expr) => {
        unsafe {
            $logger.borrow_mut().log_info($fmt);
        }
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            $logger.borrow_mut().log_info(&alloc::format!($fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $fmt:expr) => {
        unsafe {
            $logger.borrow_mut().log_warning($fmt);
        }
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            $logger.borrow_mut().log_warning(&alloc::format!($fmt, $($arg)*));
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $fmt:expr) => {
        unsafe {
            $logger.borrow_mut().log_error($fmt);
        }
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        unsafe {
            extern crate alloc;
            $logger.borrow_mut().log_error(&alloc::format!($fmt, $($arg)*));
        }
    };
}

pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    log_level: LogLevel,
    itm: ITM,
}

impl Logger {
    pub fn new(log_level: LogLevel, itm: ITM) -> Self {
        Logger { log_level, itm }
    }

    pub fn change_log_level(&mut self, log_level: LogLevel) {
        self.log_level = log_level;
    }

    pub fn log_debug(&mut self, message: &str) {
        if matches!(self.log_level, LogLevel::Debug) {
            self.log("[DEBUG] ");
            self.log(message);
            self.log("\r\n");
        }
    }

    pub fn log_info(&mut self, message: &str) {
        if matches!(self.log_level, LogLevel::Debug) || matches!(self.log_level, LogLevel::Info) {
            self.log("[INFO] ");
            self.log(message);
            self.log("\r\n");
        }
    }

    pub fn log_warning(&mut self, message: &str) {
        if matches!(self.log_level, LogLevel::Debug)
            || matches!(self.log_level, LogLevel::Info)
            || matches!(self.log_level, LogLevel::Warning)
        {
            self.log("[WARNING] ");
            self.log(message);
            self.log("\r\n");
        }
    }

    pub fn log_error(&mut self, message: &str) {
        self.log("[ERROR] ");
        self.log(message);
        self.log("\r\n");
    }

    fn log(&mut self, message: &str) {
        iprint!(&mut self.itm.stim[0], message);
    }
}
