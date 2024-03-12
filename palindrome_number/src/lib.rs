#[must_use]
pub fn is_palindrome(x: i32) -> bool {

    if x.le(&0) {
	return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
