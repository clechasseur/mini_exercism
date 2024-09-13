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
    pub fn new() -> Self {
        Self(PhantomData)
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
            type_name::<T>()
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
        if v == "not_queued" {
            Ok(None)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
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
