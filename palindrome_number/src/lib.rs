#[must_use]
pub fn is_palindrome(x: i32) -> bool {

    if x.to_string().as_str().chars().rev().collect::<String>() == x.to_string().as_str() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_1() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn is_palindrome_2() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn is_palindrome_3() {
        assert!(!is_palindrome(10));
    }
}
