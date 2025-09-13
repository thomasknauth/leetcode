// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

// Return the merged string.

struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let bs1 = word1.as_bytes();
        let bs2 = word2.as_bytes();
        let mut r = Vec::<u8>::new();
        for i in 0..bs1.len() {
            r.push(bs1[i]);
            if i < bs2.len() {
                r.push(bs2[i]);
            }
        }
        if bs2.len() > bs1.len() {
            //r.append(&bs2[bs1.len()..]);
            r.extend_from_slice(&bs2[bs1.len()..]);
        }
        return String::from_utf8(r).unwrap();
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let r = Solution::merge_alternately(String::from("abc"), String::from("pqr"));
        assert_eq!(r, "apbqcr");
        let r = Solution::merge_alternately(String::from("ab"), String::from("pqrs"));
        assert_eq!(r, "apbqrs");
        let r = Solution::merge_alternately(String::from("abcd"), String::from("pq"));
        assert_eq!(r, "apbqcd");
        let r = Solution::merge_alternately(String::from("abcd"), String::from(""));
        assert_eq!(r, "abcd");
        let r = Solution::merge_alternately(String::from(""), String::from("abcd"));
        assert_eq!(r, "abcd");
        let r = Solution::merge_alternately(String::from(""), String::from(""));
        assert_eq!(r, "");
    }
}
