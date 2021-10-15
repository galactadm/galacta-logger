//! Simple logger crate
//!
//! # Usage
//! ```
//! use galacta_logger::{self,GalactaLogger,LevelFilter,prelude::*};
//! /// First initialize logger.
//! /// You may set "GALACTA_LOG_LEVEL" environment variable
//! /// If environment variable is set, argument of init function will be ignored
//! GalactaLogger::init(LevelFilter::Trace);
//!
//! /// These are logs with specified target(unrecommended)
//! error!("This is errror");
//! warn!(target: "target","This is warning");
//! info!("This is info");
//! debug!("This is debug");
//! trace!("This is trace");
//!
//! /// These are logs without specified target or initializations(recommended)
//! gerror!("This is errror");
//! gwarn!("This is warning");
//! ginfo!("This is info");
//! gdebug!("This is debug");
//! gtrace!("This is trace");
//! ```
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![allow(unused_imports)]
#[macro_use]
extern crate log;
extern crate chrono;
use log::{Metadata, Record, SetLoggerError};

/// Enum for setting max level of log
pub use log::LevelFilter;

use std::env;
use std::str::FromStr;

/// prerule include macros.
///
/// # RECCOMENDATION
/// "use galacta_logger::prelude::*" is strongly reccomended
pub mod prelude {
    /// Structure of Logger
    pub use super::GalactaLogger;
    /// macro for invoke debug level log with specified target
    pub use log::debug;
    /// macro for invoke error level log with specified target
    pub use log::error;
    /// macro for invoke info level log with specified target
    pub use log::info;
    /// macro for invoke trace level log with specified target
    pub use log::trace;
    /// macro for invoke warn level log with specified target
    pub use log::warn;
    /// enum for initialize logger for [`super::GalactaLogger::init()`] function
    pub use log::LevelFilter;
    /// macro for invoke debug level log without initialization and target
    pub use super::gdebug;
    /// macro for invoke error level log without initialization and target
    pub use super::gerror;
    /// macro for invoke info level log without initialization and target
    pub use super::ginfo;
    /// macro for invoke trace level log without initialization and target
    pub use super::gtrace;
    /// macro for invoke warn level log without initialization and target
    pub use super::gwarn;
}

/// macro for invoke debug level log without initialization and target
#[macro_export]
macro_rules! gdebug {
    ( $($x:expr),* ) => {{
        let _result = GalactaLogger::init(LevelFilter::Trace);
        struct Dummy {}
        let tmp = std::any::type_name::<Dummy>();
        let target: &str = &tmp[..tmp.len()-7];
        debug!(target: target,$($x),*)
    }}
}

/// macro for invoke debug error log without initialization and target
#[macro_export]
macro_rules! gerror {
    ( $($x:expr),* ) => {{
        let _result = GalactaLogger::init(LevelFilter::Trace);
        struct Dummy {}
        let tmp = std::any::type_name::<Dummy>();
        let target: &str = &tmp[..tmp.len()-7];
        error!(target: target,$($x),*)
    }}
}

/// macro for invoke info level log without initialization and target
#[macro_export]
macro_rules! ginfo {
    ( $($x:expr),* ) => {{
        let _result = GalactaLogger::init(LevelFilter::Trace);
        struct Dummy {}
        let tmp = std::any::type_name::<Dummy>();
        let target: &str = &tmp[..tmp.len()-7];
        info!(target: target,$($x),*)
    }}
}

/// macro for invoke trace level log without initialization and target
#[macro_export]
macro_rules! gtrace {
    ( $($x:expr),* ) => {{
        let _result = GalactaLogger::init(LevelFilter::Trace);
        struct Dummy {}
        let tmp = std::any::type_name::<Dummy>();
        let target: &str = &tmp[..tmp.len()-7];
        trace!(target: target,$($x),*)
    }}
}

/// macro for invoke warn level log without initialization and target
#[macro_export]
macro_rules! gwarn {
    ( $($x:expr),* ) => {{
        let _result = GalactaLogger::init(LevelFilter::Trace);
        struct Dummy {}
        let tmp = std::any::type_name::<Dummy>();
        let target: &str = &tmp[..tmp.len()-7];
        warn!(target: target,$($x),*)
    }}
}

/// Logger structure
pub struct GalactaLogger;

impl GalactaLogger {
    /// Initialize logger.
    /// You MUST initialize logger before any log messages.
    ///
    /// # NOTE
    /// You can use either environment variable "GALACTA_LOG_LEVEL" or variable
    /// "default_max_level" for setting max level of logging.
    /// Environment variable overrides inline variable.
    pub fn init(default_max_level: LevelFilter) -> Result<(), SetLoggerError> {
        match env::var("GALACTA_LOG_LEVEL") {
            Ok(val) => {
                let env_max_level =
                    LevelFilter::from_str(val.as_str()).unwrap_or_else(|_| LevelFilter::max());
                log::set_max_level(env_max_level);
                log::set_boxed_logger(Box::new(GalactaLogger))
            }
            Err(_) => {
                log::set_max_level(default_max_level);
                log::set_boxed_logger(Box::new(GalactaLogger))
            }
        }
    }
}

/// Needed implementation of Log trait for logger
impl log::Log for GalactaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let timedate: String = chrono::Local::now().format("%F %T").to_string();
            let module_path: String = record.module_path().unwrap_or("Unknown module").to_string();
            let level: String = record.level().to_string();
            let message = record.args();
            let file_path: String = record.file().unwrap_or("Unknown file").to_string();
            let line: String = record.line().unwrap_or(0).to_string();
            let target:String = record.target().to_string();
            //let log_massage: String = format!("[{}] {} {} {} {} {} {}",timedate,level,target,module_path,message,file_path,line);
            let log_massage: String = format!(
                "\n[{}] [{}] {}\n{}\n{} ({}:{})\n",
                timedate, level,target, message, module_path, file_path, line,
            );
            eprintln!("{}", log_massage);
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    mod another_module {
        pub fn another_function() {
            info!("This is from another function");
        }
    }
    #[test]
    fn test_log_first_use() {
        let _result = GalactaLogger::init(LevelFilter::Trace);
        info!("This is first log test");
        another_module::another_function();
        info!("This is info");
        warn!("This is warn");
    }

    #[test]
    fn test_log_env_level_max() {
        let _result = GalactaLogger::init(LevelFilter::Trace);
        error!(target: "test_log_env_level_max()","This is error");
        warn!(target: "test_log_env_level_max()","This is warning");
        info!(target: "test_log_env_level_max()","This is information");
        debug!(target: "test_log_env_level_max()","This is debug information");
        trace!(target: "test_log_env_level_max()","This is trace information");
    }

    #[test]
    fn test_gdebug() {
        fn errfunc() {
            gerror!("This is gerror");
        }
        errfunc();
        gdebug!("This is gdebug");
        gwarn!("This is gwarn");
        ginfo!("This is ginfo");
        gtrace!("This is gtrace");

    }
}
