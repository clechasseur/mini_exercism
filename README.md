# mini_exercism

[![CI](https://github.com/clechasseur/mini_exercism/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/clechasseur/mini_exercism/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/clechasseur/mini_exercism/branch/main/graph/badge.svg?token=qSFdAkbb8U)](https://codecov.io/gh/clechasseur/mini_exercism) [![Security audit](https://github.com/clechasseur/mini_exercism/actions/workflows/audit-check.yml/badge.svg?branch=main)](https://github.com/clechasseur/mini_exercism/actions/workflows/audit-check.yml)<br/>
[![crates.io](https://img.shields.io/crates/v/mini_exercism.svg)](https://crates.io/crates/mini_exercism) [![downloads](https://img.shields.io/crates/d/mini_exercism.svg)](https://crates.io/crates/mini_exercism) [![docs.rs](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/mini_exercism)

Minimalistic Rust library to interact with the [Exercism.org](https://exercism.org) APIs.

## Exerci-what?

[Exercism](https://exercism.org) is a free, not-for-profit platform to learn new programming languages. It supports a web editor for solving exercises, mentoring with real humans and a lot more. For more information, see [its about page](https://exercism.org/about).

## Installing

Add `mini_exercism` to your dependencies:

```toml
[dependencies]
mini_exercism = "0"
```

or by running:

```bash
cargo add mini_exercism
```

## Example

```rust
use mini_exercism::api;
use mini_exercism::api::v2::ExerciseFilters;
use mini_exercism::core::Credentials;

async fn get_published_solution_uuids(
    api_token: &str,
    track: &str,
) -> mini_exercism::core::Result<Vec<String>> {
    let credentials = Credentials::from_api_token(api_token);
    let client = api::v2::Client::builder()
        .credentials(credentials)
        .build();

    let filters = ExerciseFilters::builder()
        .include_solutions(true)
        .build();
    let solutions = client
        .get_exercises(track, Some(filters))
        .await?
        .solutions;

    Ok(solutions
        .into_iter()
        .filter(|solution| solution.published_at.is_some())
        .map(|solution| solution.uuid)
        .collect())
}
```

For more information, see [the docs](https://docs.rs/mini_exercism).

## Minimum Rust version

`mini_exercism` currently builds on Rust 1.63 or newer.
