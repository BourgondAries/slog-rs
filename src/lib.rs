//! # Slog -  Structured, composable logging for Rust

#![feature(unboxed_closures)]
#![warn(missing_docs)]

extern crate crossbeam;
extern crate chrono;
extern crate rustc_serialize as serialize;

#[macro_use]
extern crate error_chain;

use std::sync::Arc;
use std::fmt;

/// Convenience function for building `&[OwnedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use slog::drain::IntoLogger;
///
/// fn main() {
///     let drain = slog::drain::discard();
///     let root = drain.into_logger(o!("key1" => "value1", "key2" => "value2"));
/// }
/// ```
#[macro_export]
macro_rules! o(
    () => {
        vec![]
    };
    ($($k:expr => $v:expr),*) => {
        {
        use std;
        vec![$(($k, std::boxed::Box::new($v) as std::boxed::Box<$crate::ser::SyncSerialize>)),*]
        }
    };
);

/// Log message of a given level
///
/// Use wrappers `error!`, `warn!` etc. instead
///
/// The `max_level_*` features can be used to statically disable logging at
/// various levels.
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use slog::drain::IntoLogger;
///
/// fn main() {
///     let drain = slog::drain::discard();
///     let root = drain.into_logger(o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "test info log", "log-key" => true);
/// }
/// ```
#[macro_export]
macro_rules! log(
    ($l:expr, $lvl:expr, $msg:expr) => {
        let lvl = $lvl;
        if lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            $l.log(
                &$crate::RecordInfo::new(
                    lvl,
                    &$msg,
                    file!(),
                    line!(),
                    module_path!(),
                    &[]
                )
            )
        }
    };
    ($l:expr, $lvl:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        let lvl = $lvl;
        if lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            $l.log(
                &$crate::RecordInfo::new(
                    $lvl,
                    &$msg,
                    file!(),
                    line!(),
                    module_path!(),
                    &[$(($k, &$v)),*]
                )
            )
        }
    };
);

/// Alias for `log`
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_log(
    ($l:expr, $lvl:expr, $msg:expr) => {
        $l.log(
            &$crate::RecordInfo::new(
                $lvl,
                &$msg,
                file!(),
                line!(),
                module_path!(),
                &[]
            )
        )
    };
    ($l:expr, $lvl:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        $l.log(
            &$crate::RecordInfo::new(
                $lvl,
                &$msg,
                file!(),
                line!(),
                module_path!(),
                &[$(($k, &$v)),*]
            )
        )
    };
);

/// Log critical level record
#[macro_export]
macro_rules! crit(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Critical, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Critical, $msg, $($k => $v),*)
    };
);

/// Log critical level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_crit(
    ($l:expr, $msg:expr) => {
        slog_log!($l, $crate::Level::Critical, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Critical, $msg, $($k => $v),*)
    };
);

/// Log error level record
#[macro_export]
macro_rules! error(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Error, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Error, $msg, $($k => $v),*)
    };
);

/// Log error level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_error(
    ($l:expr, $msg:expr) => {
        slog_log!($l, $crate::Level::Error, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Error, $msg, $($k => $v),*)
    };
);


/// Log warning level record
#[macro_export]
macro_rules! warn(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Warning, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Warning, $msg, $($k => $v),*)
    };
);

/// Log warning level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_warn(
    ($l:expr, $msg:expr) => {
    slog_log!($l, $crate::Level::Warning, $msg)
};
($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Warning, $msg, $($k => $v),*)
    };
);

/// Log info level record
#[macro_export]
macro_rules! info(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Info, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Info, $msg, $($k => $v),*)
    };
);

/// Log info level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_info(
    ($l:expr, $msg:expr) => {
        slog_log!($l, $crate::Level::Info, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Info, $msg, $($k => $v),*)
    };
);

/// Log debug level record
#[macro_export]
macro_rules! debug(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Debug, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Debug, $msg, $($k => $v),*)
    };
);

/// Log debug level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_debug(
    ($l:expr, $msg:expr) => {
        slog_log!($l, $crate::Level::Debug, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Debug, $msg, $($k => $v),*)
    };
);


