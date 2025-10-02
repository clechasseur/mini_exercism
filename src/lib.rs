//! A lightweight crate to interact with the [Exercism website](https://exercism.org)'s APIs.
//!
//! # TOC
//!
//! - [`What is Exercism?`](#what-is-exercism)
//! - [`Installing`](#installing)
//! - [`API clients`](#api-clients)
//! - [`Async methods`](#async-methods)
//! - [`Example`](#example)
//! - [`Credentials`](#credentials)
//! - [`CLI credentials`](#cli-credentials)
//! - [`Custom HTTP client`](#custom-http-client)
//! - [`Crate status`](#crate-status)
//! - [`Minimum Rust version`](#minimum-rust-version)
//!
//! ## What is Exercism?
//!
//! [Exercism](https://exercism.org) is a free, not-for-profit platform to learn new programming
//! languages. It supports a web editor for solving exercises, mentoring with real humans and
//! a lot more. For more information, see [its about page](https://exercism.org/about).
//!
//! ## Installing
//!
//! Add [mini_exercism](crate) to your dependencies:
//!
//! ```toml
//! [dependencies]
//! mini_exercism = "6.0.0"
//! ```
//!
//! or by running:
//!
//! ```bash
//! cargo add mini_exercism
//! ```
//!
//! ## API clients
//!
//! To interact with an [Exercism](https://exericms.org) API, you can simply use one of
//! the provided API clients. Each API has its own client:
//!
//! - [`api::v1::Client`]
//! - [`api::v2::Client`]
//!
//! To create a client, either use its `new` method to create a default instance, or use its
//! `builder` method to construct one:
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! fn get_default_client() -> anyhow::Result<api::v2::Client> {
//!     Ok(api::v2::Client::new()?)
//! }
//!
//! fn get_custom_client() -> anyhow::Result<api::v2::Client> {
//!     let mut builder = api::v2::Client::builder();
//!     // ... customize API client with builder ...
//!
//!     Ok(builder.build()?)
//! }
//! ```
//!
//! ## Async methods
//!
//! The client methods used to query the APIs for information are `async`. As such, in order to
//! call them, you will need to use the `await` keyword. For more information on async programming
//! in Rust, see [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/).
//!
//! Asynchronous programming in Rust requires an _async runtime_. Runtimes handle the execution
//! of asynchronous code and the callbacks.
//!
//! One popular async runtime is [Tokio](https://tokio.rs/). It offers a simple way to write
//! programs that support asynchronous code. You can use the [`#[tokio::main]`](https://docs.rs/tokio/latest/tokio/attr.main.html)
//! attribute to make your `main` function `async`, allowing the use of `await`:
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = api::v2::Client::new()?;
//!     let tracks = client.get_tracks(None).await?;
//!     // ...
//!
//!     Ok(())
//! }
//! ```
//!
//! [Tokio](https://tokio.rs/) offers many customization options; see the [docs](https://docs.rs/tokio/latest/tokio/index.html)
//! for more details.
//!
//! ## Example
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! async fn print_language_tracks() -> anyhow::Result<()> {
//!     let client = api::v2::Client::new()?;
//!
//!     let tracks = client.get_tracks(None).await?.tracks;
//!     for track in &tracks {
//!         println!("Exercism language track: {}", track.title);
//!     }
//!
//!     Ok(())
//! }
//!
//! async fn print_solutions(track: &str) -> anyhow::Result<()> {
//!     let client = api::v2::Client::new()?;
//!
//!     let solutions = client.get_exercises(track, None).await?.solutions;
//!     for solution in &solutions {
//!         println!(
//!             "Solution for exercise {}, public URL: {}",
//!             solution.exercise.title, solution.public_url,
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Credentials
//!
//! API clients use [`Credentials`] to perform requests as a specific user. Some requests can
//! work both anonymously and authenticated (behaving differently depending on which is used),
//! others require authentication to work.
//!
//! [`Credentials`] use [Exercism](https://exercism.org) API tokens to identify a user. This is the
//! token that is used for the [Exercism CLI application](https://exercism.org/docs/using/solving-exercises/working-locally).
//! It can be fetched from the [Exercism's user settings page](https://exercism.org/settings/api_cli).
//!
//! To pass [`Credentials`] to an API client, use its `builder`:
//!
//! ```no_run
//! use mini_exercism::api;
//! use mini_exercism::core::Credentials;
//!
//! fn get_api_client() -> anyhow::Result<api::v2::Client> {
//!     let credentials = Credentials::from_api_token("SOME_API_TOKEN");
//!     Ok(api::v2::Client::builder()
//!         .credentials(credentials)
//!         .build()?)
//! }
//! ```
//!
//! ## CLI credentials
//!
//! This crate provides a helper function to fetch the [`Credentials`] used by the currently-installed
//! [Exercism CLI application](https://exercism.org/docs/using/solving-exercises/working-locally).
//! In order to use this, you need to enable the `cli` feature:
//!
//! ```toml
//! [dependencies]
//! mini_exercism = { version = "6.0.0", features = ["cli"] }
//! ```
//!
//! Then, you can fetch CLI credentials and use them to perform API requests. Note that it's
//! possible for the method to fail to find credentials if the CLI is not installed, for instance.
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! fn get_api_client() -> anyhow::Result<api::v2::Client> {
//!     let mut client_builder = api::v2::Client::builder();
//!
//!     #[cfg(feature = "cli")]
//!     if let Ok(credentials) = mini_exercism::cli::get_cli_credentials() {
//!         client_builder.credentials(credentials);
//!     } else {
//!         // Find some other way to fetch credentials, or perform queries anonymously
//!     }
//!
//!     Ok(client_builder.build()?)
//! }
//! ```
//!
//! ## Custom HTTP client
//!
//! Internally, [mini_exercism](crate) uses the [reqwest](https://crates.io/crates/reqwest)
//! library to perform HTTP calls (whose types are re-exported through the [`mini_exercism::http`](http)
//! module). Unless overridden, API clients will create a default HTTP client.  If you need to customize the
//! behaviour of the HTTP client, you can use the API client's `builder` to specify a different HTTP client:
//!
//! ```no_run
//! use mini_exercism::api;
//! use mini_exercism::http;
//!
//! fn get_api_client() -> anyhow::Result<api::v2::Client> {
//!     let http_client_builder = http::Client::builder();
//!     // ... customize HTTP client with `http_client_builder` here ...
//!     let http_client = http_client_builder.build()?;
//!
//!     Ok(api::v2::Client::builder()
//!         .http_client(http_client)
//!         .build()?)
//! }
//!
//! // Another possible way:
//! fn get_api_client_too() -> anyhow::Result<api::v2::Client> {
//!     Ok(api::v2::Client::builder()
//!         .build_http_client(|builder| {
//!             // ... customize HTTP client with `builder` here ...
//!             builder
//!         })
//!         .build()?)
//! }
//! ```
//!
//! ## Retry support
//!
//! Recently (circa 2025), the Exercism API started throttling incoming requests much more
//! aggressively to fight a bot situation. Because of this, it is now sometimes necessary to
//! retry API requests should they be throttled.
//!
//! By default, API clients will retry API requests up to 5 times (with exponential backoff)
//! before giving up. To configure this, you can either use the API client's `builder`'s
//! `num_retries` or `retry_policy` methods:
//!
//! ```no_run
//! use std::time::Duration;
//!
//! use mini_exercism::api;
//! use mini_exercism::http::retry::Jitter;
//! use mini_exercism::http::retry::policies::ExponentialBackoff;
//!
//! fn get_api_client() -> anyhow::Result<api::v2::Client> {
//!     Ok(api::v2::Client::builder().num_retries(2).build()?)
//! }
//!
//! // Or fully customize the retry policy:
//! fn get_api_client_too() -> anyhow::Result<api::v2::Client> {
//!     let retry_policy = ExponentialBackoff::builder()
//!         .retry_bounds(Duration::from_secs(1), Duration::from_secs(30))
//!         .jitter(Jitter::Bounded)
//!         .build_with_max_retries(2);
//!
//!     Ok(api::v2::Client::builder()
//!         .retry_policy(retry_policy)
//!         .build()?)
//! }
//! ```
//!
//! ## Crate status
//!
//! Currently, this crate is a bit minimalistic and does not implement all the [Exercism](https://exercism.org)
//! API endpoints. To suggest new endpoints to add, you can enter an [issue](https://github.com/clechasseur/mini_exercism/issues).
//! Or, even better, don't hesitate to submit a [pull request](https://github.com/clechasseur/mini_exercism/pulls)! 😁
//!
//! ## Minimum Rust version
//!
//! [mini_exercism](crate) currently builds on Rust 1.85 or newer.
//!
//! [`Credentials`]: core::Credentials

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

// Re-export `reqwest`, `reqwest-middleware` and `reqwest-retry` in a `http` module
#[doc(hidden)]
pub mod http {
    pub use reqwest::*;
    pub use reqwest_middleware as middleware;
    pub use reqwest_retry as retry;
}

// Re-export `Bytes` and some `futures` types in a `stream` module
#[doc(hidden)]
pub mod stream {
    pub use bytes::Bytes;
    pub use futures::{Stream, StreamExt, TryStreamExt};
}

pub mod api;
#[cfg(feature = "cli")]
pub mod cli;
pub mod core;

pub use crate::core::Error;
pub use crate::core::Result;
