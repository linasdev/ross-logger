extern crate alloc;

use cortex_m::iprint;
use stm32f1xx_hal_bxcan::pac::ITM;

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $fmt:expr) => {
        $logger.log_debug($fmt);
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        $logger.log_debug(&format!($fmt, $($arg)*));
    };
}

#[macro_export]
macro_rules! log_warning {
    ($logger:expr, $fmt:expr) => {
        $logger.log_warning($fmt);
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        $logger.log_warning(&format!($fmt, $($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $fmt:expr) => {
        $logger.log_error($fmt);
    };
    ($logger:expr, $fmt:expr, $($arg:tt)*) => {
        $logger.log_error(&format!($fmt, $($arg)*));
    };
}

pub enum RossLogLevel {
    Debug,
    Warning,
    Error,
}

pub struct RossLogger {
    log_level: RossLogLevel,
    itm: ITM,
}

impl RossLogger {
    pub fn new(log_level: RossLogLevel, itm: ITM) -> Self {
        RossLogger { log_level, itm }
    }

    pub fn change_log_level(&mut self, log_level: RossLogLevel) {
        self.log_level = log_level;
    }

    pub fn log_debug(&mut self, message: &str) {
        if matches!(self.log_level, RossLogLevel::Debug) {
            self.log("[DEBUG] ");
            self.log(message);
            self.log("\r\n");
        }
    }

    pub fn log_warning(&mut self, message: &str) {
        if matches!(self.log_level, RossLogLevel::Debug)
            || matches!(self.log_level, RossLogLevel::Warning)
        {
            self.log("[DEBUG] ");
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
