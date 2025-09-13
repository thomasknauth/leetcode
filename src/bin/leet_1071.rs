// For two strings s and t, we say "t divides s" if and only if s = t + t + t + ... + t + t (i.e., t is concatenated with itself one or more times).

// Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

struct Solution {}

fn strdiv(s: &str, div: &str) -> bool {

    if div.is_empty() {
        return false;
    }

    if s.len() % div.len() != 0 {
        return false;
    }

    // Those cases should also be covered by the above condition.
    assert!(s.len() >= div.len());

    for i in 0..s.as_bytes().len() {
        if s.as_bytes()[i] != div.as_bytes()[i%div.as_bytes().len()] {
            return false;
        }
    }
    return true;
}

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut result = "".to_string();
        for len in 1..str1.as_bytes().len()+1 {
            if str1.as_bytes().len() % len != 0 ||
               str2.as_bytes().len() % len != 0 {
                continue;
            }

            let div = str::from_utf8(&str1.as_bytes()[0..len]).unwrap();
            if strdiv(&str1, div) && strdiv(&str2, div) {
                result = String::from(div);
            }
        }
        result
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_cases =[
            ("ABCABC".to_string(), "ABC".to_string(), "ABC".to_string()),
            ("ABABAB".to_string(), "ABAB".to_string(), "AB".to_string()),
            ("LEET".to_string(), "CODE".to_string(), "".to_string()),
            ("ABCD".repeat(100), "ABCD".to_string(), "ABCD".to_string()),
            ("A".repeat(1000), "A".repeat(1000), "A".repeat(1000))
        ];

        for (str1, str2, expected) in test_cases {
            let result = Solution::gcd_of_strings(str1, str2);
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_strdiv() {
        assert_eq!(strdiv("", ""), false);
        assert_eq!(strdiv("a", ""), false);
        assert_eq!(strdiv("a", "a"), true);
        assert_eq!(strdiv("aa", "a"), true);
        assert_eq!(strdiv("aa", "b"), false);
        assert_eq!(strdiv("aba", "ab"), false);
        assert_eq!(strdiv("abcabc", "abc"), true);
    }
}
