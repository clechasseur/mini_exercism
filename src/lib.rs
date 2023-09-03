//! # mini_exercism
//!
//! A lightweight crate to interact with [Exercism](https://exercism.org)'s website API.

pub mod api;
#[cfg(feature = "cli")]
pub mod cli;
pub mod core;
