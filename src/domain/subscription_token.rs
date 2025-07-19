use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriptionToken(String);

impl SubscriptionToken {
    pub const TOKEN_LENGTH: usize = 25;

    pub fn generate() -> Self {
        let token: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(Self::TOKEN_LENGTH)
            .map(char::from)
            .collect();

        Self(token)
    }

    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty = s.trim().is_empty();
        let is_wrong_length = s.len() != Self::TOKEN_LENGTH;
        let is_not_alphanumeric = !s.chars().all(|c| c.is_ascii_alphanumeric());

        if is_empty || is_wrong_length || is_not_alphanumeric {
            Err(format!("Invalid subscription token: '{}'", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriptionToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use rand::distributions::Alphanumeric;

    #[derive(Clone, Debug)]
    struct ValidTokenFixture(pub String);

    impl Arbitrary for ValidTokenFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let s: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(SubscriptionToken::TOKEN_LENGTH)
                .map(char::from)
                .collect();
            ValidTokenFixture(s)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_tokens_parse(fixture: ValidTokenFixture) -> bool {
        SubscriptionToken::parse(fixture.0).is_ok()
    }

    #[test]
    fn empty_token_is_rejected() {
        let empty = "".to_string();
        assert!(SubscriptionToken::parse(empty).is_err());
    }

    #[test]
    fn wrong_length_token_is_rejected() {
        let short = "short".to_string();
        let long = "a".repeat(26);
        assert!(SubscriptionToken::parse(short).is_err());
        assert!(SubscriptionToken::parse(long).is_err());
    }

    #[test]
    fn token_with_invalid_chars_is_rejected() {
        let bad_chars = "abcd$%^&*()efghijklmnopqrstu".to_string();
        assert!(SubscriptionToken::parse(bad_chars).is_err());
    }
}