/// Log trace level record
#[macro_export]
macro_rules! trace(
    ($l:expr, $msg:expr) => {
        log!($l, $crate::Level::Trace, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        log!($l, $crate::Level::Trace, $msg, $($k => $v),*)
    };
);

/// Log trace level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_trace(
    ($l:expr, $msg:expr) => {
        slog_log!($l, $crate::Level::Trace, $msg)
    };
    ($l:expr, $msg:expr, $($k:expr => $v:expr),*) => {
        slog_log!($l, $crate::Level::Trace, $msg, $($k => $v),*)
    };
);


/// Drains - logging outputs
pub mod drain;

/// Logger - logging handle
pub mod logger;

/// Serialization
pub mod ser;

/// Output formating
pub mod format;

pub use logger::Logger;
pub use logger::RecordInfo;

include!("_level.rs");

/// Key value pair that can be owned by `Logger`
pub type OwnedKeyValue = (&'static str, Box<ser::SyncSerialize>);
/// Key value pair that can be part of each logging record
pub type BorrowedKeyValue<'a> = (&'static str, &'a ser::Serialize);

/// Values specific for this Logger and reference to it's parent values
pub struct OwnedKeyValueNode {
    parent: Option<Arc<OwnedKeyValueNode>>,
    values: Vec<OwnedKeyValue>,
}

impl OwnedKeyValueNode {
    /// New `OwnedKeyValue` with a parent
    pub fn new(values: Vec<OwnedKeyValue>, parent: Arc<OwnedKeyValueNode>) -> Self {
        OwnedKeyValueNode {
            parent: Some(parent),
            values: values,
        }
    }

    /// New `OwnedKeyValue` without a parent (root)
    pub fn new_root(values: Vec<OwnedKeyValue>) -> Self {
        OwnedKeyValueNode {
            parent: None,
            values: values,
        }
    }

    /// Iterator over `OwnedKeyValue`-s
    pub fn iter(&self) -> OwnedKeyValueNodeIterator {
        OwnedKeyValueNodeIterator::new(self)
    }
}

/// Iterator over `OwnedKeyValue`-s
pub struct OwnedKeyValueNodeIterator<'a> {
    next_node: &'a Option<Arc<OwnedKeyValueNode>>,
    iter: std::slice::Iter<'a, OwnedKeyValue>,
}

impl<'a> OwnedKeyValueNodeIterator<'a> {
    fn new(node: &'a OwnedKeyValueNode) -> Self {
        OwnedKeyValueNodeIterator {
            next_node: &node.parent,
            iter: node.values.iter(),
        }
    }
}

impl<'a> Iterator for OwnedKeyValueNodeIterator<'a> {
    type Item = &'a OwnedKeyValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => return Some(&*x),
                None => {
                    match self.next_node {
                        &Some(ref node) => {
                            self.iter = node.values.iter();
                            self.next_node = &node.parent;
                        }
                        &None => return None,
                    }
                }
            }
        }
    }
}

#[inline(always)]
#[doc(hidden)]
/// Not an API
pub fn __slog_static_max_level() -> LevelFilter {
    if !cfg!(debug_assertions) {
        if cfg!(feature = "release_max_level_off") {
            return LevelFilter::Off;
        } else if cfg!(feature = "release_max_level_error") {
            return LevelFilter::Error;
        } else if cfg!(feature = "release_max_level_warn") {
            return LevelFilter::Warning;
        } else if cfg!(feature = "release_max_level_info") {
            return LevelFilter::Info;
        } else if cfg!(feature = "release_max_level_debug") {
            return LevelFilter::Debug;
        } else if cfg!(feature = "release_max_level_trace") {
            return LevelFilter::Trace;
        }
    }
    if cfg!(feature = "max_level_off") {
        LevelFilter::Off
    } else if cfg!(feature = "max_level_error") {
        LevelFilter::Error
    } else if cfg!(feature = "max_level_warn") {
        LevelFilter::Warning
    } else if cfg!(feature = "max_level_info") {
        LevelFilter::Info
    } else if cfg!(feature = "max_level_debug") {
        LevelFilter::Debug
    } else {
        LevelFilter::Trace
    }
}

#[cfg(test)]
mod tests;
