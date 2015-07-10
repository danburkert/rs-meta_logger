//! Simple wrapper for the log crate that enables adding arbitrary information at runtime for
//! future calls to the debugging macros.
//!
//! That magic behind this is a thread_local RefCell<Vec<&'static str>> which when populated will
//! include the included identifiers, seperated with ": ".
//!
//! ```rust
//! # #[macro_use] extern crate hierachical_log;
//! # extern crate env_logger;
//!
//! fn main () {
//!     env_logger::init().unwrap();
//!
//!     info!("1");
//!     {
//!         register_logger_info!("Test");
//!         info!("2");
//!         register_logger_info!("Testing");
//!         Foo.foo();
//!     }
//!     info!("4");
//! }
//!
//! #[derive(Debug)]
//! struct Foo;
//!
//! impl Foo {
//!     fn foo(&self) {
//!         register_logger_info!("{:?}", self);
//!         info!("3");
//!     }
//! }
//! ```

#![feature(macro_reexport)]

#[macro_reexport(error,warn,info,debug,trace)] extern crate log;

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
        $crate::__LOG_METAINFO.with(|f| f.borrow_mut().push(String::from($message)));
        let __logger_scoped_message = $crate::HierachicalLogScope
    );
    ($($arg:tt)+) => (register_logger_info!(format!($($arg)+)));
}

#[macro_export]
macro_rules! log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        static _LOC: $crate::LogLocation = $crate::LogLocation {
            __line: line!(),
            __file: file!(),
            __module_path: module_path!(),
        };
        let lvl = $lvl;
        if !cfg!(log_level = "off") &&
                (lvl <= $crate::LogLevel::Error || !cfg!(log_level = "error")) &&
                (lvl <= $crate::LogLevel::Warn || !cfg!(log_level = "warn")) &&
                (lvl <= $crate::LogLevel::Debug || !cfg!(log_level = "debug")) &&
                (lvl <= $crate::LogLevel::Info || !cfg!(log_level = "info")) &&
                lvl <= $crate::max_log_level() {
            let target = $crate::__LOG_METAINFO.with(|vec| {
                    match vec.borrow().len() {
                        0 => $target,
                        _ => format!("{}: {}", vec.borrow().connect(": "), $target).as_str(),
                    }
                });
            $crate::__log(lvl, target, &_LOC, format_args!($($arg)+))
        }
    });
    ($lvl:expr, $($arg:tt)+) => (log!(target: module_path!(), $lvl, $($arg)+))
}

#[macro_export]
macro_rules! meta_assert {
    ($condition:expr) => (
        if !$condition {
            let vec = $crate::__LOG_METAINFO.with(|f| f.borrow().clone());
            match vec.len() {
                0 => panic!(stringify!($condition)),
                _ => panic!("{}: {}", vec.connect(": "), stringify!($condition)),
            }
        }
    );
    ($condition:expr, $($arg:tt)+) => (
        if !$condition {
            let vec = $crate::__LOG_METAINFO.with(|f| f.borrow().clone());
            match vec.len() {
                0 => panic!($($arg)+),
                _ => panic!("{}: {}", vec.connect(": "), format!($($arg)+)),
            }
        }
    );
}
