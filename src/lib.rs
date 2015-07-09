//! Simple wrapper for the log crate that enables adding arbitrary information at runtime for
//! future calls to the debugging macros.
//!
//! That magic behind this is a thread_local RefCell<Vec<&'static str>> which when populated will
//! include the included identifiers, seperated with ": ".
//!
//! ```rust
//! # #[macro_use] extern crate hierachical_log;
//! # #[macro_use] extern crate log;
//! # extern crate env_logger;
//!
//! fn main () {
//!     env_logger::init().unwrap();
//!
//!     meta_info!("1");
//!     {
//!         register_logger_info!("Test");
//!         meta_info!("2");
//!         register_logger_info!("Testing");
//!         Foo.foo();
//!     }
//!     meta_info!("4");
//! }
//!
//! #[derive(Debug)]
//! struct Foo;
//!
//! impl Foo {
//!     fn foo(&self) {
//!         register_logger_info!("{:?}", self);
//!         meta_info!("3");
//!     }
//! }
//! ```

#[macro_use] extern crate log;

pub use log::*;

use std::cell::RefCell;

thread_local!(pub static __LOG_METAINFO: RefCell<Vec<String>> = RefCell::new(Vec::new()));

pub struct HierachicalLogScope;

impl Drop for HierachicalLogScope {
    fn drop(&mut self) {
        __LOG_METAINFO.with(|f| f.borrow_mut().pop());
    }
}

#[macro_export]
macro_rules! register_logger_info {
    ($message:expr) => (
        $crate::__LOG_METAINFO.with(|f| f.borrow_mut().push($message));
        let __logger_scoped_message = $crate::HierachicalLogScope
    );
    ($($arg:tt)+) => (register_logger_info!(format!($($arg)+)));
}

#[macro_export]
macro_rules! meta_log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => (
        {
            let vec = $crate::__LOG_METAINFO.with(|f| f.borrow().clone());
            match vec.len() {
                0 => log!(target: $target, $lvl, $($arg)+),
                _ => log!(target: $target, $lvl, "{}: {}", vec.connect(": "), format!($($arg)+)),
            }
        }
    );
    ($lvl:expr, $($arg:tt)+) => (
        {
            let vec = $crate::__LOG_METAINFO.with(|f| f.borrow().clone());
            match vec.len() {
                0 => log!($lvl, $($arg)+),
                _ => log!($lvl, "{}: {}", vec.connect(": "), format!($($arg)+)),
            }
        }
    )
}

#[macro_export]
macro_rules! meta_error {
    (target: $target:expr, $($arg:tt)*) => (
        meta_log!(target: $target, $crate::LogLevel::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        meta_log!($crate::LogLevel::Error, $($arg)*);
    )
}

#[macro_export]
macro_rules! meta_warn {
    (target: $target:expr, $($arg:tt)*) => (
        meta_log!(target: $target, $crate::LogLevel::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        meta_log!($crate::LogLevel::Warn, $($arg)*);
    )
}

#[macro_export]
macro_rules! meta_info {
    (target: $target:expr, $($arg:tt)*) => (
        meta_log!(target: $target, $crate::LogLevel::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        meta_log!($crate::LogLevel::Info, $($arg)*);
    )
}

#[macro_export]
macro_rules! meta_debug {
    (target: $target:expr, $($arg:tt)*) => (
        meta_log!(target: $target, $crate::LogLevel::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        meta_log!($crate::LogLevel::Debug, $($arg)*);
    )
}

#[macro_export]
macro_rules! meta_trace {
    (target: $target:expr, $($arg:tt)*) => (
        meta_log!(target: $target, $crate::LogLevel::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        meta_log!($crate::LogLevel::Trace, $($arg)*);
    )
}
