use std::fmt;

use serde::{de, ser, Serialize};

/// See `:h command-range` for details.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum CommandRange {
    #[serde(rename(deserialize = "."), serialize_with = "serialize_as_true")]
    CurrentLine,

    #[serde(rename = "%")]
    WholeFile,

    #[serde(deserialize_with = "parse_string")]
    Count(u32),
}

impl<'de> de::Deserialize<'de> for CommandRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct CommandRangeVisitor;

        impl<'de> de::Visitor<'de> for CommandRangeVisitor {
            type Value = CommandRange;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("\".\", \"%\" or an integer")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match v {
                    "." => return Ok(Self::Value::CurrentLine),
                    "%" => return Ok(Self::Value::WholeFile),

                    other => {
                        if let Ok(n) = other.parse::<u32>() {
                            return Ok(Self::Value::Count(n));
                        }
                    },
                };

                Err(E::invalid_value(
                    de::Unexpected::Str(v),
                    &"\".\", \"%\" or an integer",
                ))
            }
        }

        deserializer.deserialize_str(CommandRangeVisitor)
    }
}

fn serialize_as_true<S>(serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    serializer.serialize_bool(true)
}