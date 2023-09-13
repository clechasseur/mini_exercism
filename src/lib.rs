//! A lightweight crate to interact with the [Exercism website](https://exercism.org)'s APIs.
//!
//! # TOC
//!
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
//! ## Installing
//!
//! Add [mini_exercism](crate) to your dependencies:
//!
//! ```toml
//! [dependencies]
//! mini_exercism = "0"
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
//! To create a client, either use its `new` or `default` methods to create a default instance, or
//! use its `builder` method to construct one:
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! fn get_default_client() -> api::v2::Client {
//!     api::v2::Client::new()
//! }
//!
//! fn get_custom_client() -> api::v2::Client {
//!     let mut builder = api::v2::Client::builder();
//!     // ... customize API client with builder ...
//!
//!     builder.build()
//! }
//! ```
//!
//! Note that the creation of a default client results in the creation of a default internal
//! HTTP client, which can result in a [panic in the `reqwest crate`](https://docs.rs/reqwest/latest/reqwest/struct.Client.html#method.new).
//! This is a rare occurrence, but if you need to handle these errors, you can build the HTTP
//! client manually - see [`Custom HTTP client`](#custom-http-client).
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
//! async fn main() {
//!     let client = api::v2::Client::new();
//!     let tracks = client.get_tracks(None).await;
//!     // ...
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
//! async fn print_language_tracks() -> mini_exercism::core::Result<()> {
//!     let client = api::v2::Client::new();
//!
//!     let tracks = client.get_tracks(None).await?.tracks;
//!     for track in &tracks {
//!         println!("Exercism language track: {}", track.title);
//!     }
//!
//!     Ok(())
//! }
//!
//! async fn print_solutions(track: &str) -> mini_exercism::core::Result<()> {
//!     let client = api::v2::Client::new();
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
//! API clients use [`Credentials`](core::Credentials) to perform requests as a specific user.
//! Some requests can work both anonymously and authenticated (behaving differently depending on
//! which is used), others require authentication to work.
//!
//! [`Credentials`](core::Credentials) use [Exercism](https://exercism.org) API tokens to identify
//! a user. This is the token that is used for the [Exercism CLI application](https://exercism.org/docs/using/solving-exercises/working-locally).
//! It can be fetched from the [Exercism's user settings page](https://exercism.org/settings/api_cli).
//!
//! To pass [`Credentials`](core::Credentials) to an API client, use its `builder`:
//!
//! ```no_run
//! use mini_exercism::api;
//! use mini_exercism::core::Credentials;
//!
//! fn get_api_client() -> api::v2::Client {
//!     let credentials = Credentials::from_api_token("SOME_API_TOKEN");
//!     api::v2::Client::builder().credentials(credentials).build()
//! }
//! ```
//!
//! ## CLI credentials
//!
//! This crate provides a helper function to fetch the [`Credentials`](core::Credentials) used by
//! the currently-installed [Exercism CLI application](https://exercism.org/docs/using/solving-exercises/working-locally).
//! In order to use this, you need to enable the `cli` feature:
//!
//! ```toml
//! [dependencies]
//! mini_exercism = { version = "0", features = ["cli"] }
//! ```
//!
//! Then, you can fetch CLI credentials and use them to perform API requests. Note that it's
//! possible for the method to fail to find credentials if the CLI is not installed, for instance.
//!
//! ```no_run
//! use mini_exercism::api;
//! use mini_exercism::cli::get_cli_credentials;
//!
//! fn get_api_client() -> api::v2::Client {
//!     let mut client_builder = api::v2::Client::builder();
//!
//!     let cli_credentials = get_cli_credentials();
//!     if let Ok(credentials) = cli_credentials {
//!         client_builder.credentials(credentials);
//!     } else {
//!         // Find some other way to fetch credentials, or perform queries anonymously
//!     }
//!
//!     client_builder.build()
//! }
//! ```
//!
//! ## Custom HTTP client
//!
//! Internally, [`mini_exercism`](crate) uses the [`reqwest`](https://crates.io/crates/reqwest)
//! library to perform HTTP calls. Unless overridden, API clients will create a default HTTP client.
//! If you need to customize the behavior of the HTTP client, you can use the API client's `builder`
//! to specify a different HTTP client:
//!
//! ```no_run
//! use mini_exercism::api;
//!
//! fn get_api_client() -> mini_exercism::core::Result<api::v2::Client> {
//!     let http_client_builder = reqwest::Client::builder();
//!     // ... customize HTTP client with `http_client_builder` here ...
//!     let http_client = http_client_builder.build()?;
//!
//!     Ok(api::v2::Client::builder().http_client(http_client).build())
//! }
//! ```
//!
//! Creating the HTTP client via its `builder` also has the advantage of being able to handle
//! errors that may arise when doing so instead of [panicking](https://docs.rs/reqwest/latest/reqwest/struct.Client.html#method.new).
//!
//! ## Crate status
//!
//! Currently, this crate is a bit minimalistic and does not implement all the [Exercism](https://exercism.org)
//! API endpoints. To suggest new endpoints to add, you can enter an [issue](https://github.com/clechasseur/mini_exercism/issues).
//! Or, even better, don't hesitate to submit a [pull request](https://github.com/clechasseur/mini_exercism/pulls)! 😁
//!
//! ## Minimum Rust version
//!
//! TODO: complete this section

#![deny(missing_docs)]
#![deny(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![cfg_attr(any(nightly_rustc, docsrs), feature(doc_cfg))]

pub mod api;
#[cfg(feature = "cli")]
#[cfg_attr(any(nightly_rustc, docsrs), doc(cfg(feature = "cli")))]
pub mod cli;
pub mod core;
