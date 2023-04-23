#![warn(clippy::all, rust_2018_idioms)]
// Maybe remove these when the code is more mature
#![allow(dead_code)]
#![allow(unreachable_patterns)]
mod app;
pub use app::HopfiledNetsApp;
