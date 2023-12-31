use serde::Deserialize;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<String> for SubscriberName {
    //TODO: use string as error type for now
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_empty_or_whitespace = value.trim().is_empty();

        let is_too_long = value.graphemes(true).count() > 256;

        let forbidden_caracters = ['/', '(', ')', '{', '}', '"', '\\', '<', '>', '='];

        let contains_forbidden_caracter = value.chars().any(|c| forbidden_caracters.contains(&c));

        if is_too_long || is_empty_or_whitespace || contains_forbidden_caracter {
            return Err("{value} is not a valid subscriber name.".to_string());
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::subscriber_name::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn subscriber_name_accepts_a_string_with_256_graphemes() {
        let s = "a".repeat(256);
        assert_ok!(SubscriberName::try_from(s));
    }

    #[test]
    fn subscriber_name_rejects_string_longer_than_256_graphemes() {
        let s = "a".repeat(257);
        assert_err!(SubscriberName::try_from(s));
    }

    #[test]
    fn subscriber_name_rejects_string_containing_only_whitespaces() {
        let s = "    ".to_string();
        assert_err!(SubscriberName::try_from(s));
    }

    #[test]
    fn subscriber_name_rejects_empty_string() {
        let s = "".to_string();
        assert_err!(SubscriberName::try_from(s));
    }

    #[test]
    fn subscriber_name_rejects_invalid_characters() {
        let forbidden_caracters = ['/', '(', ')', '{', '}', '"', '\\', '<', '>', '='];
        for name in &forbidden_caracters {
            let s = name.to_string();
            assert_err!(SubscriberName::try_from(s));
        }
    }

    #[test]
    fn subscriber_name_accepts_valid_name() {
        let valid_names = ["this is a long name", "this is another name", "bo", "jo"];
        for name in valid_names {
            let s = name.to_string();
            assert_ok!(SubscriberName::try_from(s));
        }
    }
}
