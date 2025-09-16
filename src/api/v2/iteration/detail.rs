use std::any::type_name;
use std::fmt::Formatter;
use std::marker::PhantomData;

use serde::de::value::MapAccessDeserializer;
use serde::de::{MapAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

pub fn deserialize_optional_feedback<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    deserializer.deserialize_option(OptionalFeedbackVisitor::new())
}

struct OptionalFeedbackVisitor<T>(PhantomData<T>);

impl<T> OptionalFeedbackVisitor<T> {
    fn new() -> Self {
        Self(PhantomData)
    }

    fn feedback_type_name(&self) -> &'static str {
        type_name::<T>().split("::").last().unwrap()
    }
}

impl<'de, T> Visitor<'de> for OptionalFeedbackVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "an optional feedback struct (of type {}) or the string 'not_queued'",
            self.feedback_type_name(),
        )
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // When deserializing a deleted iteration, the Exercism v2 API sends a string
        // value of 'not_queued' for representer and analyzer feedback. This might be
        // a bug as that is a value of the `tests_status` enum. Regardless, we need
        // to handle this case by considering it like it was not specified (e.g. `null`).
        //
        // See https://github.com/clechasseur/exercism-website/blob/0b598e464de39f6cfc53c15ed80879f2e1a4aade/app/serializers/serialize_iteration.rb#L57-L58
        if v == "not_queued" { Ok(None) } else { Err(E::invalid_value(Unexpected::Str(v), &self)) }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        Ok(Some(T::deserialize(MapAccessDeserializer::new(map))?))
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    mod deserialize_optional_feedback {
        use assert_matches::assert_matches;

        use super::*;
        use crate::api::v2::submission::analysis::AnalyzerCommentType::Informative;
        use crate::api::v2::submission::analysis::{
            AnalyzerComment, AnalyzerFeedback, FeedbackAuthor, RepresenterFeedback,
        };
        use crate::api::v2::user::Flair::LifetimeInsider;

        #[derive(Debug, PartialEq, Eq, Deserialize)]
        struct TestIteration {
            #[serde(default, deserialize_with = "deserialize_optional_feedback")]
            pub representer_feedback: Option<RepresenterFeedback>,

            #[serde(default, deserialize_with = "deserialize_optional_feedback")]
            pub analyzer_feedback: Option<AnalyzerFeedback>,
        }

        #[test]
        fn test_null() {
            let json = r#"{
                "representer_feedback": null,
                "analyzer_feedback": null
            }"#;

            let expected = TestIteration { representer_feedback: None, analyzer_feedback: None };
            let actual: TestIteration = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_not_queued() {
            let json = r#"{
                "representer_feedback": "not_queued",
                "analyzer_feedback": "not_queued"
            }"#;

            let expected = TestIteration { representer_feedback: None, analyzer_feedback: None };
            let actual: TestIteration = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_invalid_string_value() {
            let json = r#"{
                "representer_feedback": "foo",
                "analyzer_feedback": "bar"
            }"#;

            assert_matches!(serde_json::from_str::<TestIteration>(json), Err(err) => {
                assert!(err.to_string().contains("an optional feedback struct (of type RepresenterFeedback) or the string 'not_queued'"));
            });
        }

        #[test]
        fn test_map() {
            let json = r#"{
                "representer_feedback": {
                    "html": "<p>This looks great. Thank you for the submission!</p>\n",
                    "author": {
                        "name": "John Smith",
                        "reputation": 12345,
                        "flair": "lifetime_insider",
                        "avatar_url": "https://assets.exercism.org/avatars/31337/0",
                        "profile_url": "https://exercism.org/profiles/jsmith_mini_exercism"
                    },
                    "editor": null
                },
                "analyzer_feedback": {
                    "summary": null,
                    "comments": [
                        {
                            "type": "informative",
                            "html": "<p>Nice work using <code>impl Display</code> to implement to_string.</p>\n"
                        },
                        {
                            "type": "informative",
                            "html": "<p>Nice work using rem_euclid!</p>\n"
                        },
                        {
                            "type": "informative",
                            "html": "<p>(Some people don't bother storing the hours in the struct which simplifies things a bit.)</p>\n"
                        }
                    ]
                }
            }"#;

            let expected = TestIteration {
                representer_feedback: Some(RepresenterFeedback {
                    html: "<p>This looks great. Thank you for the submission!</p>\n".into(),
                    author: FeedbackAuthor {
                        name: "John Smith".into(),
                        reputation: 12345,
                        flair: Some(LifetimeInsider),
                        avatar_url: "https://assets.exercism.org/avatars/31337/0".into(),
                        profile_url: Some("https://exercism.org/profiles/jsmith_mini_exercism".into()),
                    },
                    editor: None,
                }),
                analyzer_feedback: Some(AnalyzerFeedback {
                    summary: None,
                    comments: vec![
                        AnalyzerComment {
                            comment_type: Informative,
                            html: "<p>Nice work using <code>impl Display</code> to implement to_string.</p>\n".into(),
                        },
                        AnalyzerComment {
                            comment_type: Informative,
                            html: "<p>Nice work using rem_euclid!</p>\n".into(),
                        },
                        AnalyzerComment {
                            comment_type: Informative,
                            html: "<p>(Some people don't bother storing the hours in the struct which simplifies things a bit.)</p>\n".into(),
                        },
                    ],
                }),
            };
            let actual: TestIteration = serde_json::from_str(json).unwrap();
            assert_eq!(expected, actual);
        }
    }
}
