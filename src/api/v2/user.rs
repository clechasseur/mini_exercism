//! Types related to users returned by the [Exercism website](https://exercism.org) v2 API.

use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

/// Possible values for a user's "flair", which is a kind of special status.
///
/// A user's "flair" is represented next to their name when displayed on the website, for example
/// in mentoring sessions, on their public profile, etc. Not all users have a "flair".
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Flair {
    /// Reserved for [Exercism's leadership team](https://exercism.org/about/team#:~:text=help%20to%20others.-,Leadership%20Team,-Our%20leadership%20team).
    ///
    /// Visually represented by an [orange star](https://assets.exercism.org/assets/icons/staff-flair-41fb1beb636fd38666afd45b29601cce8c16fa77.svg)
    /// (apparently, it uses the same graphic as [`Staff`](Self::Staff)).
    Founder,

    /// Reserved for [Exercism's staff](https://exercism.org/about/team#:~:text=Twitter-,Staff,-Our%20staff%20manage).
    ///
    /// Visually represented by an [orange star](https://assets.exercism.org/assets/icons/staff-flair-41fb1beb636fd38666afd45b29601cce8c16fa77.svg).
    Staff,

    /// An [Exercism insider](https://exercism.org/insiders).
    ///
    /// Visually represented by a [purple heart](https://assets.exercism.org/assets/icons/insiders-d0418ec8b59d21a8852f7326404fa20b2d21785d.svg).
    Insider,

    /// A person made [Exercism insider](https://exercism.org/insiders) for life.
    ///
    /// When the Insider program was created, some people got a special "lifetime insider" status
    /// due to their extensive contributions to Exercism in the past. This status can no longer be
    /// obtained today (AFAWK).
    ///
    /// Visually represented by a [purple heart surrounded by shiny stars](https://assets.exercism.org/assets/icons/lifetime-insiders-0422ed0ba9e5ac00e64a126895ad2c6caa17b4d3.svg).
    LifetimeInsider,

    /// Unknown flair.
    ///
    /// Included so that if new "flairs" are introduced in the website API later, this crate won't break (hopefully).
    #[serde(skip_serializing, other)]
    Unknown,
}
