//! Pure validation functions for forms

/// Validates two passwords are non-empty and match
pub fn passwords_match(a: &str, b: &str) -> bool {
    !a.is_empty() && a == b
}

/// Validates email has local part, @, and dotted domain
pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> =
        email.split('@').collect();
    parts.len() == 2
        && !parts[0].is_empty()
        && parts[1].contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passwords_match_identical_strings() {
        assert!(passwords_match("abc123", "abc123"));
    }

    #[test]
    fn test_passwords_match_different_strings() {
        assert!(!passwords_match("abc", "xyz"));
    }

    #[test]
    fn test_passwords_match_empty_returns_false() {
        assert!(!passwords_match("", ""));
    }

    #[test]
    fn test_passwords_match_one_empty() {
        assert!(!passwords_match("abc", ""));
    }

    #[test]
    fn test_is_valid_email_standard_format() {
        assert!(is_valid_email("user@example.com"));
    }

    #[test]
    fn test_is_valid_email_subdomain() {
        assert!(is_valid_email(
            "a@sub.example.com"
        ));
    }

    #[test]
    fn test_is_valid_email_missing_at() {
        assert!(!is_valid_email("userexample.com"));
    }

    #[test]
    fn test_is_valid_email_missing_domain_dot() {
        assert!(!is_valid_email("user@example"));
    }

    #[test]
    fn test_is_valid_email_empty_string() {
        assert!(!is_valid_email(""));
    }

    #[test]
    fn test_is_valid_email_empty_local_part() {
        assert!(!is_valid_email("@example.com"));
    }

    #[test]
    fn test_is_valid_email_double_at() {
        assert!(!is_valid_email("a@b@c.com"));
    }
}
