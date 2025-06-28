use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::thread;
#[cfg(feature = "_log")]
use flexi_logger::{Cleanup, Criterion, DeferredNow, Duplicate, FileSpec, FlexiLoggerError, Naming, Record};
use flexi_logger::filter::{LogLineFilter, LogLineWriter};
#[cfg(all(feature = "_log", not(feature = "nolog")))]
pub use tracing::{info, warn, trace, debug, error};
use tracing::log;
use tracing::log::Metadata;

#[cfg(feature = "nolog")]
#[macro_export]
macro_rules! error {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, $($arg:tt)+ ) => (
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, $($arg:tt)+ ) => (
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, $($arg:tt)+) => (
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    ($($k:ident).+ = $($field:tt)*) => (
    );
    (?$($k:ident).+ = $($field:tt)*) => (
    );
    (%$($k:ident).+ = $($field:tt)*) => (
    );
    ($($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+, $($field:tt)*) => (
    );
    (%$($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+) => (
    );
    (%$($k:ident).+) => (
    );
    ($($k:ident).+) => (
    );
    ($($arg:tt)+) => (
    );
}

#[cfg(feature = "nolog")]
#[macro_export]
macro_rules! debug {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, $($arg:tt)+ ) => (
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, $($arg:tt)+ ) => (
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, $($arg:tt)+) => (
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    ($($k:ident).+ = $($field:tt)*) => (
    );
    (?$($k:ident).+ = $($field:tt)*) => (
    );
    (%$($k:ident).+ = $($field:tt)*) => (
    );
    ($($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+, $($field:tt)*) => (
    );
    (%$($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+) => (
    );
    (%$($k:ident).+) => (
    );
    ($($k:ident).+) => (
    );
    ($($arg:tt)+) => (
    );
}

#[cfg(feature = "nolog")]
#[macro_export]
macro_rules! trace {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, $($arg:tt)+ ) => (
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, $($arg:tt)+ ) => (
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, $($arg:tt)+) => (
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    ($($k:ident).+ = $($field:tt)*) => (
    );
    (?$($k:ident).+ = $($field:tt)*) => (
    );
    (%$($k:ident).+ = $($field:tt)*) => (
    );
    ($($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+, $($field:tt)*) => (
    );
    (%$($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+) => (
    );
    (%$($k:ident).+) => (
    );
    ($($k:ident).+) => (
    );
    ($($arg:tt)+) => (
    );
}

#[cfg(feature = "nolog")]
#[macro_export]
macro_rules! info {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, $($arg:tt)+ ) => (
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, $($arg:tt)+ ) => (
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, $($arg:tt)+) => (
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    ($($k:ident).+ = $($field:tt)*) => (
    );
    (?$($k:ident).+ = $($field:tt)*) => (
    );
    (%$($k:ident).+ = $($field:tt)*) => (
    );
    ($($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+, $($field:tt)*) => (
    );
    (%$($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+) => (
    );
    (%$($k:ident).+) => (
    );
    ($($k:ident).+) => (
    );
    ($($arg:tt)+) => (
    );
}


#[cfg(feature = "nolog")]
#[macro_export]
macro_rules! warn {
    // Name / target / parent.
    (name: $name:expr, target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / target.
    (name: $name:expr, target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, target: $target:expr, $($arg:tt)+ ) => (
    );

    // Target / parent.
    (target: $target:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name / parent.
    (name: $name:expr, parent: $parent:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, parent: $parent:expr, $($arg:tt)+ ) => (
    );

    // Name.
    (name: $name:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (name: $name:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (name: $name:expr, $($arg:tt)+ ) => (
    );

    // Target.
    (target: $target:expr, { $($field:tt)* }, $($arg:tt)* ) => (
    );
    (target: $target:expr, $($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, ?$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, %$($k:ident).+ $($field:tt)* ) => (
    );
    (target: $target:expr, $($arg:tt)+ ) => (
    );

    // Parent.
    (parent: $parent:expr, { $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    (parent: $parent:expr, $($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+ = $($field:tt)*) => (
    );
    (parent: $parent:expr, $($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, ?$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, %$($k:ident).+, $($field:tt)*) => (
    );
    (parent: $parent:expr, $($arg:tt)+) => (
    );

    // ...
    ({ $($field:tt)+ }, $($arg:tt)+ ) => (
    );
    ($($k:ident).+ = $($field:tt)*) => (
    );
    (?$($k:ident).+ = $($field:tt)*) => (
    );
    (%$($k:ident).+ = $($field:tt)*) => (
    );
    ($($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+, $($field:tt)*) => (
    );
    (%$($k:ident).+, $($field:tt)*) => (
    );
    (?$($k:ident).+) => (
    );
    (%$($k:ident).+) => (
    );
    ($($k:ident).+) => (
    );
    ($($arg:tt)+) => (
    );
}


#[cfg(feature = "_log")]
fn custom_format(writer: &mut dyn std::io::Write, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
    let file = match record.file() {
        None => {
            "<unknown>".to_string()
        }
        Some(path) => {
            // Path::new(path).file_name().map(|v| v.to_string_lossy().to_string()).unwrap_or("<unknown>".to_string())
            path.to_string()
        }
    };
    write!(
        writer,
        "{} [{}] [{}:{}:{}] [{}] - {}",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.metadata().target(),
        file,
        record.line().unwrap_or(0),
        thread::current().name().unwrap_or(format!("{:?}", thread::current().id()).as_str()),
        &record.args()
    )
}
pub struct Logger {
    app_name: String,
    log_level: String,
    log_to_file: bool,
    log_path: PathBuf,
    log_file_size: u64,
    log_file_count: usize,
    instance_id: String,
    output_console: bool,
    filter: Vec<String>,
    module_logs: Vec<(String, String)>,
}

impl Logger {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            log_level: "info".to_string(),
            log_to_file: false,
            log_path: std::env::current_dir().unwrap().join("logs"),
            log_file_size: 10 * 1024 * 1024,
            log_file_count: 10,
            instance_id: "".to_string(),
            output_console: true,
            filter: vec![],
            module_logs: vec![],
        }
    }

    pub fn set_instance_id(mut self, instance_id: &str) -> Self {
        self.instance_id = instance_id.to_string();
        self
    }

    pub fn set_process_id_to_instance_id(mut self) -> Self {
        self.instance_id = format!("{}", std::process::id());
        self
    }

    pub fn set_log_level(mut self, level: &str) -> Self {
        self.log_level = level.to_string();
        self
    }

    pub fn set_log_to_file(mut self, to_file: bool) -> Self {
        self.log_to_file = to_file;
        self
    }

    pub fn set_log_path(mut self, path: &str) -> Self {
        self.log_path = PathBuf::from(path);
        self
    }

    pub fn set_log_file_size(mut self, size: u64) -> Self {
        self.log_file_size = size;
        self
    }

    pub fn set_log_file_count(mut self, count: usize) -> Self {
        self.log_file_count = count;
        self
    }

    pub fn set_output_to_console(mut self, output_console: bool) -> Self {
        self.output_console = output_console;
        self
    }

    pub fn add_module_log(mut self, module_key: &str, log_name: &str) -> Self {
        self.module_logs.push((module_key.to_string(), log_name.to_string()));
        self
    }

    pub fn add_filter(mut self, filter: &str) -> Self {
        self.filter.push(filter.to_string());
        self
    }

    fn new_log(&self, log_name: &str, filters: Vec<String>) -> Result<Box<dyn log::Log>, FlexiLoggerError> {
        let mut logger = flexi_logger::Logger::try_with_env_or_str(self.log_level.as_str())?;
        if self.log_to_file {
            let mut base_name = self.app_name.clone();
            if !self.instance_id.is_empty() {
                base_name = format!("{}_{}", self.app_name, self.instance_id);
            }
            if !log_name.is_empty() {
                base_name = format!("{}_{}", base_name, log_name);
            }
            logger = logger.log_to_file(FileSpec::default().directory(self.log_path.as_path()).basename(base_name.as_str()))
                .rotate(Criterion::Size(self.log_file_size), // 文件大小达到 10MB 时轮转
                        Naming::Numbers, // 使用数字命名轮转文件
                        Cleanup::KeepLogFiles(self.log_file_count), // 保留最近 7 个日志文件
                );
        }
        if !self.output_console {
            logger = logger.duplicate_to_stderr(Duplicate::None);
        } else {
            logger = logger.duplicate_to_stderr(Duplicate::All);
        }

        logger = logger.filter(Box::new(SfoLogFilter::new(filters)));

        let (log, _) = logger.format(custom_format).build()?;
        Ok(log)
    }

    #[cfg(not(feature = "nolog"))]
    pub fn start(self) -> Result<(), FlexiLoggerError> {
        let main_log = self.new_log("", self.filter.clone())?;
        let mut module_logs = Vec::new();
        for (match_key, log_name) in self.module_logs.iter() {
            module_logs.push((match_key.clone(), self.new_log(log_name.as_str(), vec![])?));
        }
        
        let sfo_log = SfoLogger {
            main_logger: main_log,
            module_loggers: module_logs,
        };
        
        log::set_boxed_logger(Box::new(sfo_log))?;
        
        Ok(())
    }

    #[cfg(feature = "nolog")]
    pub fn start(self) -> Result<(), FlexiLoggerError> {
        Ok(())
    }
}

struct SfoLogFilter {
    filters: HashSet<String>,
}

impl SfoLogFilter {
    fn new(filters: Vec<String>) -> Self {
        Self {
            filters: filters.into_iter().map(|filter| filter.to_string()).collect(),
        }
    }
}

impl LogLineFilter for SfoLogFilter {
    fn write(&self, now: &mut DeferredNow, record: &Record, log_line_writer: &dyn LogLineWriter) -> std::io::Result<()> {
        if self.filters.contains(record.target()) {
            return Ok(());
        }

        log_line_writer.write(now, record)
    }
}

struct SfoLogger {
    main_logger: Box<dyn log::Log>,
    module_loggers: Vec<(String, Box<dyn log::Log>)>,
}

impl log::Log for SfoLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.main_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        for (module, log) in self.module_loggers.iter() {
            if record.metadata().target().starts_with(module) {
                log.log(record);
                break;
            }
        }
        self.main_logger.log(record);
    }

    fn flush(&self) {
        self.main_logger.flush();
        for (_, module_logger) in self.module_loggers.iter() {
            module_logger.flush();
        }
    }
}
