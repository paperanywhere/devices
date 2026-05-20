//! Per-vendor board modules.
//!
//! Each submodule owns the `Board` consts for one hardware vendor. The
//! crate root re-exports the named consts so consumers don't have to
//! reach through the vendor path. New vendors go here as a new module
//! + a re-export line in `lib.rs`.

pub mod seeed;
pub mod soldered;
pub mod waveshare;
