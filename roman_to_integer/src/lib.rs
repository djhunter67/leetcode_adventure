use std::collections::BTreeMap;

#[must_use]
pub fn roman_to_int(s: String) -> i32 {
    let mut roman: BTreeMap<&str, i32> = BTreeMap::new();

    roman.insert("I", 1);
    roman.insert("IV", 4);
    roman.insert("V", 5);
    roman.insert("IX", 9);
    roman.insert("X", 10);
    roman.insert("XL", 40);
    roman.insert("L", 50);
    roman.insert("XC", 90);
    roman.insert("C", 100);
    roman.insert("CD", 400);
    roman.insert("D", 500);
    roman.insert("CM", 900);
    roman.insert("M", 1000);

    let mut num_sum: i32 = 0;
    let mut running_vals: Vec<char> = vec![];
    for (i, letter) in s.char_indices() {
        let the_letter = letter.to_string();

        running_vals.push(letter);

        // if i.eq(&0) {
        // num_sum += roman.get(the_letter.as_str()).unwrap_or(&0);
        // continue;
        // }

        match letter {
            'V' => {
                let prev_val: char = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'I') {
                    num_sum += roman.get("IV").unwrap_or(&0);
                }
            }
            'X' => {
                let prev_val = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'I') {
                    num_sum += roman.get("IX").unwrap_or(&0);
                }
            }
            'L' => {
                let prev_val = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'X') {
                    num_sum += roman.get("XL").unwrap_or(&0);
                }
            }
            'C' => {
                let prev_val = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'X') {
                    num_sum += roman.get("XC").unwrap_or(&0);
                }
            }
            'D' => {
                let prev_val = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'C') {
                    num_sum += roman.get("CD").unwrap_or(&0);
                }
            }
            'M' => {
                let prev_val = match running_vals.get(i - 1) {
                    Some(val) => *val,
                    None => continue,
                };
                if prev_val.eq(&'C') {
                    num_sum += roman.get("CM").unwrap_or(&0);
                }
            }
            _ => num_sum += roman.get(the_letter.as_str()).unwrap_or(&0),
        }
    }

    // num.iter_mut().map(|x| *x).collect::<Vec<i32>>();
    num_sum
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

    #[test]
    fn roman_to_int_simple() {
        assert_eq!(roman_to_int("I".to_string()), 1);
        assert_eq!(roman_to_int("V".to_string()), 5);
        assert_eq!(roman_to_int("X".to_string()), 10);
        assert_eq!(roman_to_int("L".to_string()), 50);
        assert_eq!(roman_to_int("C".to_string()), 100);
        assert_eq!(roman_to_int("D".to_string()), 500);
        assert_eq!(roman_to_int("M".to_string()), 1000);
    }

    #[test]
    fn roman_to_int_subtractive() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int("XL".to_string()), 40);
        assert_eq!(roman_to_int("XC".to_string()), 90);
        assert_eq!(roman_to_int("CD".to_string()), 400);
        assert_eq!(roman_to_int("CM".to_string()), 900);
    }

    #[test]
    fn roman_to_int_repeated() {
        assert_eq!(roman_to_int("II".to_string()), 2);
        assert_eq!(roman_to_int("XX".to_string()), 20);
        assert_eq!(roman_to_int("CCC".to_string()), 300);
    }

    #[test]
    fn roman_to_int_mixed() {
        assert_eq!(roman_to_int("XII".to_string()), 12);
        assert_eq!(roman_to_int("LXVII".to_string()), 67);
        assert_eq!(roman_to_int("CCXLV".to_string()), 245);
        assert_eq!(roman_to_int("DCCCXLV".to_string()), 845);
    }

    #[test]
    fn roman_to_int_edge() {
        assert_eq!(roman_to_int("MMMCMXCIX".to_string()), 3999);
    }
}
