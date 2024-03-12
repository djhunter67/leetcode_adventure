use std::collections::HashMap;

#[must_use]
pub fn roman_to_int(s: String) -> i32 {
    let roman: HashMap<&str, i32> = HashMap::new();

    roman.insert("I", 1);
    roman.insert("V", 5);
    roman.insert("X", 10);
    roman.insert("L", 50);
    roman.insert("C", 100);
    roman.insert("D", 500);
    roman.insert("M", 1000);

    let mut num: Vec<i32>;
    for letter in s.chars() {
	 num.push(*roman.get(&letter.into()).unwrap());
    }

    num.iter_mut().map(|x| String::from(*x)).collect::<Vec<String>>();
	0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int1() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn roman_to_int2() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn roman_to_int3() {
        assert_eq!(roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn roman_to_int4() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn roman_to_int5() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
