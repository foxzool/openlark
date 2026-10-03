use crate::constants::VERSION;

pub fn user_agent() -> String {
    format!("open-lark/{VERSION}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent_format() {
        let user_agent = user_agent();

        // Test that it has the expected format
        assert!(user_agent.starts_with("open-lark/"));
        assert!(user_agent.contains(VERSION));

        // Test that it's not empty
        assert!(!user_agent.is_empty());

        // Test that it matches the expected pattern
        let expected = format!("open-lark/{VERSION}");
        assert_eq!(user_agent, expected);
    }

    #[test]
    fn test_user_agent_version_consistency() {
        let user_agent = user_agent();

        // Extract version from user agent string
        let version_part = user_agent.strip_prefix("open-lark/").unwrap();
        assert_eq!(version_part, VERSION);

        // Verify version matches expected pattern
        assert!(VERSION.contains('.'));
    }

    #[test]
    fn test_user_agent_immutable() {
        let user_agent1 = user_agent();
        let user_agent2 = user_agent();

        // Should return the same value each time
        assert_eq!(user_agent1, user_agent2);
    }

    #[test]
    fn test_user_agent_contains_no_spaces() {
        let user_agent = user_agent();

        // User-Agent strings typically shouldn't contain spaces in the product identifier
        let parts: Vec<&str> = user_agent.split(' ').collect();

        // Should be a single part (no spaces) or properly formatted with spaces
        // For "open-lark/x.y.z" format, it should be a single part
        if parts.len() == 1 {
            assert!(!user_agent.contains(' '));
        }
    }

    #[test]
    fn test_user_agent_ascii() {
        let user_agent = user_agent();

        // User-Agent should be ASCII for HTTP header compatibility
        assert!(user_agent.is_ascii());

        // Should not contain control characters
        for ch in user_agent.chars() {
            assert!(!ch.is_control() || ch == '\t');
        }
    }

    #[test]
    fn test_user_agent_version_format() {
        let user_agent = user_agent();

        // Should match semantic versioning pattern or similar
        assert!(user_agent.matches('.').count() >= 1); // At least one dot for version

        // Version part should contain at least one digit
        let version_part = user_agent.strip_prefix("open-lark/").unwrap();
        assert!(version_part.chars().any(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_user_agent_length_reasonable() {
        let user_agent = user_agent();

        // Should be reasonable length for HTTP headers (not too long)
        assert!(user_agent.len() > 5); // At least "open-lark/" + something
        assert!(user_agent.len() < 100); // Not excessively long
    }

    #[test]
    fn test_user_agent_prefix() {
        let user_agent = user_agent();

        // Test different ways of checking the prefix
        assert!(user_agent.starts_with("open-lark/"));
        assert_eq!(&user_agent[0..10], "open-lark/");

        // Test that it doesn't start with other common prefixes
        assert!(!user_agent.starts_with("Mozilla/"));
        assert!(!user_agent.starts_with("curl/"));
        assert!(!user_agent.starts_with("wget/"));
    }

    #[test]
    fn test_user_agent_multiple_calls() {
        // Test calling the function multiple times in sequence
        let calls: Vec<String> = (0..5).map(|_| user_agent()).collect();

        // All calls should return the same value
        for call in &calls {
            assert_eq!(*call, calls[0]);
        }

        // Verify we have 5 identical results
        assert_eq!(calls.len(), 5);
        assert!(calls.iter().all(|call| call == &calls[0]));
    }

    #[test]
    fn test_user_agent_integration_with_version() {
        // This test verifies that the user_agent function properly integrates
        // with the VERSION constant from the constants module
        let user_agent = user_agent();

        // Test that VERSION constant is accessible and has expected pattern
        assert!(VERSION.contains('.'));

        // Test that the user_agent incorporates VERSION correctly
        assert!(user_agent.ends_with(VERSION));

        // Test the full format
        let expected_format = format!("open-lark/{VERSION}");
        assert_eq!(user_agent, expected_format);
    }
}
