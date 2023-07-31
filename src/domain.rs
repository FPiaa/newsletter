use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

#[derive(Debug)]
pub struct SubscriberName(String);

// TODO: Discover how to deserialize this struct based on special conditions,
// this way we will be able to use axum extractor system to ease our life
impl SubscriberName {
    pub fn parse(possible_name: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = possible_name.trim().is_empty();

        let is_too_long = possible_name.graphemes(true).count() > 256;

        let forbidden_caracters = ['/', '(', ')', '{', '}', '"', '\\', '<', '>', '='];

        let contains_forbidden_caracter = possible_name
            .chars()
            .any(|c| forbidden_caracters.contains(&c));

        if is_too_long || is_empty_or_whitespace || contains_forbidden_caracter {
            return Err("{possible_name} is not a valid subscriber name.".to_string());
        }

        Ok(Self(possible_name))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn subscriber_name_accepts_a_string_with_256_graphemes() {
        let s = "a".repeat(256);
        assert_ok!(SubscriberName::parse(s));
    }

    #[test]
    fn subscriber_name_rejects_string_longer_than_256_graphemes() {
        let s = "a".repeat(257);
        assert_err!(SubscriberName::parse(s));
    }

    #[test]
    fn subscriber_name_rejects_string_containing_only_whitespaces() {
        let s = "    ".to_string();
        assert_err!(SubscriberName::parse(s));
    }

    #[test]
    fn subscriber_name_rejects_empty_string() {
        let s = "".to_string();
        assert_err!(SubscriberName::parse(s));
    }

    #[test]
    fn subscriber_name_rejects_invalid_characters() {
        let forbidden_caracters = ['/', '(', ')', '{', '}', '"', '\\', '<', '>', '='];
        for name in &forbidden_caracters {
            let s = name.to_string();
            assert_err!(SubscriberName::parse(s));
        }
    }

    #[test]
    fn subscriber_name_accepts_valid_name() {
        let valid_names = ["this is a long name", "this is another name", "bo", "jo"];
        for name in valid_names {
            let s = name.to_string();
            assert_ok!(SubscriberName::parse(s));
        }
    }
}
