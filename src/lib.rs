//! Simple wrapper for the log crate that enables adding arbitrary information at runtime for
//! future calls to the debugging macros.
//!
//! That magic behind this is a thread_local RefCell<Vec<String>> which when populated will
//! include the included identifiers, seperated with ": ".
//!
//! ```rust
//! #[macro_use] extern crate log;
//! #[macro_use] extern crate scoped_log;
//! extern crate env_logger;
//!
//! fn main () {
//!     env_logger::init().unwrap();
//!
//!     scoped_info!("1");
//!     {
//!         push_log_scope!("Test");
//!         scoped_info!("2");
//!         push_log_scope!("Testing");
//!         Foo.foo();
//!     }
//!     scoped_info!("4");
//! }
//!
//! #[derive(Debug)]
//! struct Foo;
//!
//! impl Foo {
//!     fn foo(&self) {
//!         push_log_scope!("{:?}", self);
//!         scoped_info!("3");
//!     }
//! }
//! ```

#[macro_use] extern crate log;

use std::cell::RefCell;

thread_local!(pub static __LOG_SCOPES: RefCell<Vec<String>> = RefCell::new(Vec::new()));

pub struct Scope;

impl Drop for Scope {
    fn drop(&mut self) {
        if !cfg!(log_level = "off") {
            __LOG_SCOPES.with(|f| f.borrow_mut().pop());
        }
    }
}

#[macro_export]
macro_rules! push_log_scope {
    ($scope:expr) => (
        if !cfg!(log_level = "off") {
            $crate::__LOG_SCOPES.with(|cell| {
                let mut scopes = cell.borrow_mut();
                match scopes.len() {
                    0 => scopes.push(format!("{}", $scope)),
                    len => {
                        let scope = format!("{}: {}", scopes[len - 1], $scope);
                        scopes.push(scope);
                    }
                }
            });
        }
        let __logger_scoped_message = $crate::Scope
    );
    ($($arg:tt)+) => (push_log_scope!(format_args!($($arg)+)));
}

#[macro_export]
macro_rules! scoped_log {
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        $crate::__LOG_SCOPES.with(|cell| {
            let scopes = cell.borrow();
            match scopes.len() {
                0 => log!(target: $target, $lvl, $($arg)+),
                len => log!(target: $target, $lvl, "{}: {}", scopes[len - 1], format_args!($($arg)+)),
            }
        })
    });
    ($lvl:expr, $($arg:tt)+) => ({
        $crate::__LOG_SCOPES.with(|cell| {
            let scopes = cell.borrow();
            match scopes.len() {
                0 => log!($lvl, $($arg)+),
                len => log!($lvl, "{}: {}", scopes[len - 1], format_args!($($arg)+)),
            }
        })
    })
}

#[macro_export]
macro_rules! scoped_assert {
    ($cond:expr) => (
        if !$cond {
            $crate::__LOG_SCOPES.with(|cell| {
                let scopes = cell.borrow();
                match scopes.len() {
                    0 => panic!(concat!("assertion failed: ", stringify!($cond))),
                    len => panic!(concat!("{}: assertion failed: ", stringify!($cond)), scopes[len - 1]),
                }
            })
        }
    );
    ($cond:expr, $($arg:tt)+) => (
        if !$cond {
            $crate::__LOG_SCOPES.with(|cell| {
                let scopes = cell.borrow();
                match scopes.len() {
                    0 => panic!($($arg)+),
                    len => panic!("{}: {}", scopes[len - 1], format_args!($($arg)+)),
                }
            })
        }
    );
}

#[macro_export]
macro_rules! scoped_error {
    (target: $target:expr, $($arg:tt)*) => (
        scoped_log!(target: $target, ::log::LogLevel::Error, $($arg)*);
    );
    ($($arg:tt)*) => (
        scoped_log!(::log::LogLevel::Error, $($arg)*);
    )
}

#[macro_export]
macro_rules! scoped_warn {
    (target: $target:expr, $($arg:tt)*) => (
        scoped_log!(target: $target, ::log::LogLevel::Warn, $($arg)*);
    );
    ($($arg:tt)*) => (
        scoped_log!(::log::LogLevel::Warn, $($arg)*);
    )
}

#[macro_export]
macro_rules! scoped_info {
    (target: $target:expr, $($arg:tt)*) => (
        scoped_log!(target: $target, ::log::LogLevel::Info, $($arg)*);
    );
    ($($arg:tt)*) => (
        scoped_log!(::log::LogLevel::Info, $($arg)*);
    )
}

#[macro_export]
macro_rules! scoped_debug {
    (target: $target:expr, $($arg:tt)*) => (
        scoped_log!(target: $target, ::log::LogLevel::Debug, $($arg)*);
    );
    ($($arg:tt)*) => (
        scoped_log!(::log::LogLevel::Debug, $($arg)*);
    )
}

#[macro_export]
macro_rules! scoped_trace {
    (target: $target:expr, $($arg:tt)*) => (
        scoped_log!(target: $target, ::log::LogLevel::Trace, $($arg)*);
    );
    ($($arg:tt)*) => (
        scoped_log!(::log::LogLevel::Trace, $($arg)*);
    )
}
