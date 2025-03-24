/// Error types.
pub mod error {
    /// Error from a TryFrom or FromStr implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Life
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "life.json",
///  "title": "Life",
///  "type": "object",
///  "required": [
///    "lifeBackground",
///    "lifeClass",
///    "lifeTrinket"
///  ],
///  "properties": {
///    "lifeBackground": true,
///    "lifeClass": true,
///    "lifeTrinket": true
///  },
///  "additionalProperties": false,
///  "version": "1.0.0"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Life {
    #[serde(rename = "lifeBackground")]
    pub life_background: ::serde_json::Value,
    #[serde(rename = "lifeClass")]
    pub life_class: ::serde_json::Value,
    #[serde(rename = "lifeTrinket")]
    pub life_trinket: ::serde_json::Value,
}
impl From<&Life> for Life {
    fn from(value: &Life) -> Self {
        value.clone()
    }
}
impl Life {
    pub fn builder() -> builder::Life {
        Default::default()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Life {
        life_background: Result<::serde_json::Value, String>,
        life_class: Result<::serde_json::Value, String>,
        life_trinket: Result<::serde_json::Value, String>,
    }
    impl Default for Life {
        fn default() -> Self {
            Self {
                life_background: Err(
                    "no value supplied for life_background".to_string(),
                ),
                life_class: Err("no value supplied for life_class".to_string()),
                life_trinket: Err("no value supplied for life_trinket".to_string()),
            }
        }
    }
    impl Life {
        pub fn life_background<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.life_background = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for life_background: {}", e)
                });
            self
        }
        pub fn life_class<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.life_class = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for life_class: {}", e)
                });
            self
        }
        pub fn life_trinket<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<::serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.life_trinket = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for life_trinket: {}", e)
                });
            self
        }
    }
    impl std::convert::TryFrom<Life> for super::Life {
        type Error = super::error::ConversionError;
        fn try_from(value: Life) -> Result<Self, super::error::ConversionError> {
            Ok(Self {
                life_background: value.life_background?,
                life_class: value.life_class?,
                life_trinket: value.life_trinket?,
            })
        }
    }
    impl From<super::Life> for Life {
        fn from(value: super::Life) -> Self {
            Self {
                life_background: Ok(value.life_background),
                life_class: Ok(value.life_class),
                life_trinket: Ok(value.life_trinket),
            }
        }
    }
}
