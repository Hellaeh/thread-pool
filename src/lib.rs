//! A simple thread pool implementation
#![warn(missing_docs)]
#![feature(panic_update_hook)]
#![feature(thread_local)]

pub use thread_pool::ThreadPool;

mod thread_pool;
mod utils;
