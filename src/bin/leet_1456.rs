// 1456

// Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.

// Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

struct Solution {}

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {

        if s.is_empty() {
            return 0;
        }

        let k: usize = usize::try_from(k).unwrap();
        let vowels = vec![b'a', b'e', b'i', b'o', b'u'];
        // use sliding window method
        // k is "window-length"
        // count vowels for a s[0..k-1]
        // shift right by 1, and +/- 1 vowel count based on s[i-1] and s[i+k]
        let b = s.as_bytes();
        let mut vowel_count = b[0..k as usize].iter().filter(|&c| vowels.contains(c)).count();
        let mut max_vowel_count = vowel_count;

        // complexity: O(n) - need to go over entire input once
        for i in k..b.len() {
            println!("i={:?}, c={:?}", i, vowel_count);
            if vowels.contains(&b[i-k]) {
                vowel_count -= 1;
            }
            if vowels.contains(&b[i]) {
                vowel_count += 1;
            }
            max_vowel_count = vowel_count.max(max_vowel_count);
        }
        i32::try_from(max_vowel_count).unwrap()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        let testcases = vec![
            ("abciiidef".to_string(), 3, 3),
            ("aeiou".to_string(), 2, 2),
            ("leetcode".to_string(), 3, 2),
            ("".to_string(), 0, 0),
            ("abc".to_string(), 0, 0),
            ("abc".to_string(), 1, 1),
            ("bcd".to_string(), 1, 0),
            ("abcdefghihklmnopqrstuvwxyz".to_string(), 26, 5),
            ("weallloveyou".to_string(), 7, 4),
        ];
        for (s, k, result) in testcases {
            assert_eq!(Solution::max_vowels(s, k), result);
        }
    }
}
