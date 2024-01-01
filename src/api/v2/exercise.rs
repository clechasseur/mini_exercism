//! Types related to exercises returned by the [Exercism website](https://exercism.org) v2 API.

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display};

/// A single exercise returned by the [Exercism website](https://exercism.org) v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Exercise {
    /// Name of the exercise.
    ///
    /// This is an internal name, like `forth`. Also called `slug`.
    #[serde(rename = "slug")]
    pub name: String,

    /// Type of exercise.
    #[serde(rename = "type")]
    pub exercise_type: Type,

    /// Exercise title.
    ///
    /// This is a textual representation of the title, like `Forth`.
    pub title: String,

    /// URL of the icon representing this exercise on the [Exercism website](https://exercism.org).
    pub icon_url: String,

    /// Exercise difficulty rating.
    pub difficulty: Difficulty,

    /// Short description of the exercise.
    pub blurb: String,

    /// Whether this is an "exernal" exercise.
    ///
    /// This is used to indicate exercises that are not tied to a user. When returned by the
    /// website API, this indicates that the request was performed anonymously.
    pub is_external: bool,

    /// Whether this exercise has been unlocked by the user.
    ///
    /// Will always be `false` when exercises are queried anonymously.
    pub is_unlocked: bool,

    /// Whether this is the next recommended exercise for the user in the language track.
    ///
    /// Will always be `false` when exercises are queried anonymously.
    pub is_recommended: bool,

    /// Links pertaining to the exercise.
    pub links: Links,
}

/// Possible type of exercise on the [Exercism website](https://exercism.org).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Type {
    /// Tutorial exercise.
    ///
    /// Currently only known to apply to `hello-world`.
    Tutorial,

    /// Concept exercise, e.g. an exercise tied to a concept on the language track's syllabus.
    Concept,

    /// Practice exercise.
    ///
    /// Most exercise are in this category.
    Practice,

    /// Unknown exercise type.
    ///
    /// Included so that if new exercise types are introduced in the website API later,
    /// this crate will not break (hopefully).
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Possible difficulty rating of an exercise on the [Exercism website](https://exercism.org).
///
/// Internally, exercises have a difficulty rating between 1 and 10 (inclusive); however, on the
/// website, this is only represented by specific, named difficulty ratings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Difficulty {
    /// Easy exercise.
    ///
    /// Internally, an exercise with a difficulty rating between 1 and 3 (inclusive).
    Easy,

    /// Medium exercise.
    ///
    /// Internally, an exercise with a difficulty rating between 4 and 7 (inclusive).
    Medium,

    /// Hard exercise.
    ///
    /// Internally, an exercise with a difficulty rating above 7.
    Hard,

    /// Unknown difficulty.
    ///
    /// Included so that if new exercise difficulty ratings are introduced in the website API later,
    /// this crate will not break (hopefully).
    #[serde(skip_serializing, other)]
    Unknown,
}

/// Links pertaining to an [Exercism](https://exercism.org) exercise returned by the v2 API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Links {
    /// Path of the exercise on the [Exercism website](https://exercism.org), without the domain name.
    #[serde(rename = "self")]
    pub self_path: String,
}
