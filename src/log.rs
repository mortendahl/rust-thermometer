//! This module provides custom logging macros (info, error, debug). They wraps
//! slog_info, ... macros with our root logger as a first argument.
//!
//! In other words, we're not forced to pass the logger around.
use config;
use slog::{Drain, Logger};
use slog_async::Async;
use slog_term::{FullFormat, TermDecorator};

pub fn create_root_logger() -> Logger {
    let values = o!(
        "version" => config::PKG_VERSION,
    );

    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let console_drain = Async::new(drain).build().fuse();

    Logger::root(console_drain, values)
}
