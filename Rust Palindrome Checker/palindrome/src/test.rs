#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("racecar"), true);
        assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
        assert_eq!(is_palindrome("Was it a car or a cat I saw?"), true);
        assert_eq!(is_palindrome("not a palindrome"), false);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(is_palindrome(""), true);
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(is_palindrome("   "), true);
        assert_eq!(is_palindrome("  a  "), true);
    }

    #[test]
    fn test_punctuation() {
        assert_eq!(is_palindrome("Madam, in Eden I'm Adam"), true);
        assert_eq!(is_palindrome("A Santa, at NASA"), true);
        assert_eq!(is_palindrome("No 'x' in Nixon"), true);
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(is_palindrome("🚀🛸🛰️"), true);
        assert_eq!(is_palindrome("😀😃😄😃😀"), true);
        assert_eq!(is_palindrome("hello world"), false);
    }
}
